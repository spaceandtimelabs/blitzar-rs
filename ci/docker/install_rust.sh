#!/bin/bash

set -e

export DEBIAN_FRONTEND="noninteractive"
export TZ=Etc/UTC

# Install rust
curl https://sh.rustup.rs -sSf | bash -s -- -y --profile minimal

echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

source "$HOME/.cargo/env"

# For latest version see https://github.com/rust-lang/rust/blob/master/RELEASES.md
rustup install 1.76.0

rustup component add rustfmt clippy

$HOME/.cargo/bin/cargo install --version 0.69.4 bindgen-cli
