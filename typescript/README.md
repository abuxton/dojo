# TypeScript Dojo 🥋

A hands-on, implementation-first TypeScript curriculum. Each lab builds on the previous one — from a single `console.log` all the way to serving a web page from your laptop.

---

## Prerequisites

| Tool | Install |
|------|---------|
| **Node.js ≥ 18** | https://nodejs.org |
| **npm** | ships with Node.js |

> You do **not** need to install TypeScript globally. Every lab uses `npx` so everything is self-contained.

---

## Quick start (all labs)

```bash
cd typescript/
npm install          # install shared deps (typescript, ts-node, express)
npm run lab01        # run any lab
```

---

## Curriculum

| Lab | Topic | One-liner to run |
|-----|-------|-----------------|
| [Lab 01](./lab01-hello-world/) | Hello World | `npx ts-node lab01-hello-world/hello.ts` |
| [Lab 02](./lab02-types-variables/) | Types & Variables | `npx ts-node lab02-types-variables/types.ts` |
| [Lab 03](./lab03-functions/) | Functions | `npx ts-node lab03-functions/functions.ts` |
| [Lab 04](./lab04-loops-arrays/) | Loops & Arrays | `npx ts-node lab04-loops-arrays/loops.ts` |
| [Lab 05](./lab05-interfaces-objects/) | Interfaces & Objects | `npx ts-node lab05-interfaces-objects/interfaces.ts` |
| [Lab 06](./lab06-web-server/) | Web Server | `npx ts-node lab06-web-server/server.ts` |

---

## The "one-liner" philosophy

TypeScript compiles to JavaScript which runs on Node.js. The tool `ts-node` lets you skip the compile step and run `.ts` files directly:

```bash
npx ts-node <file>.ts
```

For a web server (Lab 06), that single command starts a server you can open in your browser.

---

## Reference links

- [TypeScript Official Docs](https://www.typescriptlang.org/docs/)
- [TypeScript Playground](https://www.typescriptlang.org/play) — try code in the browser without installing anything
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html)
- [Node.js Docs](https://nodejs.org/en/docs)
- [ts-node](https://typestrong.org/ts-node/)
- [Express.js](https://expressjs.com/)
