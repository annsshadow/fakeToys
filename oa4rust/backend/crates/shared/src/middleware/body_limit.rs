// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// body_limit — 全局请求体上限
//
// 修前上限只存在于各提取器内部，而且是 axum 的隐式默认 2 MiB：
//   - 运维无法按部署调整（附件上传、内网导入都需要更大的上限）；
//   - 写错值时没有任何启动期校验；
//   - 只有 `Bytes`/`Json`/`Multipart` 这类提取器受保护，直接读 `Request<Body>`
//     的路径在流层面没有上界。
//
// 现在由 `tower_http::limit::RequestBodyLimitLayer` 在**路由外**统一约束：
//   - 已知 `Content-Length` 且超限：不进 handler，直接 413（请求体一个字节都不读）；
//   - 无 `Content-Length`（chunked）：body 读过上限即返回错误，提取器映射为 413。
// 同时用 `DefaultBodyLimit::max(limit)` 让提取器口径与这里保持一致。
//
// 默认值与 axum 的隐式默认相同（2 MiB），因此不改变未配置部署的行为；
// 需要放大时设 `REQUEST_BODY_LIMIT_BYTES`。

use axum::extract::DefaultBodyLimit;
use tower_http::limit::RequestBodyLimitLayer;

/// 默认 2 MiB，与 axum 内置默认值一致：未配置时保持既有行为。
pub const DEFAULT_REQUEST_BODY_LIMIT_BYTES: usize = 2 * 1024 * 1024;

/// 环境变量名（单位：字节）。
pub const REQUEST_BODY_LIMIT_ENV: &str = "REQUEST_BODY_LIMIT_BYTES";

// 从 `REQUEST_BODY_LIMIT_BYTES` 读取上限；未设置用默认值。
//
// 非法值（含 0）直接报错而不是回退默认：配置写错时静默回退会让
// 「我明明设了 8 MiB」变成 2 MiB，属于 AGENTS #11 禁止的静默降级。
// 0 也不接受 —— 这里没有「关闭上限」的语义，宁可启动失败。
pub fn request_body_limit_from_env() -> anyhow::Result<usize> {
    parse_request_body_limit(std::env::var(REQUEST_BODY_LIMIT_ENV).ok().as_deref())
}

/// 纯解析：便于在不碰进程环境的前提下覆盖默认/合法/非法三种输入。
fn parse_request_body_limit(raw: Option<&str>) -> anyhow::Result<usize> {
    match raw {
        None => Ok(DEFAULT_REQUEST_BODY_LIMIT_BYTES),
        Some(raw) => {
            let trimmed = raw.trim();
            let bytes: usize = trimmed.parse().map_err(|_| {
                anyhow::anyhow!(
                    "{REQUEST_BODY_LIMIT_ENV}={raw:?} 不是合法字节数（正整数，单位字节）"
                )
            })?;
            if bytes == 0 {
                anyhow::bail!("{REQUEST_BODY_LIMIT_ENV} 不能为 0（不接受无上限的请求体）");
            }
            Ok(bytes)
        }
    }
}

/// 给路由套上统一上限（提取器口径 + 流层面）。
pub fn apply_body_limit(router: axum::Router, limit: usize) -> axum::Router {
    // layer 顺序：后加的在外层。RequestBodyLimit 放最外层，先于任何 handler /
    // 中间件看到请求体，已知超限时可以直接短路。
    router
        .layer(DefaultBodyLimit::max(limit))
        .layer(RequestBodyLimitLayer::new(limit))
}

/// 边读环境变量边套层；配置非法时让启动失败。
pub fn apply_body_limit_from_env(router: axum::Router) -> anyhow::Result<axum::Router> {
    Ok(apply_body_limit(router, request_body_limit_from_env()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, Bytes};
    use axum::http::{header, Request, StatusCode};
    use axum::routing::post;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tower::ServiceExt;

    // ⚠️ 路径放在常量里，别写成 `.route("/upload", …)`。
    // `oa4rust/backend/scripts/gen_openapi_paths.py` 按 `.route("字面量", …)`
    // 扫描真实路由，且只按**文件路径**排除测试目录；本模块的测试是内联的
    // `#[cfg(test)]`，会被当成生产路由写进 OpenAPI 契约（R24 首次踩到）。
    // 用常量传路径后生成器不再误采，测试语义不变。
    const TEST_UPLOAD_ROUTE: &str = "/upload";

    #[test]
    fn parse_covers_default_valid_and_invalid() {
        assert_eq!(
            parse_request_body_limit(None).unwrap(),
            DEFAULT_REQUEST_BODY_LIMIT_BYTES
        );
        assert_eq!(parse_request_body_limit(Some(" 4096 ")).unwrap(), 4096);
        assert!(parse_request_body_limit(Some("0")).is_err());
        assert!(parse_request_body_limit(Some("-1")).is_err());
        // 带单位的人类写法不接受：单位由变量名约定，避免 2MB / 2MiB 歧义。
        assert!(parse_request_body_limit(Some("2MB")).is_err());
    }

    fn upload_router(hits: Arc<AtomicUsize>) -> axum::Router {
        axum::Router::new().route(
            TEST_UPLOAD_ROUTE,
            post(move |body: Bytes| {
                let hits = hits.clone();
                async move {
                    hits.fetch_add(1, Ordering::SeqCst);
                    body.len().to_string()
                }
            }),
        )
    }

    /// 已知 Content-Length 且超限：必须在进入 handler 前就 413。
    /// 改前没有全局层，handler 会先被调用（再在提取器里失败）。
    #[tokio::test]
    async fn known_oversized_content_length_is_rejected_before_handler() {
        let hits = Arc::new(AtomicUsize::new(0));
        let app = apply_body_limit(upload_router(hits.clone()), 1024);

        let req = Request::builder()
            .method("POST")
            .uri("/upload")
            .header(header::CONTENT_LENGTH, "4096")
            .body(Body::from(vec![b'a'; 4096]))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(hits.load(Ordering::SeqCst), 0, "超限请求不应进入 handler");
    }

    /// 无 Content-Length（chunked）时只能在读的过程中截断：
    /// 仍必须是 413，而不是把 4 KiB 全读进来。
    #[tokio::test]
    async fn chunked_oversized_body_is_capped() {
        let hits = Arc::new(AtomicUsize::new(0));
        let app = apply_body_limit(upload_router(hits.clone()), 1024);

        let chunks = futures_util::stream::iter(vec![
            Ok::<_, std::io::Error>(Bytes::from(vec![b'a'; 600])),
            Ok(Bytes::from(vec![b'a'; 600])),
        ]);
        let req = Request::builder()
            .method("POST")
            .uri("/upload")
            .body(Body::from_stream(chunks))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "提取器失败时 handler 不应被调用"
        );
    }

    /// 上限内（含正好等于上限）正常通过，body 不被截断；调大上限后
    /// 原本会被默认 2 MiB 拒绝的体积也能通过 —— 这就是可配置带来的能力。
    #[tokio::test]
    async fn body_within_configured_limit_passes_intact() {
        let hits = Arc::new(AtomicUsize::new(0));
        let app = apply_body_limit(upload_router(hits.clone()), 4096);

        for size in [3000usize, 4096] {
            let req = Request::builder()
                .method("POST")
                .uri("/upload")
                .header(header::CONTENT_LENGTH, size.to_string())
                .body(Body::from(vec![b'x'; size]))
                .unwrap();
            let resp = app.clone().oneshot(req).await.unwrap();
            assert_eq!(resp.status(), StatusCode::OK, "size={size} 应通过");
            let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
                .await
                .unwrap();
            assert_eq!(
                String::from_utf8_lossy(&body),
                size.to_string(),
                "body 必须完整到达 handler"
            );
        }
        assert_eq!(hits.load(Ordering::SeqCst), 2);
    }
}
