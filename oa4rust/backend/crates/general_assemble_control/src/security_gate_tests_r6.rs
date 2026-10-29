// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（A2-R6 批）：general_assemble_control 已认证+读会话无门禁的写路由补
//! `require_session` 门。
//!
//! 覆盖 8 把 handler（TSV 行 28-35）：
//!   excel_upload / excel_upload_with_url / generalfile_create /
//!   invoice_create / invoice_upload / invoice_upload_for_create /
//!   invoice_upload_with_url / qrcode_create
//! 判据：无会话（路由层不注入 Session 扩展）→ 401，门在首个 DB 访问之前，
//! 死池离线可证（旧行为：门缺失时 pool.get() 失败 → 500）。

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use serde_json::json;
use shared::error::AppError;
use tower::ServiceExt;

fn dead_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
    Pool::builder(mgr).max_size(1).build().unwrap()
}

/// 直调 handler：无会话必须 401，且发生在 pool.get() 之前。
#[tokio::test]
async fn r6_writes_reject_without_session() {
    let pool = dead_pool();
    let cases: Vec<(&str, Result<(), AppError>)> = vec![
        (
            "excel_upload",
            crate::excel_upload(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "e" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "excel_upload_with_url",
            crate::excel_upload_with_url(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "e" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "generalfile_create",
            crate::generalfile_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "g", "content": "c" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "invoice_create",
            crate::invoice_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "i" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "invoice_upload",
            crate::invoice_upload(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "i" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "invoice_upload_for_create",
            crate::invoice_upload_for_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "i" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "invoice_upload_with_url",
            crate::invoice_upload_with_url(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "i" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "qrcode_create",
            crate::qrcode_create(
                Extension(pool),
                None,
                axum::extract::Json(json!({ "text": "q" })),
            )
            .await
            .map(|_| ()),
        ),
    ];
    assert_eq!(cases.len(), 8, "本批 8 把必须逐把点名");
    let leaked: Vec<String> = cases
        .iter()
        .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
        .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
        .collect();
    assert!(
        leaked.is_empty(),
        "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
    );
}

/// 路由层：匿名 POST 必须 401，而非 500/404。
#[tokio::test]
async fn r6_routes_reject_anonymous_writes() {
    let app = crate::router(dead_pool());
    let json = |uri: &str, body: &str| {
        Request::builder()
            .uri(uri)
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    };
    let cases: Vec<(&str, Request<Body>)> = vec![
        (
            "excel_upload",
            json("/api/general/assemble/control/excel/upload", r#"{"name":"e"}"#),
        ),
        (
            "excel_upload_with_url",
            json(
                "/api/general/assemble/control/excel/upload/with/url",
                r#"{"name":"e"}"#,
            ),
        ),
        (
            "generalfile_create",
            json(
                "/api/general/assemble/control/generalfile",
                r#"{"name":"g","content":"c"}"#,
            ),
        ),
        (
            "invoice_create",
            json(
                "/api/general/assemble/control/invoice/create",
                r#"{"name":"i"}"#,
            ),
        ),
        (
            "invoice_upload",
            json(
                "/api/general/assemble/control/invoice/upload",
                r#"{"name":"i"}"#,
            ),
        ),
        (
            "invoice_upload_for_create",
            json(
                "/api/general/assemble/control/invoice/upload/for/create",
                r#"{"name":"i"}"#,
            ),
        ),
        (
            "invoice_upload_with_url",
            json(
                "/api/general/assemble/control/invoice/upload/with/url",
                r#"{"name":"i"}"#,
            ),
        ),
        (
            "qrcode_create",
            json("/api/general/assemble/control/qrcode", r#"{"text":"q"}"#),
        ),
    ];
    assert_eq!(cases.len(), 8, "本批 8 条写路由必须逐条点名");
    let mut leaked: Vec<String> = Vec::new();
    for (name, req) in cases {
        let status = app
            .clone()
            .oneshot(req)
            .await
            .unwrap()
            .status();
        if status != StatusCode::UNAUTHORIZED {
            leaked.push(format!("{name} => 期望 401，实际 {status}"));
        }
    }
    assert!(
        leaked.is_empty(),
        "匿名写路由未回 401（出现 500 就是门插在读库之后，出现 400/415 是 body 未过提取器）：{leaked:?}"
    );
}