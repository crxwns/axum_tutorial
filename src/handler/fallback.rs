use axum::{extract::Request, http::StatusCode, response::IntoResponse};

pub async fn fallback(req: Request) -> impl IntoResponse {
    tracing::debug!("Called URI: {}", req.uri());
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
