#!/usr/bin/env bash
set -Eeuo pipefail

trap 'echo "❌ build-wasm failed on line $LINENO"' ERR

build() {
  wasm-pack build --target web
}

optimise() {
  wasm-opt -Oz --enable-bulk-memory pkg/components_bg.wasm -o pkg/components_bg.wasm
}

copy_artifacts() {
  cp pkg/components_bg.wasm ../ingress/site/dist/components_bg.wasm
}

cd ../../../components
build
optimise
copy_artifacts
echo "✅ wasm build done"
