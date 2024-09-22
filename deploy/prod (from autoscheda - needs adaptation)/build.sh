#!/bin/bash

set -e

TAG=$1

if [ "$#" -ne 1 ]; then
    echo "Supply exactly one argument, which should be the SEMVAR tag in the format 0.0.0"
    exit
else
    echo "Pushing version $TAG"
    echo "Working directory: $(pwd)"
fi

# Change package.json
json -I -f ../../src/frontend/package.json -e "this.version=\"$TAG\""

# Build frontend and docs
#rm -f ../../src/backend/src/main/resources/static/* #this breaks caching. Can just trust build to handle it?
npm run --prefix ../../src/frontend build_prod

# Docker build and push
REGISTRY=docker.io
UNAME=overtone1000
IMAGE_NAME=autoscheda

FULL_PREFACE=$REGISTRY/$UNAME/$IMAGE_NAME

docker build -t $FULL_PREFACE:$TAG -t $FULL_PREFACE:prod -f ./docker/Dockerfile ../../src/backend/
#docker login -u "$UNAME" $REGISTRY #Can't do this noninteractively. Just log in before deploy.
docker push $UNAME/$IMAGE_NAME:$TAG
docker push $UNAME/$IMAGE_NAME:prod

# Git tag
if [ -n "$(git status --porcelain)" ]; then
    git commit -a --signoff -m "Version $TAG"
    git tag -af v$TAG -m "Version $TAG"

    CURRENT_BRANCH=$(git branch --show-current)
    git checkout prod
    git merge $CURRENT_BRANCH
    git push -f origin v$TAG
    git checkout $CURRENT_BRANCH
else
    echo "No git changes. Skipping commit."
fi

exit 0