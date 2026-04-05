// Lab 01 — Hello World
// Run: npx ts-node hello.ts

const message: string = "Hello, TypeScript!";
console.log(message);

// TypeScript knows the type — hover over 'message' in VS Code to see it
const year: number = new Date().getFullYear();
console.log(`Welcome to TypeScript in ${year}!`);
