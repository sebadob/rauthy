#!/bin/bash

TAG=20230429

#docker build -t rauthy-build:$TAG -f Dockerfile.builder --no-cache .
docker build -t rauthy-build:$TAG -f Dockerfile.rauthy-build .
docker push rauthy-build:$TAG
