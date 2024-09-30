#!/bin/bash

set -e

IMAGE=rust:1.81

# If you don't have a GPU instance configured in your machine
docker run -v "$PWD":/src -w /src --privileged -it "$IMAGE"
