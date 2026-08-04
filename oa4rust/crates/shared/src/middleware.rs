use axum::middleware::Next;
use axum::body::Body;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use crate::error::AppError;
use tracing::warn;

pub async fn trace_middleware(mut request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    if response.status().is_server_error() {
        warn!(?method, ?uri, status = response.status().as_u16(), "server error");
    }

    response
}

pub async fn error_handler(err: AppError, _request: Request<Body>) -> Response {
    err.into_response()
}
