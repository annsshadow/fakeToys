use axum::routing::get;
use axum::{middleware, Router};

use crate::middleware::trace_middleware;

// ──────────────────────────────────────────────────────────────────────────────
// router
//
// 组装并返回当前服务的 Axum Router。
//
// 路由结构：
//   GET /health  → 健康检查端点，返回 "ok"
//
// 中间件层：
//   trace_middleware  → 记录请求日志并在 5xx 时输出 warning
//
// 各二进制 crate 在 main 中调用此函数，再挂载数据库等 AppState 后启动。
// ──────────────────────────────────────────────────────────────────────────────
pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(middleware::from_fn(trace_middleware))
}
