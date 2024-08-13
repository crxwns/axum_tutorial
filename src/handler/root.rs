pub async fn root() -> &'static str {
    tracing::debug!("Called root handler.");
    "Hello, World!"
}
