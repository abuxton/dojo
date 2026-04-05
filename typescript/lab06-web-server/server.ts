// Lab 06 — Simple Web Server
// Run: npx ts-node lab06-web-server/server.ts
// Open: http://localhost:3000

import express, { Request, Response } from "express";

const app = express();
const PORT = 3000;

// ── Routes ────────────────────────────────────────────────────────────────

// Home page — returns a styled HTML page
app.get("/", (_req: Request, res: Response) => {
  res.send(`
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>TypeScript Web Server</title>
  <style>
    body { font-family: system-ui, sans-serif; max-width: 700px; margin: 80px auto; padding: 0 20px; }
    h1   { color: #3178c6; }
    code { background: #f0f0f0; padding: 2px 6px; border-radius: 4px; }
    a    { color: #3178c6; }
    ul   { line-height: 2; }
  </style>
</head>
<body>
  <h1>🎉 Hello from TypeScript!</h1>
  <p>You are running a web server written in TypeScript, served with <strong>Express</strong>.</p>

  <h2>Routes in this server</h2>
  <ul>
    <li><a href="/">/</a> — this page</li>
    <li><a href="/hello">/hello</a> — JSON greeting</li>
    <li><a href="/labs">/labs</a> — list of dojo labs</li>
  </ul>

  <h2>How to run</h2>
  <p>One command from the <code>typescript/</code> directory:</p>
  <pre><code>npx ts-node lab06-web-server/server.ts</code></pre>
</body>
</html>
  `);
});

// JSON endpoint
app.get("/hello", (_req: Request, res: Response) => {
  res.json({
    message: "Hello from TypeScript!",
    timestamp: new Date().toISOString(),
    language: "TypeScript",
  });
});

// Lab listing endpoint
app.get("/labs", (_req: Request, res: Response) => {
  const labs = [
    { id: 1, title: "Hello World", path: "lab01-hello-world" },
    { id: 2, title: "Types & Variables", path: "lab02-types-variables" },
    { id: 3, title: "Functions", path: "lab03-functions" },
    { id: 4, title: "Loops & Arrays", path: "lab04-loops-arrays" },
    { id: 5, title: "Interfaces & Objects", path: "lab05-interfaces-objects" },
    { id: 6, title: "Web Server", path: "lab06-web-server" },
  ];
  res.json({ labs });
});

// ── Start server ──────────────────────────────────────────────────────────
app.listen(PORT, () => {
  console.log(`\n🚀 TypeScript web server running!`);
  console.log(`   Local:  http://localhost:${PORT}`);
  console.log(`   Press Ctrl+C to stop\n`);
});
