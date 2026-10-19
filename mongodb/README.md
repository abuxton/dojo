# MongoDB Learning Lab

A small, repeatable set of hands-on MongoDB exercises using the official
Node.js driver. Each lesson writes only to a database whose name starts with
`mongodb_learning`, and resets its own `lesson_*` collection before running.

## Start MongoDB

1. Install dependencies: `npm install`
2. Start the included local database: `npm run db:up`
3. Run the lessons in order:

   ```sh
   npm run lesson:01
   npm run lesson:02
   npm run lesson:03
   ```

The default connection is `mongodb://127.0.0.1:27017` and the default database
is `mongodb_learning`. Copy `.env.example` to `.env` if you want to record
different values, then export them in your shell before running a lesson:

```sh
export MONGODB_URI="mongodb://127.0.0.1:27017"
export MONGODB_DB="mongodb_learning"
```

Stop the local database with `npm run db:down`. Its Docker volume persists
between restarts; use Docker directly if you deliberately want to remove it.

## Kubernetes quickstart

The [Kubernetes quickstart](kubernetes/README.md) deploys MongoDB Controllers
for Kubernetes and Ops Manager to Rancher Desktop. Run `npm run k8s:guide` for
an interactive do-nothing operator guide, or `npm run k8s:deploy` after
exporting the required credentials. Run `npm run k8s:teardown` to remove the
quickstart and its local persistent data.

## Use the MongoDB MCP server

`mcp.json` configures MongoDB's official local MCP server for this learning
database. It is intentionally read-only, so an AI client can inspect the
lesson data without changing it.

Start MongoDB first, then load the configuration for a GitHub Copilot CLI
session:

```sh
npm run db:up
copilot --additional-mcp-config @mcp.json
```

For another MCP-compatible client, add the `mongodb-learning` entry from
`mcp.json` to that client's MCP configuration. The server is launched through
`npx`; no project dependency or credentials are required for this local
instance. Remove `--readOnly` only if you deliberately want an AI client to
modify the learning database.

## Learning path

| Lesson | Concepts | Visual preview | Command |
| --- | --- | --- | --- |
| 1 | Collections and CRUD | `insertMany()` → `updateOne()` → `deleteOne()` | `npm run lesson:01` |
| 2 | Querying and indexes | `find(filter, projection).sort()` | `npm run lesson:02` |
| 3 | Aggregation | `$unwind` → `$group` → `$project` → `$sort` | `npm run lesson:03` |
| 4 | Node.js client configuration | `new MongoClient(uri, options)` → `ping` | `npm run lab:04:client` |
| 5 | Interactive shell queries | `db.collection.find(filter)` | `npm run lab:05:mongosh` |
| 6 | Database Tools | `mongoexport` (JSON) + `mongodump` (BSON) | `npm run lab:06:export` |
| 7 | Compass exploration | connect → inspect documents, indexes, pipeline | `npm run lab:07:compass` |
| 8 | PyMongo | `collection.update_one(filter, update)` | `npm run lab:08:python` |
| 9 | Shell automation | `mongosh --quiet --eval` → JSON | `npm run lab:09:shell` |

Read a lesson before running it, change one query at a time, predict its
result, then run it again. The [MongoDB Manual](https://www.mongodb.com/docs/manual/)
and [Node.js driver documentation](https://www.mongodb.com/docs/drivers/node/current/)
are the primary references for concepts that need more depth.

## Guided course

Open the [guided course](lessons/index.html) for nine short, interactive
lessons tied to the runnable labs. Use `npm run <script>` for every command in
this project—`npx run` does not run package scripts. The course mission,
curated sources, reference material, and learning records live in
`MISSION.md`, `RESOURCES.md`, `reference/`, and `learning-records/`.

## Local client tools

`mongosh`, `mongoexport`, and `mongodump` run in the Compose `tools` service,
so Docker is the only requirement for Labs 05, 06, and 09. The service uses the
internal Docker connection `mongodb://mongodb:27017/mongodb_learning`, while
the Node.js lessons, MCP server, and Compass use the forwarded host port.

On macOS, install the Compass desktop client with:

```sh
brew bundle --file Brewfile
```

The scripts share `MONGODB_URI`, `MONGODB_DB`,
`MONGODB_CONNECTION_STRING`, and `MONGODB_CONTAINER_CONNECTION_STRING`.
Their safe defaults target the local `mongodb_learning` database; set these
environment variables before a command only when you intentionally need a
different local learning database. See Labs 06 and 07 for backup/restore and
Compass-specific instructions.
