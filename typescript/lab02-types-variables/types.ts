// Lab 02 — Types & Variables
// Run: npx ts-node lab02-types-variables/types.ts

// ── Primitive types ────────────────────────────────────────────────────────
const name: string = "Alice";
const age: number = 30;
const isLearning: boolean = true;

console.log(`Name: ${name}, Age: ${age}, Learning: ${isLearning}`);

// ── Type inference ─────────────────────────────────────────────────────────
// TypeScript figures out the type automatically when you assign a value
const city = "London"; // TypeScript infers: string
const temperature = 18.5; // TypeScript infers: number
console.log(`${city} is ${temperature}°C`);

// ── Arrays ────────────────────────────────────────────────────────────────
const languages: string[] = ["TypeScript", "JavaScript", "Python"];
const scores: number[] = [95, 87, 100];
console.log("Languages:", languages);
console.log("Top score:", Math.max(...scores));

// ── Union types ───────────────────────────────────────────────────────────
// A variable that can be one of several types
let id: string | number = "abc-123";
console.log("ID (string):", id);
id = 42;
console.log("ID (number):", id);

// ── const vs let ──────────────────────────────────────────────────────────
let mutableValue = "I can change";
mutableValue = "Changed!";
// const immutableValue = "I cannot change";
// immutableValue = "Error!"; // ❌ would cause a compile error
console.log(mutableValue);

// ── Any and unknown ───────────────────────────────────────────────────────
// Avoid 'any' — it turns off type checking
// Use 'unknown' when you truly don't know the type yet
const userInput: unknown = "some value from an API";
if (typeof userInput === "string") {
  console.log("User input length:", userInput.length);
}
