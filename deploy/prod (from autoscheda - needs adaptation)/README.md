# Deployment Configuration

## Local dependencies required
- Development environment dependencies (see that readme)
- `sudo npm install -g json` for editing json files from CLI

## Remote/Server Config
- Install Docker engine. https://docs.docker.com/engine/install/
- Put in swarm node with `docker swarm init -advertise-addr $NODE_IP`
- `systemd` needs to be configured to start docker daemon. Do this with `sudo systemctl enable docker`.
- The user needs to be configured into the docker group:
    ```
    sudo groupadd docker
    sudo usermod -aG docker $(whoami)
    sudo service docker restart
    ```
- The tutorial suggests automating sudo access for the remote user by editing the file at `/etc/sudoers` with the following line: `{username} ALL=(ALL) NOPASSWD:ALL` but have done without this. Try not to use sudo with docker.

## Swarm Multi-node Config
`core` and `manager` nodes
`MANGER_IP=The private IP for node`
On manager node: 
`docker swarm init --advertise-addr $PRIVATE_MANAGER_IP`
`docker swarm join-token worker` provides the token for joining

On core node, run the join command from above to join it to swarm. Could add advertise address (`docker swarm join --token $TOKEN_FROM_ABOVE --advertise-addr $PRIVATE_CORE_IP $PRIVATE_MANAGER_IP:2377`), but this isn't necessary if the private IP was used above! Can just copy. Check by inspecting the node that gets created.

On manager node, run:
```
ufw reset && \
ufw allow 22/tcp && \
ufw allow 80/tcp && \
ufw allow 443/tcp && \
ufw allow 2377/tcp && \
ufw allow 7946/tcp && \
ufw allow 7946/udp && \
ufw allow 4789/udp && \
ufw reload && \
ufw enable && \
systemctl restart docker
```

On core node, run
```
ufw reset && \
ufw allow 22/tcp && \
ufw allow 2377/tcp && \
ufw allow 7946/tcp && \
ufw allow 7946/udp && \
ufw allow 4789/tcp && \
ufw allow 4789/udp && \
ufw reload && \
ufw enable && \
systemctl restart docker
```

Docker docs discuss encryption on overlay networks. However, if the private IPs are used above for the nodes (see Digital Ocean VPC), an encrypted network is not necessary.

On manager node:
`docker node ls` to list nodes.
`docker node update --label-add name=manager MANAGER_NODE_ID` to give the "manager" name to the manager node
`docker node update --label-add name=core CORE_NODE_ID` to give the "core" name to the core node
`docker network create -d overlay autoscheda_internal` (previously used `--opt encrypted` but this caused firewall problems. Better to use an isolated network for pertinent nodes.)

On core node:
`docker login`

On every node:
`docker plugin install grafana/loki-docker-driver:latest --alias loki --grant-all-permissions`

Use VPS to block access to ports 3100 and 9096.
- Before, did this, but probably can't in true swarm mode:
    - Block external access to loki directly through IP tables rules
        - Get network interface name (usually `eth0`) with `ifconfig -s -a`
        - Add the following rules for resulting interface `NETWORK_INTERFACE` name to IP tables with the following command: 
            `iptables -I DOCKER-USER -i $NETWORK_INTERFACE -p tcp --destination-port 3100 -j REJECT`
            `iptables -I DOCKER-USER -i $NETWORK_INTERFACE -p tcp --destination-port 9096 -j REJECT`
        - Probably good to block port 3100 via the VPS provider as well

Things coming along, but still a couple of bugs:
- [x] Containers didn't end up where intended. Constraints were ignored.
    - [x] Fixed by modifying yml (too many spaces), but now grafana and loki won't start.
    - [x] One node wasn't labeled. Fixed.
Yes, needed to login, but how does this affect updates?? Shouldn't have to do this!
- [ ] Supposed to use encrypted networks but can't be done in the compose file?

## nginx stack
If loki config and credentials hasn't been created on this system...

If not yet done, generate the loki_credentials file with `htpasswd -n autoscheda` and inputting password 2dSgzwic94b9c (available with `sudo apt-get install apache2-utils`)

`docker config create autoscheda_loki ./docker/grafana_configs/loki.yaml`
`docker config create loki_credentials ./docker/loki_credentials`

```
docker pull overtone1000/nginx_certbot:alpine && \
docker pull overtone1000/autoscheda:prod && \
env $(cat .env) docker stack deploy -c ./docker/stack.yml autoscheda && \
docker service update autoscheda_nginx --force && \
watch docker ps
```

## nginx configuration
To upload a change in the nginx configuration...
```
CONTAINER_NAME=id_of_running_nginx_container
CONFIG_DIR=source_directory_containing_configs #usually "./nginx_config"

docker exec $CONTAINER_NAME bash -c "rm -rfv /etc/nginx/sites-available/*" &&  docker cp "${CONFIG_DIR}/." $CONTAINER_NAME:/etc/nginx/ &&  docker exec $CONTAINER_NAME nginx -t && docker exec $CONTAINER_NAME nginx -s reload
```

## certbot initialization
Get into the running nginx container and:
```
certbot --nginx --preferred-challenges "http" --redirect --noninteractive --expand --keep-until-expiring --agree-tos --email "TylerRMoore@gmail.com" -d "logs.autoscheda.com" -d "loki.autoscheda.com" -d "rotations.autoscheda.com" -d "schedule.autoscheda.com" -d "pa-schedule.autoscheda.com"
```

Took out `-d "autoscheda.com" -d "dev.autoscheda.com"` as no longer using.

## autorestic backup
Tried to mount the backup from inside the autorestic container with...
```
apk add fuse
autorestic exec -c autorestic.yml -b minio mount ./tmpmountpoint
```
...but, not luck. getting errors like "fuse device not found".

## autorestic restore
Stop postgres: `docker service scale autoscheda_postgres=0`
Get into restic container
List snapshots: `autorestic exec -v -c ./autorestic.yml -b minio snapshots`
Restore snapshot: `autorestic -c autorestic.yml restore -l remote -f $SNAPSHOT_ID`
Leave restic container
Start postgres: `docker service scale autoscheda_postgres=1`

## Deploying

0. **Strongly consider** running Selenium tests before deploying!
0. Update the changelog!
1. If there has been a major (breaks backward compatability) or minor (backward compatible, new API) change to the database, upgrade the version string `autoscheda_database_version` in the `src/backend/src/core/autoscheda/persistence/AutoschedaDatabase.java`. This will change the schema so old tables will not be referenced. __If preserving a database, update the `versionAndUpgradeCheck` function for that table.__
2. Iterate semvar version in pom.xml (change `<version>M.m.p</vserion>` in `src/backend/pom.xml`)
3. `docker login` to allow pushing the image to the registry.
4. If a server is running, need to close it for final tests to be run.
5. Maven clean (gets rid of old jars, need to run manually, deploy doesn't do it automatically for some reason).
6. Maven deploy (will run package for you).
7. Do another Maven clean to return dev env to normal conditions.

