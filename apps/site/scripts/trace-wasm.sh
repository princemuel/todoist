#!/usr/bin/env bash
set -Eeuo pipefail

trap 'echo "❌ trace-wasm failed on line $LINENO"' ERR

trace() {
  wasm2wat interface_bg.wasm >interface_bg.wat
}

cd dist
trace
echo "✅ trace done → dist/interface_bg.wat"
