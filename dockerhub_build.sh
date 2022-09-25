#!/bin/bash

VERSION=$(git rev-parse --short head)
docker build --platform=linux/arm64,linux/amd64 --output=type=docker \
  -t redheadset/fall-vacation-back:$VERSION \
  -t redheadset/fall-vacation-back:latest .