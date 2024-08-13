use axum::{routing::get, Router};
use handler::{fallback::fallback, root::root};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod handler;

#[tokio::main]
async fn main() {
    // Setup tracing
    // https://www.shuttle.rs/blog/2024/01/09/getting-started-tracing-rust
    let ef = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(ef).init();

    // Create Router
    let app = Router::new().route("/", get(root));
    let app = app.fallback(fallback);
    // Create TcpListener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Serve Router via the listener
    axum::serve(listener, app).await.unwrap();
}
