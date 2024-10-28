#!/bin/bash

set -e

export CONTAINER_HOST="ssh://root@137.184.81.246:22/run/podman/podman.sock"

podman compose --env-file ./.env -f ./docker/stack.yml up --detach