# Lab 01 — Hello World 👋

> **Goal:** write and run your first TypeScript script in one command.

---

## What you will learn

- How TypeScript differs from JavaScript
- Basic type annotation (`: string`, `: number`)
- Running a `.ts` file directly with `npx ts-node`

---

## Steps

### 1 — Install dependencies (once per project)

```bash
cd typescript/
npm install
```

### 2 — Run the script

```bash
npx ts-node lab01-hello-world/hello.ts
```

**Expected output:**
```
Hello, TypeScript!
Welcome to TypeScript in 2025!
```

### 3 — Open the file and experiment

Open `hello.ts`. Try changing the `message` variable to your name.  
Notice that the type annotation `: string` means TypeScript will warn you if you accidentally assign a number.

Try this — it will cause a compile error on purpose:
```typescript
const message: string = 42; // Error: Type 'number' is not assignable to type 'string'
```

---

## Key concepts

| Concept | JavaScript | TypeScript |
|---------|-----------|-----------|
| Variable | `let x = "hi"` | `let x: string = "hi"` |
| Type safety | ❌ runtime | ✅ compile time |
| Tooling hints | basic | rich (autocomplete, errors) |

---

## Reference links

- [TypeScript in 5 minutes](https://www.typescriptlang.org/docs/handbook/typescript-in-5-minutes.html)
- [ts-node docs](https://typestrong.org/ts-node/docs/)
- [TypeScript Playground](https://www.typescriptlang.org/play) — paste code, see output instantly

---

➡️ **Next:** [Lab 02 — Types & Variables](../lab02-types-variables/)
