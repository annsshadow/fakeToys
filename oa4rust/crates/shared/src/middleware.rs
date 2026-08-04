use axum::middleware::Next;
use axum::body::Body;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use crate::error::AppError;
use tracing::warn;

// ──────────────────────────────────────────────────────────────────────────────
// trace_middleware
//
// 请求追踪中间件，记录每个请求的 HTTP 方法和 URI，
// 并在服务端返回 5xx 错误时额外输出 warning 级别日志，便于生产环境排查。
//
// 该中间件被挂载在 router() 顶层，对所有路由生效。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn trace_middleware(mut request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    // 仅对服务端错误（5xx）打 warning 日志，避免正常请求污染日志
    if response.status().is_server_error() {
        warn!(?method, ?uri, status = response.status().as_u16(), "server error");
    }

    response
}

// ──────────────────────────────────────────────────────────────────────────────
// error_handler
//
// 全局错误处理回调，接收 Axum 层抛出的 AppError，
// 并通过 IntoResponse 将其转换为标准化的 HTTP 响应。
//
// 在 Router 中通过 `.layer(axum::middleware::from_fn_with_error(error_handler))`
// 挂载后，所有未匹配的业务错误都会经过此函数统一处理。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn error_handler(err: AppError, _request: Request<Body>) -> Response {
    err.into_response()
}
