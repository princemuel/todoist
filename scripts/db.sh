#!/usr/bin/env bash
set -Eeuo pipefail

SCRIPTPATH="$(
  cd "$(dirname "$0")" || exit
  pwd -P
)"

cd "$SCRIPTPATH" || exit
cd ..

docker compose up -d

until pg_isready -h localhost -p 5432 -U kalel; do
  echo "Waiting for postgres"
  sleep 2
done

echo "docker is now running"

docker compose down
