// Lab 03 — Functions
// Run: npx ts-node lab03-functions/functions.ts

// ── Basic function with typed parameters and return type ──────────────────
function greet(name: string): string {
  return `Hello, ${name}!`;
}
console.log(greet("TypeScript"));

// ── Arrow functions ───────────────────────────────────────────────────────
const add = (a: number, b: number): number => a + b;
console.log("3 + 4 =", add(3, 4));

// ── Optional parameters ────────────────────────────────────────────────────
function greetWithTitle(name: string, title?: string): string {
  if (title) {
    return `Hello, ${title} ${name}!`;
  }
  return `Hello, ${name}!`;
}
console.log(greetWithTitle("Alice"));
console.log(greetWithTitle("Alice", "Dr."));

// ── Default parameters ─────────────────────────────────────────────────────
function repeat(text: string, times: number = 3): string {
  return text.repeat(times);
}
console.log(repeat("ha "));         // uses default: 3
console.log(repeat("Go! ", 5));     // override: 5

// ── void return type ──────────────────────────────────────────────────────
function logMessage(message: string): void {
  console.log(`[LOG] ${message}`);
}
logMessage("Functions lab complete!");

// ── Functions as values ────────────────────────────────────────────────────
// TypeScript can type a function itself
const transform = (value: number, fn: (n: number) => number): number => fn(value);
const double = (n: number) => n * 2;
console.log("double(7) =", transform(7, double));
