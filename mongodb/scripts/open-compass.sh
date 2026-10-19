#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$script_dir/mongodb-tools-env.sh"

if [ "$(uname)" != "Darwin" ]; then
  printf '%s\n' "Compass must be opened from its desktop application on this platform."
  printf '%s\n' "Connect it to: $MONGODB_CONNECTION_STRING"
  exit 1
fi

if ! open -Ra "MongoDB Compass"; then
  printf '%s\n' "MongoDB Compass is not installed. Run: brew bundle --file Brewfile"
  exit 1
fi

exec open -a "MongoDB Compass" "$MONGODB_CONNECTION_STRING"
