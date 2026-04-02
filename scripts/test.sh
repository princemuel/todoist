#!/usr/bin/env bash
set -Eeuo pipefail

# navigate to directory
SCRIPT_PATH="$(
  cd "$(dirname "$0")" || exit
  pwd -P
)"
cd "$SCRIPT_PATH" || exit

cd ..

# Default framework is actix if no argument is passed
framework=${1:-actix}

# Build and run the specified framework server
case $framework in
actix)
  echo "Building and running Actix server..."
  cargo build -p tactix
  cargo run -p tactix &
  PID=$!
  ;;
axum)
  echo "Building and running Axum server..."
  cargo build -p taxum
  cargo run -p taxum &
  PID=$!
  ;;
hyper)
  echo "Building and running Hyper server..."
  cargo build -p typer
  cargo run -p typer &
  PID=$!
  ;;
rocket)
  echo "Building and running Rocket server..."
  cargo build -p tocket
  cargo run -p tocket &
  PID=$!
  ;;
*)
  echo "Unknown framework: $framework"
  echo "Usage: $0 [actix|axum|hyper|rocket]"
  exit 1
  ;;
esac
sleep 1

rm db.local.json
rm output.local.txt
cat <<EOF >db.local.json
{}
EOF

# cargo build -p tactix
# cargo run -p tactix &
# cargo run -p taxum &
# cargo run -p typer &
# cargo run -p rocketsh &
# PID=$!
sleep 1
echo "Server started with PID: $PID"

curl -X POST http://127.0.0.1:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "writing", "status": "PENDING"}' >>output.local.txt

echo "" >>output.local.txt

curl -X POST http://127.0.0.1:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "coding", "status": "PENDING"}' >>output.local.txt

echo "" >>output.local.txt

curl -X DELETE http://127.0.0.1:8080/api/v1/tasks/coding >>output.local.txt

echo "" >>output.local.txt

curl -X PATCH http://127.0.0.1:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -H "token: some token" \
  -d '{"title": "writing", "status": "DONE"}' >>output.local.txt

kill "$PID"
