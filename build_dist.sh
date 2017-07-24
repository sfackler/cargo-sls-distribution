#!/bin/bash
set -e

apt-get update
apt-get install -y --no-install-recommends cmake musl-tools

rustup target add x86_64-unknown-linux-musl
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc
cargo build --release --target x86_64-unknown-linux-musl
