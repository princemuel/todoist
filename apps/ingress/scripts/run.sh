#!/usr/bin/env bash
# File: ingress/scripts/run_server.sh
# navigate to directory
SCRIPTPATH="$(
  cd "$(dirname "$0")" || exit
  pwd -P
)"

cd "$SCRIPTPATH" || exit
cd ../..

cd site || exit

npm install

npm run build

cd ../ingress || exit

cargo clean
cargo run
