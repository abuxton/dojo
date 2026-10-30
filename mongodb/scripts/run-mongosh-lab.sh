#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$script_dir/mongodb-tools-env.sh"

exec docker compose run --rm tools \
  mongosh "$MONGODB_CONTAINER_CONNECTION_STRING" --file labs/05-mongosh.js
