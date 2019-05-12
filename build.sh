#!/bin/bash

# If new system, please install the following:
# rustup target add x86_64-unknown-linux-musl
# brew install filosottile/musl-cross/musl-cross
#
# From https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

# Set script dir to CWD
cd "${0%/*}"

# Build rust binary
cargo build --release --target x86_64-unknown-linux-musl

# Zip to out/ directory
if [ ! -d out ]; then
  mkdir out
fi

zip -j out/rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

