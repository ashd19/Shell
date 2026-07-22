#!/bin/sh
set -e
cd "$(dirname "$0")/.."
cargo build --release --target-dir=/tmp/codecrafters-build-shell-rust --manifest-path Cargo.toml
