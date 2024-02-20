#!/bin/bash

set -e

IMAGE=rust:1.76

# If you don't have a GPU instance configured in your machine
docker run -v "$PWD":/src -w /src --privileged -it "$IMAGE"
