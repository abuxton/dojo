// Lab 04 — Loops & Arrays
// Run: npx ts-node lab04-loops-arrays/loops.ts

// ── for loop ──────────────────────────────────────────────────────────────
console.log("── Classic for loop ──");
for (let i = 1; i <= 5; i++) {
  console.log(`  Step ${i}`);
}

// ── for...of loop ─────────────────────────────────────────────────────────
console.log("\n── for...of (iterate items) ──");
const fruits: string[] = ["apple", "banana", "cherry"];
for (const fruit of fruits) {
  console.log(`  🍎 ${fruit}`);
}

// ── while loop ────────────────────────────────────────────────────────────
console.log("\n── while loop ──");
let count = 0;
while (count < 3) {
  console.log(`  count = ${count}`);
  count++;
}

// ── Array methods ─────────────────────────────────────────────────────────
console.log("\n── Array.map — transform each item ──");
const numbers: number[] = [1, 2, 3, 4, 5];
const squared = numbers.map((n) => n * n);
console.log("  squared:", squared);

console.log("\n── Array.filter — keep matching items ──");
const evens = numbers.filter((n) => n % 2 === 0);
console.log("  evens:", evens);

console.log("\n── Array.reduce — accumulate a result ──");
const sum = numbers.reduce((acc, n) => acc + n, 0);
console.log("  sum:", sum);

console.log("\n── Array.forEach — side-effects only ──");
fruits.forEach((fruit, index) => {
  console.log(`  [${index}] ${fruit}`);
});

// ── Spread and rest ───────────────────────────────────────────────────────
console.log("\n── Spread operator ──");
const moreFruits = [...fruits, "date", "elderberry"];
console.log("  moreFruits:", moreFruits);

function sumAll(...nums: number[]): number {
  return nums.reduce((acc, n) => acc + n, 0);
}
console.log("\n── Rest params — sumAll(1,2,3,4) =", sumAll(1, 2, 3, 4));
