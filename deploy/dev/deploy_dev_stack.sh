#!/bin/bash

set -e

#docker context use default

#docker stack deploy -c ./stack.yml block_division_dev

docker compose -f stack.yml up -d