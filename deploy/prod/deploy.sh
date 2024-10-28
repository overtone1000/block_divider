#!/bin/bash

set -e

source .env

podman compose -f ./docker/stack.yml up --detach