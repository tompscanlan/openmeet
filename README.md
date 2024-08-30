# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).


# installing couchdb

```
sudo apt update && sudo apt upgrade -y
sudo apt install -y curl apt-transport-https gnupg
curl https://couchdb.apache.org/repo/keys.asc | gpg --dearmor | sudo tee /usr/share/keyrings/couchdb-archive-keyring.gpg >/dev/null 2>&1
echo "deb [signed-by=/usr/share/keyrings/couchdb-archive-keyring.gpg] https://apache.jfrog.io/artifactory/couchdb-deb/ $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/couchdb.list >/dev/null

sudo apt update
sudo apt install -y couchdb

sudo apt install -y nginx

sudo apt install -y certbot python3-certbot-nginx


```

Configure Nginx as a reverse proxy for CouchDB. Create a new file /etc/nginx/sites-available/couchdb:

```
server {
    listen 80;
    server_name your_domain.com;

    location / {
        proxy_pass http://localhost:5984;
        proxy_redirect off;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```


```
sudo ln -s /etc/nginx/sites-available/couchdb /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
sudo certbot --nginx -d your_domain.com

```

Update CouchDB configuration to bind to localhost only. Edit /opt/couchdb/etc/local.ini:

```
[chttpd]
bind_address = 127.0.0.1
```

```
sudo systemctl restart couchdb
sudo certbot renew --dry-run
sudo systemctl enable certbot.timer
sudo systemctl start certbot.timer
```

To customize the renewal process, you can create a renewal hook. This is useful for reloading Nginx after renewal. Create a file /etc/letsencrypt/renewal-hooks/deploy/01-reload-nginx:

```
#!/bin/bash
nginx -t && systemctl reload nginx
```
```
sudo chmod +x /etc/letsencrypt/renewal-hooks/deploy/01-reload-nginx
```

# installing cassandra

Edit cassandra.yaml to suit your needs, then use ansible to deploy a server
```
cd ansible
 ansible-playbook -i inventory/  playbooks/main.yaml 
```

### Add server to DNS

Create certs for the server using certbot and route53. Edit cassandra.yaml with the DNS name of the server.

```
sudo certbot certonly --dns-route53 -d cassandra1.int.butterhead.net
```

### set up cassandra to support SSL

server_encryption_options
1. set internode_encryption to all, set optional to true
2. set require_client_auth to true
3. restart cassandra
4. generate server keystores and truststores
5. set optional to false


client_encryption_options
1.   enabled: true
2.   optional: true
3. create keystore
4. create truststore
5. updata cassandra.yaml with the location of the keystore and truststore. A keystore contains private keys. The truststore contains SSL certificates for each node.
6. restart cassandra

# https://docs.datastax.com/en/cassandra-oss/3.x/cassandra/configuration/secureSSLCertificates.html

### Client to node communication

keytool -genkey -keyalg RSA -alias cassandra1.int.butterhead.net  -validity 36500 -keystore keystore.cassandra1.int.butterhead.net
 keytool -genkey -keyalg RSA -alias cassandra1.int.butterhead.net -keystore keystore.cassandra1.int.butterhead.net -storepass XXXXXXXXX  -keypass XXXXXXXXX -dname "CN=192.168.8.102, OU=None, O=None, L=None, C=None"
 keytool -export -alias cassandra1.int.butterhead.net  -file cassandra1.cer -keystore keystore.cassandra1.int.butterhead.net
 keytool -import -v -trustcacerts -alias cassandra1.int.butterhead.net -file cassandra1.cer  -keystore truststore.cassandra1.int.butterhead.net

keytool -importkeystore -srckeystore keystore.cassandra1.int.butterhead.net -destkeystore cassandra1.p12 -deststoretype PKCS12 -srcstorepass XXXXXXXXX -deststorepass XXXXXXXXX
openssl pkcs12 -in cassandra1.p12 -nokeys -out cassandra1.cer.pem -passin pass:XXXXXXXXX
openssl pkcs12 -in cassandra1.p12 -nodes -nocerts -out cassandra1.key.pem -passin pass:XXXXXXXXX

### Production certificates
https://docs.datastax.com/en/cassandra-oss/3.x/cassandra/configuration/secureSSLCertWithCA.html
Use letsencrypt to create a certificate for the domain.
sudo certbot certonly --dns-route53 -d cassandra1.int.butterhead.net

# Convert Let's Encrypt certificates to PKCS12 format
sudo openssl pkcs12 -export -in /etc/letsencrypt/live/cassandra1.int.butterhead.net/fullchain.pem -inkey /etc/letsencrypt/live/cassandra1.int.butterhead.net/privkey.pem -out cassandra1.int.butterhead.net.p12 -name cassandra1.int.butterhead.net -passout pass:XXXXXXXXX

# Convert PKCS12 to JKS format
keytool -importkeystore -srckeystore cassandra1.int.butterhead.net.p12 -srcstoretype PKCS12 -destkeystore keystore.cassandra1.int.butterhead.net -deststoretype JKS -srcstorepass XXXXXXXXX -deststorepass XXXXXXXXX

pushd /tmp
# Download Let's Encrypt root certificate
wget https://letsencrypt.org/certs/isrgrootx1.pem
popd

# Import the root certificate into a new truststore
keytool -import -trustcacerts -alias root -file /tmp/isrgrootx1.pem -keystore truststore.cassandra1.int.butterhead.net -storepass XXXXXXXXX

sudo nano /etc/letsencrypt/renewal-hooks/post/update-cassandra-keystore.sh

```
#!/bin/bash

# Set variables
DOMAIN="cassandra1.int.butterhead.net"
KEYSTORE_PATH="/etc/cassandra/keys/keystore.$DOMAIN"
KEYSTORE_PASSWORD="XXXXXXXXX"

# Convert the renewed certificate to PKCS12 format
openssl pkcs12 -export \
    -in /etc/letsencrypt/live/$DOMAIN/fullchain.pem \
    -inkey /etc/letsencrypt/live/$DOMAIN/privkey.pem \
    -out /tmp/$DOMAIN.p12 \
    -name $DOMAIN \
    -passout pass:$KEYSTORE_PASSWORD

# Update the keystore with the new certificate
keytool -importkeystore \
    -srckeystore /tmp/$DOMAIN.p12 \
    -srcstoretype PKCS12 \
    -destkeystore $KEYSTORE_PATH \
    -deststoretype JKS \
    -srcstorepass $KEYSTORE_PASSWORD \
    -deststorepass $KEYSTORE_PASSWORD \
    -noprompt

# Clean up the temporary PKCS12 file
rm /tmp/$DOMAIN.p12

# Restart Cassandra to use the new certificate
systemctl restart cassandra
```
sudo chmod +x /etc/letsencrypt/renewal-hooks/post/update-cassandra-keystore.sh

