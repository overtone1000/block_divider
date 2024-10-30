#!/bin/bash

set -e

source ./server.sh

CONTAINER_ID=$(podman container ls -q -f name=nginx)

reload_nginx () {

    echo Testing nginx config
    podman exec "$CONTAINER_ID" nginx -t

    echo Reloading nginx
    podman exec "$CONTAINER_ID" nginx -s reload

}

echo Deleting existing sites available
podman exec "$CONTAINER_ID" rm -rf /etc/nginx/sites-available/*

echo Copying new sites available
podman cp ../nginx_config/. "$CONTAINER_ID":/etc/nginx/

reload_nginx