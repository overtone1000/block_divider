#!/bin/bash

set -e

source ./server.sh

ssh -l $REMOTE_USER $REMOTE_IP "podman pull docker.io/overtone1000/block_divider:prod"
ssh -l $REMOTE_USER $REMOTE_IP "podman restart stack_core_1"