# Agent Guide: MongoDB Learning Lab

## Purpose and scope

This is a deliberately small Node.js and Python learning project for MongoDB
concepts. The exercises use the official `mongodb` and `pymongo` drivers and a
local MongoDB container from `compose.yaml`. Keep changes approachable for a
learner: one main concept per lesson, small datasets, visible output, and no
framework abstractions.

## Environment and commands

- Install the driver with `npm install`.
- Start and stop the local database with `npm run db:up` and `npm run db:down`.
- Validate all lesson syntax with `npm run check`.
- Run an individual tutorial with `npm run lesson:01`, `npm run lesson:02`, or
  `npm run lesson:03`.
- The Compose `tools` service supplies `mongosh`, `mongoexport`, and
  `mongodump`; do not add host dependencies for those commands. Install only
  the Compass desktop app on macOS with `brew bundle --file Brewfile`.
- Run the tool labs with `npm run lab:04:client` (Node.js client options),
  `npm run lab:05:mongosh` (mongosh), `npm run lab:06:export` (Database
  Tools), and `npm run lab:07:compass` (Compass).
- Set up and run the PyMongo lab with `npm run lab:08:python:setup` and
  `npm run lab:08:python`. Run the read-only shell automation lab with
  `npm run lab:09:shell`.
- The defaults are `MONGODB_URI=mongodb://127.0.0.1:27017` and
  `MONGODB_DB=mongodb_learning`. `src/db.mjs` refuses database names that do
  not start with `mongodb_learning`; retain this guard in learning scripts.
- `mcp.json` starts MongoDB's official MCP server against
  `mongodb_learning`. It must remain read-only by default. Load it in GitHub
  Copilot CLI with `copilot --additional-mcp-config @mcp.json` after starting
  the local database.
- `scripts/mongodb-tools-env.sh` is the shared configuration for `mongosh`,
  Database Tools, and Compass. Preserve its local defaults and reuse it
  instead of duplicating connection strings in new tool labs.
- Configure `MongoClient` only through `src/db.mjs`. Preserve the validated
  `appName` and server-selection timeout options, and expose new client
  configuration through environment variables rather than hard-coded values.
- `compose.yaml` deliberately uses two connection addresses: the `tools`
  service connects to `mongodb:27017` inside Docker, while host processes use
  `127.0.0.1:27017`. `MONGODB_CONTAINER_CONNECTION_STRING` is for Docker
  tools; `MONGODB_CONNECTION_STRING` is for host scripts. Do not use the
  internal hostname from host scripts.

## Tutorial authoring

- Sequence concepts from documents and CRUD to querying/indexes, aggregation,
  schema design, relationships, and transactions.
- Explain the concept in `README.md` before adding a lesson. In the lesson
  itself, prefer runnable examples with concise output over long comments.
- Make lessons repeatable. A lesson may reset only its own collection named
  `lesson_<number>_*`; never drop the database or operate on unspecified
  collections.
- Keep interactive shell programs in `labs/*.js`, explanatory tool labs in
  `labs/*.md`, and reusable local-tool commands in `scripts/`. Tool output
  belongs in `.mongodb-learning-artifacts/`, which must remain ignored.
- Keep Python source under `python/`, with shared client configuration in
  `python/mongodb_learning/db.py` and exercises in `python/lessons/`.
  Python exercises must use their own `lesson_08_python_*` collections so
  they remain repeatable without resetting Node.js data.
- Teaching lessons are numbered HTML files in `lessons/`; reuse
  `assets/course.css` and `assets/quiz.js`, keep each lesson short, and link
  it to the mission, a primary source in `RESOURCES.md`, and a runnable lab.
- The teaching workspace is stateful: use `MISSION.md` to select the next
  lesson, add a numbered record only after demonstrated understanding, and
  add newly understood terms to `GLOSSARY.md`.
- Place reusable connection code in `src/db.mjs` and standalone exercises in
  `lessons/<number>-<topic>.mjs`. Add a matching `lesson:<number>` npm script,
  README learning-path row, and syntax-check entry for each lesson.
- Use MongoDB's official Manual and Node.js driver documentation as the source
  of truth. Distinguish server behavior from Node.js driver behavior when
  explaining a concept.

## Installed MongoDB agent skills

Load the installed skill that matches the task instead of relying on general
knowledge:

| Skill | Use it for |
| --- | --- |
| `mongodb-mcp-setup` | Configuring or troubleshooting the MCP server, connection string, or required MCP environment variables. Do not use it for ordinary queries when `mcp.json` already works. |
| `mongodb-connection` | Changing `MongoClient` setup in `src/db.mjs`, connection-pool options, timeouts, or connection failures. |
| `mongodb-natural-language-querying` | Turning a learner's question into a read-only `find` query or aggregation pipeline. Use the local MCP server for collection schema and sample-document context. |
| `mongodb-query-optimizer` | Explicit query-performance, slow-query, explain-plan, or index-optimization work. Do not use it for a first-pass query tutorial. |
| `mongodb-schema-design` | Data-model lessons, schema reviews, embed-versus-reference decisions, JSON Schema validation, and data-lifecycle design. |
| `mongodb-search-and-ai` | Atlas Search, Vector Search, hybrid search, embeddings, RAG, fuzzy search, autocomplete, or relevance-ranking questions. This needs an Atlas-capable setup; do not present it as supported by the local container alone. |

`mongodb-atlas-stream-processing` and `mongodb-qe-size-estimation` are not
part of the default local learning path. Use them only for an explicitly
requested Atlas Stream Processing or Queryable Encryption exercise.

## MongoDB concepts and safety

- Use BSON-native values deliberately: `ObjectId` for references, `Date` for
  timestamps, and MongoDB update/aggregation operators rather than application
  loops when teaching database operations.
- Show projections when output does not need every field; do not print
  connection strings, credentials, or full production-like documents.
- Keep sample data fictitious and non-sensitive. Do not add credentials to
  `.env.example`, source files, or documentation.
- Do not remove `--readOnly` from the MCP configuration unless the task
  explicitly needs MCP-driven writes. Keep the MCP connection string limited
  to the local `mongodb_learning` database.
- Before using a skill that requires MCP access, start MongoDB with
  `npm run db:up` and load the project configuration with
  `copilot --additional-mcp-config @mcp.json`. Seed a relevant lesson before
  inspecting it so schema and query examples have real sample data.
- For data-modifying examples, state the affected collection and make the
  operation deterministic. Avoid production URI examples and destructive
  commands such as unqualified `dropDatabase()`.

## Change and validation expectations

- Preserve ESM style, two-space indentation, semicolons, and descriptive
  names used by existing lessons.
- When behavior changes, run the affected lesson against the local database
  and run `npm run check`. If Docker is unavailable, report that limitation
  rather than claiming runtime validation.
- Keep dependencies minimal. Do not add an ODM unless a lesson explicitly
  compares it with the official driver.
