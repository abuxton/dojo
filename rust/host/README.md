# host — Minimal HTTP example server (Rust)

Small example HTTP server inspired by The Rust Book (single-threaded server).
Serves `hello.html` in the crate root and responds with a simple 200 OK HTML response.

## Requirements

- Rust toolchain (rustup)
- cargo (comes with Rust)

Tested with modern stable Rust.

## Quick start

From the crate directory:

```bash
cd /Users/abuxton/src/github/dojo/rust/host
cargo run
```

Open a browser or curl:

```bash
curl http://127.0.0.1:7878/
# or open http://127.0.0.1:7878/ in your browser
```

You should see the HTML from `hello.html`:
- filepath: `host/hello.html`

## Generate and view documentation (rustdoc)

To generate API documentation for this crate and open it in your browser:

```bash
cd /Users/abuxton/src/github/dojo/rust/host
cargo doc --no-deps --open
```

`cargo doc` will generate HTML docs under `target/doc`. `--no-deps` restricts output to this crate; `--open` attempts to open the docs in your default browser.

Note: This crate is an executable (binary) with its documentation primarily being crate-level notes and README. If you want item-level docs, add `///` doc comments to functions and modules in the source.

## Code overview

Main implementation: `host/src/main.rs`

- Binds a TCP listener on `127.0.0.1:7878`.
- Accepts incoming connections and handles them sequentially.
- `handle_connection` reads the HTTP request headers and returns a 200 OK response with `Content-Type: text/html`.
- The server currently reads `hello.html` at runtime:
  - `fs::read_to_string("hello.html")`
  - For a static embed at compile-time, prefer `include_str!("hello.html")` and place `hello.html` next to `main.rs` (e.g., `host/src/hello.html`).

Suggested small improvements already applied in source:
- Add `Content-Length`, `Content-Type`, and `Connection` headers.
- Use `expect`/`unwrap` messages for clearer failures, or handle errors gracefully for production.

## Linting, formatting, and checks

Run these maintenance commands from crate root:

```bash
# Check compilation
cargo check

# Run clippy (lints) and treat warnings as errors
cargo clippy -- -D warnings

# Format code
cargo fmt -- --check
```

## Tests

This crate currently has no unit tests. To add tests, create a `tests/` or add `#[cfg(test)]` modules in `src`.

## Files

- src/main.rs — server implementation
- hello.html — HTML served by the server
- Cargo.toml — crate metadata

## Notes

- This project is intended as a learning/example workspace. For production servers, use async runtimes (tokio, async-std), proper error handling, request parsing libraries, and multi-threading or async IO.
- To embed `hello.html` at compile time with `include_str!`, move `hello.html` into `src/` and change the code to:
  ```rust
  let contents = include_str!("hello.html");
  ```

## License

Refer to repository root for license information.