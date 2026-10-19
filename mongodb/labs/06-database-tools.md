# Lab 06: Export and restore with MongoDB Database Tools

## Goal

Create both a readable JSON export and a BSON backup of the Lab 01 students
collection. `mongoexport` and `mongodump` run from the Compose `tools`
container, so no host Database Tools installation is needed. Run Lab 01 first
so the collection has predictable sample data.

## Run the export

```sh
npm run lesson:01
npm run lab:05:export
```

The output is deliberately ignored by Git:

- `.mongodb-learning-artifacts/lab-06/lesson_01_students.json` is a readable
  JSON export created by `mongoexport`.
- `.mongodb-learning-artifacts/lab-06/dump/` is a BSON backup created by
  `mongodump`, preserving BSON types and indexes where applicable.

Inspect the JSON with:

```sh
cat .mongodb-learning-artifacts/lab-06/lesson_01_students.json
```

## Restore safely

Restore only into a new learning database, never over the original lesson
database:

```sh
mongorestore \
  --uri="mongodb://127.0.0.1:27017/mongodb_learning_restore" \
  .mongodb-learning-artifacts/lab-06/dump/mongodb_learning
```

Compare the original and restored collections in Compass or with `mongosh`.
When finished, remove the temporary restore database from `mongosh` with
`db.getSiblingDB("mongodb_learning_restore").dropDatabase()`.
