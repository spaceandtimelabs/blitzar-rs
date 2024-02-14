#!/bin/bash

set -e

IMAGE=spaceandtimelabs/blitzar:12.3.1-cuda-1.76.0-rust-0

# If you have a GPU instance configured in your machine
docker run -v "$PWD":/src -w /src --gpus all --privileged -it "$IMAGE"
