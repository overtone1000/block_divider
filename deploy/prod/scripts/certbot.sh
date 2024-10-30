#!/bin/bash

set -e

source ./server.sh

CONTAINER_ID=$(podman container ls -q -f name=nginx)
echo Executing certbot
podman exec "$CONTAINER_ID" certbot \
    --nginx \
    --preferred-challenges "http" \
    --redirect \
    --noninteractive \
    --expand \
    --keep-until-expiring \
    --agree-tos \
    --email "TylerRMoore@gmail.com" \
    -d "rotations.autoscheda.com" \
    -d "schedule.autoscheda.com" \
    -d "pa-schedule.autoscheda.com" \
    -d "block-division.autoscheda.com"
    #-d "logs.autoscheda.com" \
    #-d "loki.autoscheda.com" \

reload_nginx