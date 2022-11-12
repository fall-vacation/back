#!/bin/bash

#VERSION=$(git rev-parse --short head)
docker build --platform=linux/arm64 \
  -t redheadset/fall-vacation-back:arm64_latest --push .
