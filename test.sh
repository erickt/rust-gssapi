#!/bin/sh

# Run this script from your laptop (where the Docker client lives) to run the test
# suite.

set -e
docker build -t rust-gssapi-test . 
docker run \
  -v /tmp/cargo-registry:/root/.cargo/registry \
  -v /tmp/rust-gssapi-build:/rust-gssapi/build \
  -v /tmp/rust-gssapi-target:/rust-gssapi/target \
  -it rust-gssapi-test