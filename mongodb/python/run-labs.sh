#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
python_bin="$script_dir/../.venv/bin/python"

if [ ! -x "$python_bin" ]; then
  printf '%s\n' "Python dependencies are not installed. Run: npm run lab:08:python:setup"
  exit 1
fi

for lesson in 01_crud.py 02_query_and_index.py 03_aggregation.py 04_client_configuration.py; do
  PYTHONPATH="$script_dir" "$python_bin" "$script_dir/lessons/$lesson"
done
