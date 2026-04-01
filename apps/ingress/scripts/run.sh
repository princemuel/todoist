#!/usr/bin/env bash

set -euoE pipefail

SCRIPTPATH="$(
  cd "$(dirname "$0")" || exit
  pwd -P
)"

cd "$SCRIPTPATH" || exit
cd ../..

cd site || exit

pnpm install

pnpm run build

cd ../ingress || exit

# cargo clean
cargo run
