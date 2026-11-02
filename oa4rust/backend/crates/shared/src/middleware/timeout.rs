// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// timeout — 全局请求超时
//
// 修前整个 Router 没有任何请求上界：一条卡住的 SQL（未设 statement_timeout）、
// 一次拿不到连接的 `pool.get()`、或一个死循环 handler，都会让请求无限挂着。
// 客户端的表现是「转圈直到断网」，服务端的表现是「连接被慢慢耗尽」，
// 而日志里只有一条迟迟不结束的访问日志。
//
// 现在由 `tower_http::timeout::TimeoutLayer` 在 handler future 上加上界：
// 超时返回 408 Request Timeout，客户端可以据此重试，连接立即归还。
//
// 注意边界：该层只等「响应头产生」，不等响应体。SSE / 长轮询这类
// 立即返回流式 body 的接口不受影响（有测试盯着这一点）。

use std::time::Duration;

use tower_http::timeout::TimeoutLayer;

/// 默认 30s：覆盖现有最慢的管理导出（10 万行）与看板聚合，
/// 又能保证「写坏的查询」不会永久占住请求。
pub const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 30;

/// 环境变量名；`0` 显式关闭（例如排查问题需要观察长事务）。
pub const REQUEST_TIMEOUT_ENV: &str = "REQUEST_TIMEOUT_SECONDS";

// 从 `REQUEST_TIMEOUT_SECONDS` 读取超时；未设置用默认值，`0` 表示关闭。
//
// 非法值直接报错而不是回退默认：配置写错时静默回退会让「我明明设了 5s」
// 变成 30s，属于 AGENTS #11 明令禁止的静默降级。
pub fn request_timeout_from_env() -> anyhow::Result<Option<Duration>> {
    parse_request_timeout(std::env::var(REQUEST_TIMEOUT_ENV).ok().as_deref())
}

/// 纯解析：便于在不碰进程环境的前提下覆盖非法值/关闭/默认三种输入。
fn parse_request_timeout(raw: Option<&str>) -> anyhow::Result<Option<Duration>> {
    match raw {
        None => Ok(Some(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_SECONDS))),
        Some(raw) => {
            let trimmed = raw.trim();
            let secs: u64 = trimmed.parse().map_err(|_| {
                anyhow::anyhow!(
                    "{REQUEST_TIMEOUT_ENV}={raw:?} 不是合法秒数（整数，0 表示关闭）"
                )
            })?;
            Ok(if secs == 0 {
                None
            } else {
                Some(Duration::from_secs(secs))
            })
        }
    }
}

/// 便于调用点统一写法：有值就套层，`None` 原样返回。
pub fn apply_request_timeout(
    router: axum::Router,
    timeout: Option<Duration>,
) -> axum::Router {
    match timeout {
        // 408 而不是 500：语义是「服务端等不了」，客户端可以安全重试。
        Some(d) => router.layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            d,
        )),
        None => router,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use std::time::Instant;
    use tower::ServiceExt;

    fn get_uri(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    /// 默认值存在、`0` 关闭、非法值报错——三种输入都要有确定行为。
    #[test]
    fn parse_covers_default_disabled_and_invalid() {
        assert_eq!(
            parse_request_timeout(None).unwrap(),
            Some(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_SECONDS))
        );
        assert_eq!(parse_request_timeout(Some("0")).unwrap(), None);
        assert_eq!(
            parse_request_timeout(Some(" 5 ")).unwrap(),
            Some(Duration::from_secs(5))
        );
        assert!(parse_request_timeout(Some("5s")).is_err());
        assert!(parse_request_timeout(Some("-1")).is_err());
    }

    /// 卡住的 handler 必须在上界处被切断，而不是拖到客户端超时。
    #[tokio::test]
    async fn slow_handler_is_cut_off_with_408() {
        let app = apply_request_timeout(
            axum::Router::new().route(
                "/slow",
                get(|| async {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    "late"
                }),
            ),
            Some(Duration::from_millis(50)),
        );

        let started = Instant::now();
        let resp = app.oneshot(get_uri("/slow")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
        assert!(
            started.elapsed() < Duration::from_millis(300),
            "未在超时上界切断：{:?}",
            started.elapsed()
        );
    }

    #[tokio::test]
    async fn fast_handler_passes_through() {
        let app = apply_request_timeout(
            axum::Router::new().route("/fast", get(|| async { "ok" })),
            Some(Duration::from_millis(50)),
        );
        let resp = app.oneshot(get_uri("/fast")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"ok");
    }

    /// 超时层只约束「响应头产生」，不能截断流式 body：
    /// SSE/长轮询的响应头是立刻返回的，body 可能在超时之后才吐完。
    #[tokio::test]
    async fn streamed_body_outlives_timeout() {
        let app = apply_request_timeout(
            axum::Router::new().route(
                "/sse",
                get(|| async {
                    let stream = futures_util::stream::once(async {
                        tokio::time::sleep(Duration::from_millis(200)).await;
                        Ok::<_, std::io::Error>(axum::body::Bytes::from_static(b"data: ok\n\n"))
                    });
                    Body::from_stream(stream)
                }),
            ),
            Some(Duration::from_millis(50)),
        );

        let resp = app.oneshot(get_uri("/sse")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"data: ok\n\n", "流式 body 被超时层截断了");
    }

    /// 关闭超时后，慢 handler 应能跑完（排查长事务时的逃逸口）。
    #[tokio::test]
    async fn disabled_timeout_is_passthrough() {
        let app = apply_request_timeout(
            axum::Router::new().route(
                "/slow",
                get(|| async {
                    tokio::time::sleep(Duration::from_millis(120)).await;
                    "done"
                }),
            ),
            None,
        );
        let resp = app.oneshot(get_uri("/slow")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
