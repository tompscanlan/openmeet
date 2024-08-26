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