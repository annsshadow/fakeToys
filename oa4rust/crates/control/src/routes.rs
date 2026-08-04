use axum::{Router, routing::get};

pub fn control_router() -> Router {
    Router::new().route("/health", get(|| async { "control: ok" }))
}
