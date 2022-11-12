#!/bin/bash

docker build --platform=linux/arm64 \
  -t redheadset/fall-vacation-db:latest --push .
