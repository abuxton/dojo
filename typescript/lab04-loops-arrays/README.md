# Lab 04 — Loops & Arrays 🔁

> **Goal:** iterate with `for`, `for...of`, and `while`, and use Array methods to transform data.

---

## What you will learn

- `for`, `for...of`, `while` loops
- Typed arrays: `string[]`, `number[]`
- Array methods: `map`, `filter`, `reduce`, `forEach`
- Spread operator (`...`) and rest parameters

---

## Steps

### 1 — Run the script

```bash
npx ts-node lab04-loops-arrays/loops.ts
```

**Expected output:**
```
── Classic for loop ──
  Step 1
  Step 2
  Step 3
  Step 4
  Step 5

── for...of (iterate items) ──
  🍎 apple
  🍎 banana
  🍎 cherry

── while loop ──
  count = 0
  count = 1
  count = 2

── Array.map — transform each item ──
  squared: [ 1, 4, 9, 16, 25 ]

── Array.filter — keep matching items ──
  evens: [ 2, 4 ]

── Array.reduce — accumulate a result ──
  sum: 15

── Array.forEach — side-effects only ──
  [0] apple
  [1] banana
  [2] cherry

── Spread operator ──
  moreFruits: [ 'apple', 'banana', 'cherry', 'date', 'elderberry' ]

── Rest params — sumAll(1,2,3,4) = 10
```

### 2 — Challenge: FizzBuzz

Add a loop that prints numbers 1–20 but replaces multiples of 3 with `"Fizz"`, multiples of 5 with `"Buzz"`, and multiples of both with `"FizzBuzz"`:

```typescript
for (let i = 1; i <= 20; i++) {
  if (i % 15 === 0) console.log("FizzBuzz");
  else if (i % 3 === 0) console.log("Fizz");
  else if (i % 5 === 0) console.log("Buzz");
  else console.log(i);
}
```

---

## Key concepts

| Loop | Use when |
|------|----------|
| `for` | you need the index |
| `for...of` | iterating values |
| `while` | condition-based |
| `.map()` | transform → new array |
| `.filter()` | subset based on test |
| `.reduce()` | fold into single value |

---

## Reference links

- [MDN — Loops and iteration](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Loops_and_iteration)
- [MDN — Array methods](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array)
- [TypeScript Handbook — Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html)
- [TypeScript Playground](https://www.typescriptlang.org/play)

---

⬅️ [Lab 03 — Functions](../lab03-functions/) &nbsp;&nbsp; ➡️ [Lab 05 — Interfaces & Objects](../lab05-interfaces-objects/)
