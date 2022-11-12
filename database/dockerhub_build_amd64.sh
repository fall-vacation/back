#!/bin/bash

docker build --platform=linux/amd64 \
  -t redheadset/fall-vacation-db:amd64_latest --push .
