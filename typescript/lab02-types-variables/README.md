# Lab 02 — Types & Variables 🏷️

> **Goal:** understand TypeScript's type system and how it catches mistakes before your code runs.

---

## What you will learn

- Primitive types: `string`, `number`, `boolean`
- Type inference (TypeScript guesses the type)
- Arrays with typed elements
- Union types (`string | number`)
- `const` vs `let`
- `any` vs `unknown`

---

## Steps

### 1 — Run the script

```bash
npx ts-node lab02-types-variables/types.ts
```

**Expected output:**
```
Name: Alice, Age: 30, Learning: true
London is 18.5°C
Languages: [ 'TypeScript', 'JavaScript', 'Python' ]
Top score: 100
ID (string): abc-123
ID (number): 42
Changed!
User input length: 16
```

### 2 — Experiment with type errors

Open `types.ts` and try adding this line:

```typescript
const wrong: number = "oops";
```

Run again — TypeScript will refuse to compile and show you exactly what's wrong. This is the superpower of TypeScript.

### 3 — Try type inference

Remove the `: string` annotation from `name`:

```typescript
const name = "Alice"; // TypeScript still knows it's a string
```

Hover over `name` in VS Code — you'll see `const name: string` in the tooltip.

---

## Key concepts

| Type | Example | Notes |
|------|---------|-------|
| `string` | `"hello"` | text |
| `number` | `42`, `3.14` | integers and floats |
| `boolean` | `true` / `false` | |
| `string[]` | `["a", "b"]` | array of strings |
| `string \| number` | union type | either/or |
| `unknown` | safer than `any` | must check type before use |

---

## Reference links

- [TypeScript Handbook — Basic Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html)
- [TypeScript Handbook — Narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html)
- [TypeScript Playground](https://www.typescriptlang.org/play)

---

⬅️ [Lab 01 — Hello World](../lab01-hello-world/) &nbsp;&nbsp; ➡️ [Lab 03 — Functions](../lab03-functions/)
