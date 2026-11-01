// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::routing::get;
use axum::Router;

// ──────────────────────────────────────────────────────────────────────────────
// router
//
// 提供当前服务的业务路由（无中间件）。
//
// 路由结构：
//   GET /health  → 健康检查端点，返回 "ok"
//
// 认证/授权/限流/安全头中间件由 main.rs 在顶层统一挂载一次
// （SecurityState 单一实例），确保所有 /api/* 路由都经过认证，
// 避免各 crate router 各自挂载造成的重复或遗漏。
// ──────────────────────────────────────────────────────────────────────────────
pub fn router() -> Router {
    Router::new().route("/health", get(|| async { "ok" }))
}

#[cfg(test)]
mod tests {
    use super::router;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_route_returns_ok() {
        let app = router();
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = to_bytes(res.into_body(), 4096).await.unwrap();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn unknown_route_returns_404() {
        let app = router();
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/nope")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
