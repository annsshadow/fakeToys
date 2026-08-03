use axum::routing::get;
use axum::{middleware, Router};

use crate::middleware::trace_middleware;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(middleware::from_fn(trace_middleware))
}
