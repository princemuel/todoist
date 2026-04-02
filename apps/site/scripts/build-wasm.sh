#!/usr/bin/env bash
set -Eeuo pipefail

trap 'echo "❌ build-wasm failed on line $LINENO"' ERR

build() {
  wasm-pack build --target web
}

optimise() {
  wasm-opt -Oz --enable-bulk-memory pkg/interface_bg.wasm -o pkg/interface_bg.wasm
}

copy_artifacts() {
  cp pkg/interface_bg.wasm ../dist/interface_bg.wasm
}

cd interface
build
optimise
copy_artifacts
echo "✅ wasm build done"
