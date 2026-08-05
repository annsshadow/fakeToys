use axum::middleware;
use axum::routing::get;
use axum::Router;

use crate::middleware::{auth_middleware, rate_limit_middleware, trace_middleware};
use crate::rate_limit::RateLimiter;
use crate::session::SessionManager;

// ──────────────────────────────────────────────────────────────────────────────
// router
//
// 组装并返回当前服务的 Axum Router。
//
// 路由结构：
//   GET /health  → 健康检查端点，返回 "ok"
//
// 中间件层：
//   auth_middleware      → 认证（R12），豁免端点按精确路径匹配
//   rate_limit_middleware → 速率限制（R14），认证 10 次/分钟/IP、普通 100 次/分钟/IP
//   trace_middleware      → 记录请求日志并在 5xx 时输出 warning
//
// SessionManager/RateLimiter 由 main.rs 构造单一实例注入，
// 避免认证与限流状态分裂。
// ──────────────────────────────────────────────────────────────────────────────
pub fn router(session_manager: SessionManager, rate_limiter: RateLimiter) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(middleware::from_fn(trace_middleware))
        .layer(middleware::from_fn_with_state(
            rate_limiter,
            rate_limit_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            session_manager,
            auth_middleware,
        ))
}
