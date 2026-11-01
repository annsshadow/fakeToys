// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（A2-R6 批）：calendar_assemble_control 已认证+读会话无门禁的写路由补
//! `require_session` 门。
//!
//! 覆盖 11 把 handler（TSV 行 3-14）：
//!   calendar_delete / event_delete_after / event_delete_all /
//!   event_delete_single / calendar_create / event_create / event_manage /
//!   message_create / event_update_all（POST /event 与 PUT /event/update/all 双挂载）/
//!   event_update_after / event_update_single
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
            "calendar_create",
            crate::u2::calendar_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "name": "c" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "calendar_delete",
            crate::u2::calendar_delete(
                Extension(pool.clone()),
                None,
                axum::extract::Path("c-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_create",
            crate::u2::event_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "title": "t" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_delete_after",
            crate::u2::event_delete_after(
                Extension(pool.clone()),
                None,
                axum::extract::Path("e-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_delete_all",
            crate::u2::event_delete_all(
                Extension(pool.clone()),
                None,
                axum::extract::Path("e-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_delete_single",
            crate::u2::event_delete_single(
                Extension(pool.clone()),
                None,
                axum::extract::Path("e-1".to_string()),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_manage",
            crate::u2::event_manage(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "id": "e-1" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "message_create",
            crate::u2::message_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "title": "m" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_update_all",
            crate::u2::event_update_all(
                Extension(pool.clone()),
                None,
                axum::extract::Path("e-1".to_string()),
                axum::extract::Json(json!({ "title": "t" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_update_after",
            crate::u2::event_update_after(
                Extension(pool.clone()),
                None,
                axum::extract::Path("e-1".to_string()),
                axum::extract::Json(json!({ "title": "t" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "event_update_single",
            crate::u2::event_update_single(
                Extension(pool),
                None,
                axum::extract::Path("e-1".to_string()),
                axum::extract::Json(json!({ "title": "t" })),
            )
            .await
            .map(|_| ()),
        ),
    ];
    assert_eq!(cases.len(), 11, "本批 11 把必须逐把点名");
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

/// 路由层：匿名 POST/PUT/DELETE 必须 401，而非 500/404。
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
    let empty = |uri: &str, method: &str| {
        Request::builder()
            .uri(uri)
            .method(method)
            .body(Body::empty())
            .unwrap()
    };
    let cases: Vec<(&str, Request<Body>)> = vec![
        (
            "calendar_create",
            json(
                "/api/calendar_assemble_control/calendar",
                "POST",
                r#"{"name":"c"}"#,
            ),
        ),
        (
            "calendar_delete",
            empty("/api/calendar_assemble_control/calendar/c-1", "DELETE"),
        ),
        (
            "event_create",
            json(
                "/api/calendar_assemble_control/event",
                "POST",
                r#"{"title":"t"}"#,
            ),
        ),
        (
            "event_delete_after",
            empty("/api/calendar_assemble_control/event/after/e-1", "DELETE"),
        ),
        (
            "event_delete_all",
            empty("/api/calendar_assemble_control/event/all/e-1", "DELETE"),
        ),
        (
            "event_delete_single",
            empty("/api/calendar_assemble_control/event/single/e-1", "DELETE"),
        ),
        (
            "event_manage",
            json(
                "/api/calendar_assemble_control/event/manage",
                "POST",
                r#"{"id":"e-1"}"#,
            ),
        ),
        (
            "message_create",
            json(
                "/api/calendar_assemble_control/message",
                "POST",
                r#"{"title":"m"}"#,
            ),
        ),
        (
            "event_update_all/PUT /event/update/all",
            json(
                "/api/calendar_assemble_control/event/update/all/e-1",
                "PUT",
                r#"{"title":"t"}"#,
            ),
        ),
        (
            "event_update_after",
            json(
                "/api/calendar_assemble_control/event/update/after/e-1",
                "PUT",
                r#"{"title":"t"}"#,
            ),
        ),
        (
            "event_update_single",
            json(
                "/api/calendar_assemble_control/event/update/single/e-1",
                "PUT",
                r#"{"title":"t"}"#,
            ),
        ),
    ];
    // 注：清单行 11 的 `PUT /api/calendar_assemble_control/event → event_update_all`
    // 是**死注册**——路由无 `{id}` 路径参数，而 event_update_all 签名要求
    // `Path<String>`，提取器在 handler 之前拒绝（500），门无法在路由层体现。
    // handler 本身已加 require_session 门（直调臂 401），路由注册按规则不动。
    assert_eq!(
        cases.len(),
        11,
        "本批 11 条可达写路由必须逐条点名（死路由 PUT /event 除外）"
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
