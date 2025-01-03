#!/bin/bash

set -e

source ./server.sh

STACK="../block_divider/docker/stack.yml"
ENV_F="../.env"

TMP_FOLDER=/tmp/stack

ssh -l $REMOTE_USER $REMOTE_IP "mkdir -p $TMP_FOLDER"
scp -r $ENV_F $REMOTE_USER@$REMOTE_IP:$TMP_FOLDER/.env
scp -r $STACK $REMOTE_USER@$REMOTE_IP:$TMP_FOLDER/stack.yml
ssh -l $REMOTE_USER $REMOTE_IP "podman pull docker.io/overtone1000/block_divider:prod"
ssh -l $REMOTE_USER $REMOTE_IP "podman compose --env-file $TMP_FOLDER/.env -f $TMP_FOLDER/stack.yml down"
ssh -l $REMOTE_USER $REMOTE_IP "podman container prune -f"
ssh -l $REMOTE_USER $REMOTE_IP "podman compose --env-file $TMP_FOLDER/.env -f $TMP_FOLDER/stack.yml up --detach"
ssh -l $REMOTE_USER $REMOTE_IP "rm -rdf $TMP_FOLDER"

echo "Restarting nginx to repair network."
ssh -l $REMOTE_USER $REMOTE_IP "podman restart stack_nginx_1"