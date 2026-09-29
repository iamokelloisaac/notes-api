//! Liveness endpoint.
//!
//! `GET /health` answers "is the process up and able to serve HTTP?". It does
//! no I/O and touches no shared state, so it is safe for load balancers and
//! orchestrators to poll frequently.
//!
//! It also serves as the smallest complete example of an axum handler:
//!
//! 1. an `async fn` (Chapter 6.3) with no arguments (6.4),
//! 2. returning a type that implements
//!    [`IntoResponse`](axum::response::IntoResponse) (6.5),
//! 3. registered against a path *and* an HTTP method (Chapter 4.3).

use axum::{Json, Router, routing::get};
use serde::Serialize;

/// Body of a successful `GET /health` response.
///
/// Serialised as JSON, e.g. `{"status":"ok","version":"0.1.0"}`.
#[derive(Debug, Serialize)]
pub struct Health {
    /// Always `"ok"` while the process is able to answer requests.
    pub status: &'static str,
    /// The crate version the running binary was built from.
    pub version: &'static str,
}

/// Returns the router serving the health endpoint.
///
/// | Method | Path      | Handler  | Success        |
/// |--------|-----------|----------|----------------|
/// | `GET`  | `/health` | `health` | `200 OK` (JSON) |
///
/// Any other method on `/health` receives `405 Method Not Allowed`, which axum
/// produces automatically because the path exists but the method is not
/// registered.
pub fn router() -> Router {
    Router::new().route("/health", get(health))
}

/// Handler for `GET /health`.
///
/// Returning [`Json`] does three things for us: it serialises the value with
/// `serde_json`, sets `Content-Type: application/json`, and uses the default
/// status `200 OK`. If serialisation ever failed, axum would answer
/// `500 Internal Server Error` instead of panicking.
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}
