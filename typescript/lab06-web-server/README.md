# Lab 06 — Simple Web Server 🌐

> **Goal:** serve a web page from your laptop with a single command — written in TypeScript.

---

## What you will learn

- Using the `express` framework with TypeScript
- Type-safe `Request` and `Response` objects
- Serving HTML, JSON, and multiple routes
- Running a full web server with `npx ts-node`

---

## The One-Liner

From the `typescript/` directory:

```bash
npx ts-node lab06-web-server/server.ts
```

Then open your browser at **http://localhost:3000** 🎉

---

## Steps

### 1 — Install dependencies (once)

```bash
cd typescript/
npm install
```

### 2 — Start the server

```bash
npx ts-node lab06-web-server/server.ts
```

**Expected terminal output:**
```
🚀 TypeScript web server running!
   Local:  http://localhost:3000
   Press Ctrl+C to stop
```

### 3 — Explore the routes

| URL | Returns |
|-----|---------|
| http://localhost:3000/ | Styled HTML home page |
| http://localhost:3000/hello | JSON greeting |
| http://localhost:3000/labs | JSON list of all labs |

### 4 — Try it with curl (optional)

```bash
curl http://localhost:3000/hello
```

```json
{
  "message": "Hello from TypeScript!",
  "timestamp": "2025-01-01T00:00:00.000Z",
  "language": "TypeScript"
}
```

### 5 — Challenge: add your own route

Add a `/greet/:name` route that personalises the greeting:

```typescript
app.get("/greet/:name", (req: Request, res: Response) => {
  const { name } = req.params;
  res.send(`<h1>Hello, ${name}! 👋</h1>`);
});
```

Then visit http://localhost:3000/greet/Alice

---

## How it works

```
Client (browser)
     │
     │ HTTP GET /
     ▼
Express router
     │
     │ matches route handler
     ▼
res.send(html)  ──▶  browser renders the page
```

Express is a minimal Node.js web framework. TypeScript adds types for `Request` and `Response` so your IDE can autocomplete and catch mistakes.

---

## Key concepts

| Concept | Code |
|---------|------|
| Create app | `const app = express()` |
| Add route | `app.get("/path", handler)` |
| Send HTML | `res.send("<h1>hi</h1>")` |
| Send JSON | `res.json({ key: "value" })` |
| Start server | `app.listen(PORT, callback)` |
| Route params | `req.params.name` |

---

## Reference links

- [Express.js Getting Started](https://expressjs.com/en/starter/hello-world.html)
- [Express with TypeScript guide](https://expressjs.com/en/guide/using-typescript.html)
- [TypeScript @types/express](https://www.npmjs.com/package/@types/express)
- [ts-node](https://typestrong.org/ts-node/docs/)
- [MDN — HTTP overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview)

---

⬅️ [Lab 05 — Interfaces & Objects](../lab05-interfaces-objects/) &nbsp;&nbsp; 🏠 [Back to TypeScript Dojo](../README.md)
