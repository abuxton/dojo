#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$script_dir/mongodb-tools-env.sh"

output_dir="$script_dir/../.mongodb-learning-artifacts/lab-06"
collection="lesson_01_students"

mkdir -p "$output_dir"
docker compose run --rm tools \
  mongoexport \
  --uri="$MONGODB_CONTAINER_CONNECTION_STRING" \
  --collection="$collection" \
  --type=json \
  --out="/artifacts/lab-06/$collection.json"
docker compose run --rm tools \
  mongodump \
  --uri="$MONGODB_CONTAINER_CONNECTION_STRING" \
  --collection="$collection" \
  --out="/artifacts/lab-06/dump"

printf 'Exported %s to %s\n' "$collection" "$output_dir"
