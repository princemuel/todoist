#!/usr/bin/env bash
set -Eeuo pipefail

if [ $# -eq 0 ]; then
  echo "Usage: $0 <migration-name>"
  exit 1
fi

SCRIPTPATH="$(
  cd "$(dirname "$0")"
  pwd -P
)"

cd "$SCRIPTPATH"/../dal/migrations || exit

now=$(date +'%Y%m%d%H%M%S')
migration_name="$1"

touch "${now}_${migration_name}.sql"
