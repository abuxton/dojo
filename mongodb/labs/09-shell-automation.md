# Lab 09: Automate a query from a shell script

## Goal

Run a repeatable read-only MongoDB query from a POSIX shell script and emit
machine-readable JSON. The script uses `mongosh` from the Compose `tools`
service, so it works without a host shell installation.

## Run it

```sh
npm run db:up
npm run lesson:01
npm run lab:09:shell
```

The command prints the Lab 01 students who completed at least two lessons as
one JSON array. Pipe it to a JSON-aware tool or save it in a larger automation
workflow:

```sh
./scripts/09-shell-automation.sh > students.json
```

`curl` is intentionally not used: a local `mongod` accepts the MongoDB wire
protocol, not HTTP. `curl` is appropriate only when calling a separate HTTP
API such as the Atlas Data API, which this local learning deployment does not
provide.
