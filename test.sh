#!/bin/sh
set -e
docker build -t rust-gssapi-test . 
docker run -v /tmp/cargo-registry:/root/.cargo/registry -v /tmp/rust-gssapi-build:/rust-gssapi/build -it rust-gssapi-test