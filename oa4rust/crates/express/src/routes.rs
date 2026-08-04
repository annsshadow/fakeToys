use axum::{Router, routing::get};

pub fn express_router() -> Router {
    Router::new().route("/health", get(|| async { "express: ok" }))
}
