// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（A2-R6 批）：file_assemble_control 已认证+读会话无门禁的写路由补
//! `require_session` 门。
//!
//! 覆盖 12 把 handler（TSV 行 15-27）：
//!   attachment_upload_folder_folderId /
//!   attachment_upload_folder_folderId_callback_callback /
//!   attachment2_upload_folder_folderId / create_file / upload_file /
//!   u2_file_upload_octet_stream / u2_file_upload_callback /
//!   file_upload_with_url / create_file_entity / u2_folder_create /
//!   u2_share_save_to_folder / u2_file_upload_multipart
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

fn multipart_body() -> Body {
    Body::from(
        "--xboundary\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"a.txt\"\r\n\
         Content-Type: text/plain\r\n\r\nhello\r\n--xboundary--\r\n",
    )
}

/// 直调 handler（Json/Bytes/Path 族）：无会话必须 401，且发生在 pool.get() 之前。
#[tokio::test]
async fn r6_writes_reject_without_session() {
    let pool = dead_pool();
    let cases: Vec<(&str, Result<(), AppError>)> = vec![
        (
            "create_file",
            crate::create_file(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "n", "path": "p" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "upload_file",
            crate::upload_file(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "n", "path": "p" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "create_file_entity",
            crate::create_file_entity(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "n", "path": "p" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "file_upload_with_url",
            crate::file_upload_with_url(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "url": "https://example.com/a.txt" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u2_file_upload_octet_stream",
            crate::u2_file_upload_octet_stream(
                Extension(pool.clone()),
                None,
                axum::extract::Path(("taskReport".to_string(), "w-9".to_string(), "1".to_string())),
                axum::extract::Query(std::collections::HashMap::new()),
                axum::body::Bytes::from(vec![1u8, 2, 3]),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u2_folder_create",
            crate::u2_folder_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "n" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u2_share_save_to_folder",
            crate::u2_share_save_to_folder(
                Extension(pool),
                None,
                axum::extract::Path(("s-1".to_string(), "f-1".to_string(), "fd-1".to_string())),
            )
            .await
            .map(|_| ()),
        ),
    ];
    assert_eq!(cases.len(), 7, "本批可直调族 7 把必须逐把点名");
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

/// 路由层：匿名 POST/PUT 必须 401，而非 500/404。
#[tokio::test]
async fn r6_routes_reject_anonymous_writes() {
    let app = crate::router(dead_pool());
    let json = |uri: &str, method: &str, body: &str| {
        Request::builder()
            .uri(uri)
            .method(method)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    };
    let multipart = |uri: &str, method: &str| {
        Request::builder()
            .uri(uri)
            .method(method)
            .header("content-type", "multipart/form-data; boundary=xboundary")
            .body(multipart_body())
            .unwrap()
    };
    let cases: Vec<(&str, Request<Body>)> = vec![
        (
            "attachment_upload_folder_folderId",
            multipart("/api/attachment/upload/folder/fd-1", "POST"),
        ),
        (
            "attachment_upload_folder_folderId_callback_callback",
            multipart("/api/attachment/upload/folder/fd-1/callback/cb-1", "POST"),
        ),
        (
            "attachment2_upload_folder_folderId",
            multipart("/api/attachment2/upload/folder/fd-1", "POST"),
        ),
        (
            "create_file",
            json("/api/file/assemble/control/file/create", "POST", r#"{"name":"n"}"#),
        ),
        (
            "upload_file",
            json("/api/file/assemble/control/file/upload", "POST", r#"{"name":"n"}"#),
        ),
        (
            "u2_file_upload_octet_stream",
            Request::builder()
                .uri("/api/file/assemble/control/file/upload/referencetype/taskReport/reference/w-9/scale/1")
                .method("POST")
                .body(Body::from(vec![1u8, 2, 3]))
                .unwrap(),
        ),
        (
            "u2_file_upload_callback",
            multipart(
                "/api/file/assemble/control/file/upload/referencetype/taskReport/reference/w-9/scale/1/callback/cb-1",
                "POST",
            ),
        ),
        (
            "file_upload_with_url",
            json(
                "/api/file/assemble/control/file/upload/with/url",
                "POST",
                r#"{"url":"https://example.com/a.txt"}"#,
            ),
        ),
        (
            "create_file_entity",
            json("/api/file/core/entity/file/create", "POST", r#"{"name":"n"}"#),
        ),
        (
            "u2_folder_create/folder",
            json("/api/folder", "POST", r#"{"name":"n"}"#),
        ),
        (
            "u2_folder_create/folder2",
            json("/api/folder2", "POST", r#"{"name":"n"}"#),
        ),
        (
            "u2_share_save_to_folder",
            Request::builder()
                .uri("/api/share/share/s-1/file/f-1/folder/fd-1")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        ),
        (
            "u2_file_upload_multipart",
            multipart(
                "/api/file/assemble/control/file/upload/referencetype/taskReport/reference/w-9/scale/1",
                "PUT",
            ),
        ),
    ];
    assert_eq!(
        cases.len(),
        13,
        "本批 13 条写路由（含 folder/folder2 双挂载）必须逐条点名"
    );
    let mut leaked: Vec<String> = Vec::new();
    for (name, req) in cases {
        let status = app.clone().oneshot(req).await.unwrap().status();
        if status != StatusCode::UNAUTHORIZED {
            leaked.push(format!("{name} => 期望 401，实际 {status}"));
        }
    }
    assert!(
        leaked.is_empty(),
        "匿名写路由未回 401（出现 500 就是门插在读库之后，出现 400/415 是 body 未过提取器）：{leaked:?}"
    );
}
