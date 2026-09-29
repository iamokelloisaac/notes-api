# notes-api

A small, heavily documented REST service for managing notes, built with
[axum](https://docs.rs/axum) 0.8 and [Tokio](https://tokio.rs).

It doubles as a guided learning project: it is developed in milestones that
follow a chapter outline covering the Axum fundamentals, routing, path routing
and handlers. Each milestone is documented as if this were a public API.

| | |
|---|---|
| **Status** | Milestone 1 of 4 complete (server skeleton + `GET /health`) |
| **Rust** | 1.85 or newer (edition 2024) |
| **License** | Not yet chosen |

---

## Table of contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Running the server](#running-the-server)
4. [Configuration](#configuration)
5. [Trying it out](#trying-it-out)
6. [Development workflow](#development-workflow)
7. [Project layout](#project-layout)
8. [Documentation index](#documentation-index)
9. [Troubleshooting](#troubleshooting)

---

## Prerequisites

* A Rust toolchain, version **1.85 or newer**. Check with:

  ```bash
  rustc --version
  ```

* If Rust is not installed, get it with [rustup](https://rustup.rs):

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

* If Rust is installed but too old:

  ```bash
  rustup update stable
  ```

* `curl` (or any HTTP client) to call the API.

## Installation

```bash
# 1. Get the source (adapt to wherever you keep the project)
cd notes-api

# 2. Download dependencies and compile
cargo build
```

The first build downloads and compiles all dependencies, so it takes a while.
Later builds are incremental.

## Running the server

```bash
cargo run
```

You should see a log line similar to:

```text
INFO notes_api: listening addr=127.0.0.1:3000
```

Stop the server with `Ctrl+C`. It shuts down **gracefully**: it stops accepting
new connections and lets in-flight requests finish. On Unix it also reacts to
`SIGTERM`, which is what containers and process managers send.

## Configuration

Configuration is read from environment variables at start-up. Invalid values
abort start-up with a clear error message.

| Variable     | Description                                         | Default          |
|--------------|-----------------------------------------------------|------------------|
| `NOTES_ADDR` | Socket address to bind, e.g. `0.0.0.0:8080`         | `127.0.0.1:3000` |
| `RUST_LOG`   | Log filter, e.g. `debug` or `notes_api=trace,info`  | `notes_api=debug,info` |

The default binds to loopback only, so the server is **not** reachable from
other machines unless you set `NOTES_ADDR=0.0.0.0:3000`.

```bash
NOTES_ADDR=0.0.0.0:8080 RUST_LOG=debug cargo run
```

## Trying it out

With the server running, in another terminal:

```bash
curl -i http://127.0.0.1:3000/health
```

Expected response (headers abbreviated):

```http
HTTP/1.1 200 OK
content-type: application/json

{"status":"ok","version":"0.1.0"}
```

Two behaviours you get for free from axum's router:

```bash
curl -i http://127.0.0.1:3000/nope                 # 404 Not Found
curl -i -X POST http://127.0.0.1:3000/health       # 405 Method Not Allowed
```

The full endpoint reference lives in [`docs/API.md`](docs/API.md).

## Development workflow

Run these before considering any change done:

```bash
cargo fmt --all -- --check                 # formatting
cargo clippy --all-targets -- -D warnings  # lints, warnings are errors
cargo test                                 # unit + integration + doc tests
cargo doc --no-deps --open                 # browse the API docs locally
```

Test layers in this repository:

| Layer         | Location            | What it covers                                  |
|---------------|---------------------|-------------------------------------------------|
| Unit tests    | `src/**` (`#[cfg(test)]`) | Pure logic, e.g. configuration parsing    |
| Integration   | `tests/`            | The whole `Router`, called in-process, no socket |
| Doc tests     | `///` and `//!` examples | Documentation examples stay compilable     |

## Project layout

```text
notes-api/
├── Cargo.toml            Dependencies, features, lints
├── README.md             You are here
├── docs/
│   ├── ARCHITECTURE.md   Layers, module & dependency graphs, request flow, decisions
│   ├── API.md            HTTP endpoint reference
│   └── ROADMAP.md        Milestones mapped to the chapter outline
├── src/
│   ├── main.rs           Composition root: logging, config, listener, shutdown
│   ├── lib.rs            Library root and module map
│   ├── config.rs         Environment-based configuration
│   └── routes/
│       ├── mod.rs        Assembles the application Router
│       └── health.rs     GET /health
└── tests/
    └── health.rs         In-process integration tests
```

The target layout for later milestones is described in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#planned-layout).

## Documentation index

| Document | Read it when you want to... |
|----------|-----------------------------|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | understand how the pieces fit and why |
| [`docs/API.md`](docs/API.md) | call the service |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | see what is built, what is next, and which chapter each milestone teaches |
| `cargo doc --open` | read the Rust API documentation |

## Troubleshooting

**`Address already in use`**
Another process owns port 3000. Pick a different address:
`NOTES_ADDR=127.0.0.1:3001 cargo run`.

**`NOTES_ADDR must be a socket address such as 127.0.0.1:3000`**
The value needs both an IP and a port, e.g. `0.0.0.0:8080`. A bare port or a
hostname is not accepted.

**Compiler complains about `edition = "2024"`**
Your toolchain is older than 1.85. Run `rustup update stable`.
