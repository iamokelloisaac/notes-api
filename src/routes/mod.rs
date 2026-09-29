//! HTTP routing layer.
//!
//! Every feature owns a submodule exposing a `router()` function that returns
//! its own [`Router`]. This module's [`router`] merges them into the single
//! application router, so `main` and the tests only ever need one entry point.
//!
//! ```text
//! routes::router()
//!   └── health::router()   GET /health
//! ```
//!
//! # Why `router()` returns `Router` (and not `Router<S>`)
//!
//! `tower::ServiceExt::oneshot`, which our integration tests use to call the
//! application without a socket, is only available on a fully assembled
//! `Router<()>`. When shared state arrives in a later milestone, the
//! constructor will take that state as an argument and finish with
//! `.with_state(state)`, so callers still receive a `Router<()>`.

use axum::Router;

pub mod health;

/// Builds the complete application [`Router`].
///
/// The returned router is cheap to clone and implements
/// [`tower::Service`](https://docs.rs/tower/latest/tower/trait.Service.html),
/// so it can be passed straight to [`axum::serve`] or exercised in tests.
///
/// # Examples
///
/// ```
/// let app = notes_api::routes::router();
/// # let _ = app;
/// ```
pub fn router() -> Router {
    Router::new().merge(health::router())
}
