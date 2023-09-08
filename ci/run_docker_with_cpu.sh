#!/bin/bash

set -e

# To build this image from source, see `sxt-utility-images` repo
IMAGE=spaceandtimelabs/blitzar:12.2.0-cuda-1.71.1-rust-0

# If you have a GPU instance configured in your machine
docker run -v "$PWD":/src -w /src --privileged -it "$IMAGE"
