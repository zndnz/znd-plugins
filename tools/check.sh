#!/bin/sh
# ビルドして、すべてのパッケージを znd に読み込ませて検証する。
set -e
cd "$(dirname "$0")/.."
tools/build.sh debug
cargo test --workspace "$@"
