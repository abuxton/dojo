# Lab 05 — Interfaces & Objects 🧩

> **Goal:** define the shape of objects with `interface` and `type`, and compose them.

---

## What you will learn

- `interface` — describing object shapes
- Optional properties (`role?: string`)
- Functions that accept interfaces as parameters
- Extending interfaces (`AdminUser extends User`)
- `type` aliases vs `interface`
- `readonly` properties

---

## Steps

### 1 — Run the script

```bash
npx ts-node lab05-interfaces-objects/interfaces.ts
```

**Expected output:**
```
User: { id: 1, name: 'Alice', email: 'alice@example.com', role: 'admin' }
User: { id: 2, name: 'Bob', email: 'bob@example.com' }
  Alice <alice@example.com> — admin
  Bob <bob@example.com> — member

Admin: Charlie — permissions: [ 'read', 'write', 'delete' ]

Distance from origin to (3,4): 5

Config: { apiUrl: 'https://api.example.com', timeout: 3000 }
```

### 2 — Try missing a required field

```typescript
const broken: User = { id: 4, name: "Dave" };
// Error: Property 'email' is missing in type '...' but required in type 'User'
```

### 3 — Challenge: add a `Product` interface

Create an interface for a product in a shop and write a function that calculates the total price:

```typescript
interface Product {
  name: string;
  price: number;
  quantity: number;
}

function totalPrice(product: Product): number {
  return product.price * product.quantity;
}

const item: Product = { name: "Keyboard", price: 79.99, quantity: 3 };
console.log(`Total: £${totalPrice(item)}`);
```

---

## Key concepts

| Concept | Syntax |
|---------|--------|
| Interface | `interface User { name: string }` |
| Optional prop | `role?: string` |
| Readonly prop | `readonly id: number` |
| Extends | `interface Admin extends User` |
| Type alias | `type Point = { x: number; y: number }` |

---

## Reference links

- [TypeScript Handbook — Object Types](https://www.typescriptlang.org/docs/handbook/2/objects.html)
- [TypeScript Handbook — Interfaces](https://www.typescriptlang.org/docs/handbook/interfaces.html)
- [Type vs Interface — comparison](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#differences-between-type-aliases-and-interfaces)
- [TypeScript Playground](https://www.typescriptlang.org/play)

---

⬅️ [Lab 04 — Loops & Arrays](../lab04-loops-arrays/) &nbsp;&nbsp; ➡️ [Lab 06 — Web Server](../lab06-web-server/)
