#!/bin/bash

#VERSION=$(git rev-parse --short head)
docker build --platform=linux/amd64 \
  -t redheadset/fall-vacation-back:$VERSION \
  -t redheadset/fall-vacation-back:amd64_latest --push .
