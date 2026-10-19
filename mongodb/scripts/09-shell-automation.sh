#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$script_dir/mongodb-tools-env.sh"

exec docker compose run --rm tools \
  mongosh "$MONGODB_CONTAINER_CONNECTION_STRING" --quiet --eval '
    const students = db
      .getSiblingDB("mongodb_learning")
      .getCollection("lesson_01_students")
      .find(
        { completedLessons: { $gte: 2 } },
        { _id: 0, name: 1, track: 1, completedLessons: 1 },
      )
      .sort({ name: 1 })
      .toArray();
    print(JSON.stringify(students));
  '
