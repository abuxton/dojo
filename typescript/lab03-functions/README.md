# Lab 03 — Functions ⚙️

> **Goal:** define, call, and compose typed functions.

---

## What you will learn

- Function declarations with typed parameters and return types
- Arrow functions (`const fn = () => {}`)
- Optional parameters (`name?: string`)
- Default parameters (`count = 3`)
- `void` return type
- Passing functions as arguments (higher-order functions)

---

## Steps

### 1 — Run the script

```bash
npx ts-node lab03-functions/functions.ts
```

**Expected output:**
```
Hello, TypeScript!
3 + 4 = 7
Hello, Alice!
Hello, Dr. Alice!
ha ha ha 
Go! Go! Go! Go! Go! 
[LOG] Functions lab complete!
double(7) = 14
```

### 2 — Try missing a required argument

Call `greet()` without arguments and observe the compile error:
```typescript
greet(); // Error: Expected 1 arguments, but got 0.
```

### 3 — Write your own function

Add a function that takes two strings and returns the longer one:

```typescript
function longest(a: string, b: string): string {
  return a.length >= b.length ? a : b;
}
console.log(longest("cat", "elephant")); // elephant
```

---

## Key concepts

| Syntax | Meaning |
|--------|---------|
| `(name: string): string` | param type + return type |
| `name?: string` | optional parameter |
| `count = 3` | default value |
| `(): void` | returns nothing |
| `(n: number) => number` | function type annotation |

---

## Reference links

- [TypeScript Handbook — Functions](https://www.typescriptlang.org/docs/handbook/2/functions.html)
- [MDN — Arrow functions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions)
- [TypeScript Playground](https://www.typescriptlang.org/play)

---

⬅️ [Lab 02 — Types & Variables](../lab02-types-variables/) &nbsp;&nbsp; ➡️ [Lab 04 — Loops & Arrays](../lab04-loops-arrays/)
