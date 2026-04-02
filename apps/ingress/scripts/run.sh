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

pnpm build

cd ../ingress || exit

# cargo clean
export "$(cat .env | xargs)"

cargo run
