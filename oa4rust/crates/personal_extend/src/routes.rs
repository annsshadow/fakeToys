use axum::{Router, routing::get};

pub fn personal_extend_router() -> Router {
    Router::new().route("/health", get(|| async { "personal_extend: ok" }))
}
