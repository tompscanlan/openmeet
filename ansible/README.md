# ansible for deploying cassandra

## Get a seed node up

```
ansible-playbook -i inventories/dev/hosts raise_droplet.yaml
ansible-playbook -i inventories/dev/hosts install_cassandra.yaml
ansible-playbook -i inventories/dev/hosts install_cassandra_certs.yaml
```