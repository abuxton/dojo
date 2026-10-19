# Lab 08: Repeat the driver lessons with Python

## Goal

Run the CRUD, query/index, aggregation, and client-configuration exercises
from Labs 01–04 using synchronous PyMongo rather than the Node.js driver. Each
Python exercise writes to its own `lesson_08_python_*` collection, so it does
not reset the Node.js lab data.

## Set up and run

Start the local database, create an isolated Python environment, then run the
complete Python lab:

```sh
npm run db:up
npm run lab:08:python:setup
npm run lab:08:python
```

`pymongo` reads the same `MONGODB_URI`, `MONGODB_DB`,
`MONGODB_APP_NAME`, and `MONGODB_SERVER_SELECTION_TIMEOUT_MS` values as the
Node.js client. The lab uses PyMongo's default connection-pool behavior because
these are single-process, low-concurrency exercises; it does not invent
production pool settings.

To run one exercise after setup:

```sh
PYTHONPATH=python .venv/bin/python python/lessons/02_query_and_index.py
```
