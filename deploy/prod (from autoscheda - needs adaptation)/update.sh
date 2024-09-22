#!/bin/bash

set -e

docker context use autoscheda_production_core
docker pull overtone1000/autoscheda:prod

docker context use autoscheda_production_manager
docker service update autoscheda_backend --force --update-parallelism 1 --update-delay 30s
docker container prune -f
docker image prune -af

docker context use autoscheda_production_core
docker container prune -f
docker image prune -af

exit 0