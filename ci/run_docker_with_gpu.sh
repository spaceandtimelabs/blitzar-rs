#!/bin/bash

set -e

IMAGE=rust:1.84

# If you have a GPU instance configured in your machine
docker run -v "$PWD":/src -w /src --gpus all --privileged -it "$IMAGE"
