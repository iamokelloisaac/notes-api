//! # notes-api
//!
//! A small REST service for managing notes, built with [axum] 0.8.
//!
//! The crate is intentionally split into a **library** (this crate) and a thin
//! **binary** (`src/main.rs`):
//!
//! * the library owns everything that can be tested without a network socket:
//!   configuration parsing and the HTTP [`Router`](axum::Router);
//! * the binary is the *composition root*: it wires configuration, logging, the
//!   TCP listener and graceful shutdown around the library.
//!
//! This split is what lets `tests/` drive the whole application in-process.
//!
//! ## Module map
//!
//! | Module      | Responsibility                                            |
//! |-------------|-----------------------------------------------------------|
//! | [`config`]  | Read and validate runtime settings from the environment.  |
//! | [`routes`]  | Build the [`Router`](axum::Router) and own every handler. |
//!
//! Dependencies flow in one direction only: `main` → `routes` / `config`.
//! `config` and `routes` do not know about each other. See
//! `docs/ARCHITECTURE.md` for the full dependency graph.
//!
//! ## Quick start
//!
//! ```
//! use notes_api::{config::Config, routes};
//!
//! // Settings come from the environment; `default()` is what you get
//! // when nothing is set.
//! let config = Config::default();
//!
//! // `Router` is a `tower::Service`: hand it to `axum::serve` to run it,
//! // or call it directly in tests.
//! let app = routes::router();
//! # let _ = (config, app);
//! ```
//!
//! [axum]: https://docs.rs/axum

pub mod config;
pub mod routes;
