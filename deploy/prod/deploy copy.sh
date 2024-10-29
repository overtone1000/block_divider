#!/bin/bash

set -e

REMOTE_IP="137.184.81.246"
REMOTE_USER="root"

STACK="./docker/stack.yml"
ENV_F="./.env"

ssh -l $REMOTE_USER $REMOTE_IP "mkdir -p /tmp/stack"
scp -r $ENV_F $REMOTE_USER@$REMOTE_IP:/tmp/stack/.env
scp -r $STACK $REMOTE_USER@$REMOTE_IP:/tmp/stack/stack.yml
ssh -l $REMOTE_USER $REMOTE_IP "podman compose --env-file ./.env -f ./docker/stack.yml down"
#ssh -l $REMOTE_USER $REMOTE_IP "podman container prune -f"
#ssh -l $REMOTE_USER $REMOTE_IP "podman compose --env-file ./.env -f ./docker/stack.yml up --detach"
ssh -l $REMOTE_USER $REMOTE_IP "rm -rdf /tmp/stack"