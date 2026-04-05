// Lab 05 — Interfaces & Objects
// Run: npx ts-node lab05-interfaces-objects/interfaces.ts

// ── Interface — defines the shape of an object ────────────────────────────
interface User {
  id: number;
  name: string;
  email: string;
  role?: string; // optional property
}

const alice: User = { id: 1, name: "Alice", email: "alice@example.com", role: "admin" };
const bob: User = { id: 2, name: "Bob", email: "bob@example.com" }; // role is optional

console.log("User:", alice);
console.log("User:", bob);

// ── Function that accepts an interface ────────────────────────────────────
function printUser(user: User): void {
  const role = user.role ?? "member";
  console.log(`  ${user.name} <${user.email}> — ${role}`);
}

printUser(alice);
printUser(bob);

// ── Interface extending another interface ─────────────────────────────────
interface AdminUser extends User {
  permissions: string[];
}

const superAdmin: AdminUser = {
  id: 3,
  name: "Charlie",
  email: "charlie@example.com",
  role: "superadmin",
  permissions: ["read", "write", "delete"],
};
console.log("\nAdmin:", superAdmin.name, "— permissions:", superAdmin.permissions);

// ── Type alias — an alternative to interface for object shapes ─────────────
type Point = {
  x: number;
  y: number;
};

const origin: Point = { x: 0, y: 0 };
const moved: Point = { x: 3, y: 4 };

function distance(a: Point, b: Point): number {
  return Math.sqrt((b.x - a.x) ** 2 + (b.y - a.y) ** 2);
}
console.log(`\nDistance from origin to (3,4): ${distance(origin, moved)}`);

// ── Readonly properties ────────────────────────────────────────────────────
interface Config {
  readonly apiUrl: string;
  timeout: number;
}

const config: Config = { apiUrl: "https://api.example.com", timeout: 5000 };
config.timeout = 3000; // ✅ allowed
// config.apiUrl = "..."; // ❌ would error: Cannot assign to 'apiUrl'
console.log("\nConfig:", config);
