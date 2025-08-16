#!/usr/bin/env bash

set -e
set -x

cargo clippy --all-features
cargo test --all-features

exit 0
