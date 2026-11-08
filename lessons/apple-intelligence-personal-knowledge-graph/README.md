# Teach Session: Apple Intelligence, AFM, and Your Local Knowledge Graph

**Duration:** 60 minutes
**Prerequisites:** a Mac, Terminal, Xcode Command Line Tools, and `sqlite3`

This session examines the October 2026 8rank analysis of macOS's local
Intelligence Platform. You will inspect only your own machine, locally, with
read-only commands. Do not paste query output into chat, tickets, or source
control: the store may contain personal data about you and other people.

> **Scope and confidence:** The Personal Knowledge Graph details in the article
> are third-party reverse engineering, observed on macOS 26.6.2. They are not an
> Apple API contract and may differ by OS version, hardware, configuration, or
> disappear entirely. Apple documents the Foundation Models framework, but does
> not document it as an interface to this graph.

## Outcomes

By the end, you can:

1. distinguish the Intelligence Platform data store from Apple Foundation Models
   (AFM);
2. determine whether the local store exists without changing it;
3. inspect its SQLite metadata and run an intentional, local read-only query;
4. compile a small command-line availability check for AFM.

## Lesson 1 - Separate the two layers

The article describes `knowledgeconstructiond` and `intelligenceplatformd`,
which it says maintain a local SQLite-backed graph under
`~/Library/IntelligencePlatform`. It reports that `graph.db` records facts and
relationships as triples, and that adjacent databases may hold supporting
signals. Treat every claim beyond your own observation as a hypothesis to
verify, not as a guarantee.

AFM means **Apple Foundation Models**: Apple's language models that power Apple
Intelligence. Applications use them through the **Foundation Models** framework:

| Layer | Purpose | Supported interface |
| --- | --- | --- |
| Intelligence Platform | OS-managed local data store described in the article | No public query API established by the article |
| AFM / Foundation Models | Text and image understanding, structured generation, and tool calling | Swift `FoundationModels` framework |

This distinction matters: an app can use `SystemLanguageModel` without receiving
automatic, unrestricted access to the Intelligence Platform. Conversely,
discovering a database on disk does not show that an AFM session reads it.

## Lesson 2 - Inspect the local store safely

The utility at [`bin/intelligence-platform`](/Users/abuxton/src/github/dojo/lessons/apple-intelligence-personal-knowledge-graph/bin/intelligence-platform)
uses SQLite's read-only mode. Start with metadata:

```sh
cd /Users/abuxton/src/github/dojo/lessons/apple-intelligence-personal-knowledge-graph
./bin/intelligence-platform inventory
./bin/intelligence-platform tables
./bin/intelligence-platform schema
```

macOS may require you to grant **Full Disk Access** to the terminal application
that runs the command. Do that only if you accept the privacy implications, and
revoke it afterwards if you do not otherwise need it.
If macOS denies access, the utility exits with a message naming Full Disk
Access instead of showing the underlying `find` error.

Before querying, read the schema and formulate a narrow question. For example,
this query exposes only table names rather than personal records:

```sh
./bin/intelligence-platform query \
  "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name;"
```

To explore a particular table, first view its columns:

```sh
./bin/intelligence-platform schema <table-name>
```

Then write a minimal query yourself, with a small `LIMIT`, and keep the result
on your machine:

```sh
./bin/intelligence-platform query \
  "SELECT * FROM \"<table-name>\" LIMIT 5;"
```

The utility never modifies or copies the database. `query` rejects SQL that
doesn't start with `SELECT`, `WITH`, `EXPLAIN`, or `PRAGMA`; SQLite is also
opened with `-readonly`. Those measures are safeguards, not a substitute for
careful handling of the displayed data.

## Lesson 3 - Traverse structure before records

Start the guided, aggregate-only schema walk:

```sh
./bin/intelligence-platform walk
./bin/intelligence-platform profile entity
./bin/intelligence-platform profile stable_graph
./bin/intelligence-platform predicate-summary
```

On the machine used to develop this lesson, `entity` had fields for a source
fact—such as `subject`, `predicate`, `object`, and `sourceId`. `stable_graph`
had corresponding relationship fields plus `confidence`. The actual schema is
an implementation detail and must be read from the learner's own database.
`profile` counts structure without printing fact values; `predicate-summary`
lists relationship vocabulary codes and their counts, also without printing
values. Treat both as local observations, not a public API.

## Lesson 4 - Check AFM from a command-line utility

`bin/afm-check.swift` checks whether `SystemLanguageModel.default` is available.
It does not send a prompt or access the local graph.

```sh
cd /Users/abuxton/src/github/dojo/lessons/apple-intelligence-personal-knowledge-graph
mkdir -p .build
xcrun swiftc -parse-as-library -framework FoundationModels bin/afm-check.swift -o .build/afm-check
./.build/afm-check
```

The tool requires an SDK that includes Foundation Models and macOS 26 or later.
An unavailable result can mean that the device is ineligible, Apple Intelligence
is disabled, or the model is still preparing. Production apps should inspect
the availability reason and offer a non-AFM fallback.

Apple describes two on-device variants behind `SystemLanguageModel` (AFM Core
and AFM Core Advanced); the device chooses the variant. It also documents an
AFM Cloud model through `PrivateCloudComputeLanguageModel`, which requires the
relevant entitlement and has availability and quota considerations. Do not
assume model size, context capacity, or availability from this lesson: check
the current framework APIs at runtime.

## Exercise and check

1. Run `inventory`. Does `~/Library/IntelligencePlatform` exist on this Mac?
2. If present, list its database tables without viewing any records.
3. Explain why the article does not prove that `LanguageModelSession` can query
   `graph.db`.
4. Run `afm-check`. What availability state did it report, and what fallback
   would an app show for that state?

You have completed the session when you can answer: **the graph is a
reverse-engineered OS data store; AFM is the documented model layer; neither
fact establishes a public bridge between them.**

## Sources

- 8rank, [Anatomy of Apple's Personal Knowledge Graph](https://8rank.ai/game/apple-intelligence-personal-knowledge-graph) (third-party research; accessed 2026-11-08).
- Apple Developer, [Foundation Models](https://developer.apple.com/documentation/FoundationModels).
- Apple Developer, [Detecting available Apple Foundation Models](https://developer.apple.com/documentation/FoundationModels/detecting-available-apple-foundation-models).
- Apple Developer, [Generating content and performing tasks with Foundation Models](https://developer.apple.com/documentation/FoundationModels/generating-content-and-performing-tasks-with-foundation-models).
