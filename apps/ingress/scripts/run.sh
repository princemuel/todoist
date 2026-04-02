#!/usr/bin/env bash
set -Eeuo pipefail

SCRIPT_PATH="$(
  cd "$(dirname "$0")" || exit
  pwd -P
)"

cd "$SCRIPT_PATH" || exit
cd ../..

cd site || exit

pnpm install

pnpm run build

cd ../ingress || exit

# cargo clean
cargo run
