//! Binary entry point and *composition root*.
//!
//! This file contains no business logic. Its only job is to assemble the
//! pieces provided by the `notes_api` library and run them:
//!
//! ```text
//! tracing  ->  Config  ->  TcpListener  ->  axum::serve(Router)  ->  shutdown
//! ```
//!
//! Where the layers from Chapter 1 show up here:
//!
//! * **Tokio** (1.4) runs this `async fn main` and owns the listening socket.
//! * **Hyper** (1.5) is driven by [`axum::serve`]: it accepts connections and
//!   parses HTTP.
//! * **axum** (1.1) supplies the [`Router`](axum::Router) that decides which
//!   handler answers each request.

use notes_api::{config::Config, routes};
use tokio::{net::TcpListener, signal};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    // Fail fast: a bad NOTES_ADDR should stop us before we accept traffic.
    let config = Config::from_env()?;

    // Binding is separate from serving so that we can log the *actual* address
    // (relevant when the configured port is 0 and the OS picks one).
    let listener = TcpListener::bind(config.addr).await?;
    tracing::info!(addr = %listener.local_addr()?, "listening");

    axum::serve(listener, routes::router())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("server stopped");
    Ok(())
}

/// Installs the global `tracing` subscriber.
///
/// Verbosity is controlled by the standard `RUST_LOG` variable. When it is not
/// set we log this crate at `debug` and everything else at `info`.
fn init_tracing() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("notes_api=debug,info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Resolves when the process is asked to stop.
///
/// Listens for `Ctrl+C` everywhere and additionally for `SIGTERM` on Unix,
/// which is what container runtimes and process managers send. Once this
/// future completes, [`axum::serve`] stops accepting new connections and
/// waits for in-flight requests to finish.
///
/// # Panics
///
/// Panics if the operating system refuses to install a signal handler. That
/// only happens at start-up under exotic conditions, and continuing without
/// a way to shut down cleanly would be worse than aborting.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    // Non-Unix platforms have no SIGTERM: use a future that never completes so
    // that `select!` below only ever resolves via Ctrl+C.
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
