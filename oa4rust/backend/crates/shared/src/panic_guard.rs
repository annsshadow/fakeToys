// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R11：handler panic → 500 信封，而不是掐断连接。
//!
//! # 为什么需要这一层
//!
//! `Cargo.toml` 的 release profile 里写着「保留默认 panic="unwind"：
//! Web 服务需 axum 捕获 handler panic 转 500」—— 但 **此前没有任何一层真的捕获**。
//! hyper 不会替 handler 兜底：handler panic 会沿 task 冒泡，连接被直接断开，
//! 客户端看到的是「空回复 / 连接重置」，服务端日志里除了一条 panic backtrace
//! 之外**没有任何请求上下文**（method/uri/request_id 都在更外层，拿不到）。
//!
//! 后果有两个，都不罕见：
//!   1. 客户端无法区分「服务端缺陷」与「网络断了」，也就无法决定是否重试；
//!   2. 单个越界/`unwrap` 就能让一个请求以最难排查的形态失败。
//!
//! 这一层把 panic 收敛成与 [`AppError::Internal`] 完全一致的 500 信封
//! （O2OA 前端与契约测试早已按这个形状断言），同时把 panic 文案记进日志。

use axum::body::Body;
use axum::response::{IntoResponse, Response};
use std::any::Any;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::error;

use crate::error::AppError;

/// 在路由层内层套上 panic 捕获。放置位置见 `create_app`：
/// 必须比请求超时层**更靠内**，panic 才不会先撞上超时/流式写失败。
pub fn apply_panic_guard(router: axum::Router) -> axum::Router {
    router.layer(CatchPanicLayer::custom(handle_panic))
}

fn handle_panic(payload: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let detail = panic_message(payload.as_ref());
    // panic! 的 payload 正常是 &str/String；`detail` 会被日志系统按字段记录，
    // 不做字符串拼接，避免换行注入。
    error!(panic = %detail, "handler panicked; converted to 500");
    AppError::Internal.into_response()
}

fn panic_message(payload: &(dyn Any + Send)) -> &str {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.as_str()
    } else {
        "non-string panic payload"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use tower::ServiceExt;

    fn request(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    /// 返回类型必须写明：`async { panic!(..) }` 的类型是 `!`，
    /// 会触发 never-type fallback 兼容性错误（edition 2024 起是硬错误）。
    async fn boom() -> &'static str {
        panic!("boom: index out of bounds")
    }

    /// panic 必须变成 500 信封（与 `AppError::Internal` 同形状），
    /// 而不是让连接断掉、客户端拿不到任何状态码。
    #[tokio::test]
    async fn panic_becomes_500_envelope() {
        let app = apply_panic_guard(axum::Router::new().route("/boom", get(boom)));

        let resp = app.oneshot(request("/boom")).await.unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "panic 必须映射 500"
        );

        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
            .await
            .expect("读取响应体失败");
        let v: serde_json::Value = serde_json::from_slice(&bytes).expect("响应体不是合法 JSON");
        assert_eq!(v["type"], "error", "错误信封 type 字段缺失：{v}");
        assert_eq!(v["message"], "internal server error");
        assert_eq!(
            v["prompt"],
            "com.x.base.core.project.exception.ExceptionInternal"
        );
    }

    /// 一次 panic 之后服务必须还能继续服务 —— 捕获层不能把任务/状态带崩。
    #[tokio::test]
    async fn service_recovers_after_panic() {
        let app = apply_panic_guard(
            axum::Router::new()
                .route("/boom", get(boom))
                .route("/ok", get(|| async { "ok" })),
        );

        let first = app.clone().oneshot(request("/boom")).await.unwrap();
        assert_eq!(first.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let second = app.oneshot(request("/ok")).await.unwrap();
        assert_eq!(second.status(), StatusCode::OK, "panic 之后的请求不应受影响");
        let bytes = axum::body::to_bytes(second.into_body(), 1 << 20)
            .await
            .unwrap();
        assert_eq!(&bytes[..], b"ok");
    }

    /// 正常请求必须原样透传（捕获层不能改写成功响应）。
    #[tokio::test]
    async fn normal_request_is_untouched() {
        let app = apply_panic_guard(
            axum::Router::new().route("/ok", get(|| async { "ok" })),
        );
        let resp = app.oneshot(request("/ok")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
