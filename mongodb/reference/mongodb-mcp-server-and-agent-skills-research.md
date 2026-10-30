# Research note: MongoDB MCP Server and `mongodb/agent-skills`

Primary sources only. Use alongside `RESOURCES.md` (curated links) and the
`mongodb-mcp-setup` skill (interactive configuration walkthrough).

## Purpose

- The MongoDB MCP Server lets an AI client "interact with your MongoDB data
  and deployments from AI clients in natural language," for any deployment
  type including Atlas, Atlas Local, Community Edition, and Enterprise
  Advanced.
  (https://www.mongodb.com/docs/mcp-server/overview/)
- `mongodb/agent-skills` is "Collection of official MongoDB agent skills for
  use in agentic workflows," bundling skills for query writing, schema
  design, query optimization, and Atlas Search/Vector Search.
  (https://github.com/mongodb/agent-skills)

## Two deployment types (MCP Server)

- **Local MCP for MongoDB MCP Server**: self-hosted; you run, upgrade, and
  manage credentials yourself. Works with Atlas, Atlas Local, Community
  Edition, and Enterprise Advanced.
  (https://www.mongodb.com/docs/mcp-server/overview/)
- **MongoDB Atlas Managed MCP Server**: MongoDB hosts and runs it in Atlas;
  no server infrastructure to deploy; Atlas-only; supports org-level opt-in
  and read-only enforcement for governance.
  (https://www.mongodb.com/docs/mcp-server/overview/)

## Installation / setup

- MongoDB MCP Plugin install commands per AI client (Claude, Cursor, Codex,
  Copilot CLI, VS Code, Grok) and manual configuration-file setup for
  programmatic agents are both documented on the Get Started page.
  (https://www.mongodb.com/docs/mcp-server/get-started/)
- Local MCP requires Node.js 22.12.0+ (`node --version` to check), and
  Docker if using local Atlas deployments.
  (https://www.mongodb.com/docs/mcp-server/get-started/)
- This project's `mcp.json` launches the server via `npx -y
  "mongodb-mcp-server@latest" --readOnly` with
  `MDB_MCP_CONNECTION_STRING=mongodb://127.0.0.1:27017/mongodb_learning` —
  consistent with the documented `connectionString` /
  `MDB_MCP_CONNECTION_STRING` option and the `--readOnly` /
  `MDB_MCP_READ_ONLY` flag.
  (https://www.mongodb.com/docs/mcp-server/local-mcp/configuration/options/)
- `mongodb/agent-skills` installation paths: per-client plugin commands
  (e.g. Claude `/plugin install mongodb-atlas` or `mongodb`; Codex `codex
  plugin marketplace add mongodb/agent-skills` then `/plugins`; Copilot CLI
  `/plugin install https://github.com/mongodb/agent-skills.git`), the
  Vercel skills directory (`npx skills add mongodb/agent-skills`), or a
  local clone of the repo with the `skills/` directory copied into the
  agent's skill path.
  (https://github.com/mongodb/agent-skills,
  https://raw.githubusercontent.com/mongodb/agent-skills/main/README.community.md)
- Installing skills directly does **not** configure an MCP server; the docs
  say to additionally run `npx "mongodb-mcp-server@latest" setup`.
  (https://github.com/mongodb/agent-skills)

## Supported tools / capabilities (MCP Server)

Tool categories: **Atlas tools** (org/project/cluster/db-user management,
Performance Advisor, Atlas Stream Processing), **Local Atlas deployment
tools**, **Database tools**, and **Assistant tools** (MongoDB Assistant
knowledge-base search). Not every tool is available on both the Atlas
Managed and Local deployment types.
(https://www.mongodb.com/docs/mcp-server/tools/)

- Example Atlas tools: `atlas-list-orgs`, `atlas-list-projects`,
  `atlas-create-project`, `atlas-list-clusters`, `atlas-create-cluster`,
  `atlas-create-free-cluster`, `atlas-upgrade-cluster`,
  `atlas-pause-resume-cluster`, `atlas-create-access-list`,
  `atlas-create-db-user`, `atlas-get-performance-advisor` (requires
  `Project Read Only` or higher), `atlas-load-sample-dataset`,
  `atlas-streams-*` (Stream Processing).
  (https://www.mongodb.com/docs/mcp-server/tools/)
- Local Atlas deployment tools (Local MCP only, need Docker):
  `atlas-local-list-deployments`, `atlas-local-create-deployment`,
  `atlas-local-connect-deployment`, `atlas-local-delete-deployment` (the
  create/delete tools require read-only mode to be disabled).
  (https://www.mongodb.com/docs/mcp-server/tools/)
- Database tools: `connect`, `disconnect`, `list-connections`, `find`,
  `aggregate`, `aggregate-db`, `count`, `explain`, `insert-many`,
  `create-index` (including vector search indexes).
  (https://www.mongodb.com/docs/mcp-server/tools/)

`mongodb/agent-skills` bundled skills (installed in this project under
`.agents/skills/`): `mongodb-mcp-setup`, `mongodb-connection`,
`mongodb-natural-language-querying`, `mongodb-query-optimizer`,
`mongodb-schema-design`, `mongodb-search-and-ai`,
`mongodb-atlas-stream-processing`, `mongodb-qe-size-estimation`.
(https://github.com/mongodb/agent-skills — `skills/` directory listing)

## Configuration / authentication

- MCP Server options can be set via JSON config file, CLI flags, or OS
  environment variables (e.g. `MDB_MCP_CONNECTION_STRING`,
  `MDB_MCP_READ_ONLY`, `MDB_MCP_API_CLIENT_ID`/`MDB_MCP_API_CLIENT_SECRET`,
  `MDB_MCP_DISABLE_SERVER_SIDE_JS`, `MDB_MCP_DISABLED_TOOLS`,
  `MDB_MCP_DRY_RUN`, `MDB_MCP_LOGGERS`, `MDB_MCP_LOG_PATH`).
  (https://www.mongodb.com/docs/mcp-server/local-mcp/configuration/options/)
- Avoid passing the connection string at runtime where possible, because
  doing so "expose[s] the connection credentials to the large language
  model."
  (https://www.mongodb.com/docs/mcp-server/local-mcp/configuration/options/)
- Atlas tools require a service account with scoped permissions (e.g.
  `Project Read Only`, `Project Cluster Manager`, `Project Database Access
  Admin`, `Organization Member`); the docs recommend project-level roles
  and say to "Avoid `Organization Owner` unless you require full
  administrative control." The MCP Server does not hide tools based on the
  service account's permissions — calling an unauthorized tool can error.
  (https://www.mongodb.com/docs/mcp-server/prerequisites/)
- Atlas service-account setup also requires adding the MCP server's IP
  address to the project's or service account's API Access List.
  (https://www.mongodb.com/docs/mcp-server/prerequisites/)
- Supported authentication mechanisms vary by deployment: SCRAM and X.509
  everywhere; Kerberos/LDAP/OIDC on Enterprise; LDAP/OIDC on Atlas M10+.
  (https://www.mongodb.com/docs/mcp-server/local-mcp/security/)
- `mongodb/agent-skills`' `mongodb` plugin (self-managed/Community/
  Enterprise) runs the MCP server locally via `npx` and documents using the
  `mongodb-mcp-setup` skill or the MCP server configuration-options page for
  authentication.
  (https://raw.githubusercontent.com/mongodb/agent-skills/main/README.community.md)
- The `mongodb-atlas` plugin instead connects to the MongoDB-hosted Atlas
  MCP server over HTTP using OAuth, prompting a browser sign-in on first
  use.
  (https://github.com/mongodb/agent-skills)

## Workflows / learner-relevant examples

- Example natural-language prompts documented on the MCP examples page:
  "Show my Atlas organizations and projects" (runs `atlas-list-orgs`,
  `atlas-list-projects`); a single-prompt workflow that creates a project,
  free cluster, access list entry, and database user and returns the
  connection string (chains `atlas-create-project` →
  `atlas-list-projects` → `atlas-create-free-cluster` →
  `atlas-create-access-list` → `atlas-create-db-user`); "What Atlas regions
  are available on GCP?" (`atlas-get-regions`); creating a dedicated
  cluster with explicit type/instance-size/backup options
  (`atlas-create-cluster`).
  (https://www.mongodb.com/docs/mcp-server/examples/)
- Documented use cases: data exploration (explain schema/relationships),
  database operations (CRUD via natural language), database management
  (admin tasks), code generation (describe data, generate scripts), and
  performance optimization (slow-query analysis, index suggestions).
  (https://www.mongodb.com/docs/mcp-server/overview/)
- `mongodb-natural-language-querying` (an agent-skills skill) documents a
  concrete workflow for this repo: fetch indexes
  (`mcp__mongodb__collection-indexes`) and schema
  (`mcp__mongodb__collection-schema`) and sample documents
  (`mcp__mongodb__find`) before generating a query, validate field names
  against the fetched schema, and prefer `find` over aggregation when
  possible.
  (https://raw.githubusercontent.com/mongodb/agent-skills/main/skills/mongodb-natural-language-querying/SKILL.md)
- This project's own README documents the matching local workflow: start
  the database (`npm run db:up`), then load `mcp.json` in Copilot CLI with
  `copilot --additional-mcp-config @mcp.json` (see `README.md` in this
  directory, not an external source).

## Limitations / safety recommendations

- Default is to allow cluster write operations; the docs explicitly
  recommend: "Typically, always enable read-only mode"
  (`--readOnly` / `MDB_MCP_READ_ONLY`).
  (https://www.mongodb.com/docs/mcp-server/local-mcp/configuration/options/)
- Server-side JavaScript (`$where`, `$function`, `$accumulator`) is
  disabled by default ("to prevent unintended behavior") and is deprecated
  in MongoDB 8.0+.
  (https://www.mongodb.com/docs/mcp-server/local-mcp/configuration/options/)
- Avoid runtime connection strings because they expose credentials to the
  LLM; lock down the log directory so only the server's user can read/write
  it.
  (https://www.mongodb.com/docs/mcp-server/local-mcp/configuration/options/)
- For Atlas service accounts, prefer project-scoped roles over
  `Organization Owner`, and add only specific IP/CIDR ranges to the API
  Access List — `0.0.0.0/0` "is a significant security risk" and should only
  ever be temporary.
  (https://www.mongodb.com/docs/mcp-server/prerequisites/,
  agent-skills `mongodb-mcp-setup` SKILL.md)
- The local Atlas deployment tools that create/delete deployments
  (`atlas-local-create-deployment`, `atlas-local-delete-deployment`)
  require read-only mode to be disabled first.
  (https://www.mongodb.com/docs/mcp-server/tools/)
- `mongodb-search-and-ai` (agent-skills) needs an Atlas-capable deployment;
  this project's `AGENTS.md` notes it is "not supported by the local
  container alone" (local project convention, cross-referenced here, not
  an external-source claim).

## Gaps / not yet verified

- The exact OAuth scopes and token-refresh behavior of the Atlas Managed
  MCP Server were described only at a high level on the overview page; the
  dedicated security page (`/docs/mcp-server/remote-mcp/security/`) was
  referenced but not independently fetched for this note.
- Did not verify every entry in the full Database/Atlas tools tables (e.g.
  `delete-many`, `update-many`, `rename-collection`) against the live
  tools page; the examples above are representative, not exhaustive — see
  https://www.mongodb.com/docs/mcp-server/tools/ for the complete tables.
