#!/bin/bash

set -e

export CONTAINER_HOST="unix:///run/podman/podman.sock"

TAG=$1

if [ "$#" -ne 1 ]; then
    echo "Supply exactly one argument, which should be the SEMVAR tag in the format 0.0.0"
    exit
else
    echo "Pushing version $TAG"
    echo "Working directory: $(pwd)"
fi

# Docker build and push
REGISTRY=docker.io
UNAME=overtone1000
IMAGE_NAME=block_divider

FULL_PREFACE=$REGISTRY/$UNAME/$IMAGE_NAME

echo Building
sudo podman build -t $FULL_PREFACE:"$TAG" -t $FULL_PREFACE:prod -f ./docker/Dockerfile ../.. #Repository root

echo Pushing image to repository
sudo podman push $UNAME/$IMAGE_NAME:"$TAG"
sudo podman push $UNAME/$IMAGE_NAME:prod

echo Committing git
# Git tag
if [ -n "$(git status --porcelain)" ]; then
    git commit -a --signoff -m "Version $TAG"
    git tag -af v"$TAG" -m "Version $TAG"

    CURRENT_BRANCH=$(git branch --show-current)
    git checkout main
    git merge "$CURRENT_BRANCH"
    git push -f origin v"$TAG"
    git checkout "$CURRENT_BRANCH"
else
    echo "No git changes. Skipping commit."
fi

exit 0