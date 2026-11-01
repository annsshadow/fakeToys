// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（A2-R6 批）：program_center 已认证+读会话无门禁的写路由补
//! `require_session` 门。
//!
//! 覆盖 13 把 handler（TSV 行 58-71）：
//!   collect_add / collect_create / u3_config_change_password_post /
//!   dict_create / u3_mpweixin_template_send_post / u3_prompterrorlog_create /
//!   u3_qiyeweixin_create / script_create / u3_unexpectederrorlog_create /
//!   warnlog_create / u3_module_compare_upload_put / u3_module_list_put /
//!   u3_module_output_put
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
            "collect_add",
            crate::collect_add(
                Extension(pool.clone()),
                None,
                axum::extract::Json(
                    serde_json::from_value::<crate::CollectAddRequest>(
                        json!({ "title": "t", "url": "u" }),
                    )
                    .unwrap(),
                ),
            )
            .await
            .map(|_| ()),
        ),
        (
            "collect_create",
            crate::collect_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "title": "t" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u3_config_change_password_post",
            crate::u3_config_change_password_post(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({ "oldPassword": "o", "newPassword": "n" })),
            )
            .await
            .map(|_| ()),
        ),
        (
            "dict_create",
            crate::dict_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(
                    serde_json::from_value::<crate::DictCreateRequest>(json!({ "name": "d" }))
                        .unwrap(),
                ),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u3_mpweixin_template_send_post",
            crate::u3_mpweixin_template_send_post(
                Extension(pool.clone()),
                None,
                axum::extract::Json(json!({})),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u3_prompterrorlog_create",
            crate::u3_prompterrorlog_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(
                    serde_json::from_value::<crate::U3PromptErrorLogRequest>(
                        json!({ "message": "boom" }),
                    )
                    .unwrap(),
                ),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u3_unexpectederrorlog_create",
            crate::u3_unexpectederrorlog_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(
                    serde_json::from_value::<crate::U3UnexpectedErrorLogRequest>(
                        json!({ "message": "boom" }),
                    )
                    .unwrap(),
                ),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u3_qiyeweixin_create",
            crate::u3_qiyeweixin_create(Extension(pool.clone()), None)
                .await
                .map(|_| ()),
        ),
        (
            "script_create",
            crate::script_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(
                    serde_json::from_value::<crate::ScriptCreateRequest>(json!({ "name": "s" }))
                        .unwrap(),
                ),
            )
            .await
            .map(|_| ()),
        ),
        (
            "warnlog_create",
            crate::warnlog_create(
                Extension(pool.clone()),
                None,
                axum::extract::Json(
                    serde_json::from_value::<crate::WarnLogCreateRequest>(
                        json!({ "level": "WARN", "message": "m" }),
                    )
                    .unwrap(),
                ),
            )
            .await
            .map(|_| ()),
        ),
        (
            "u3_module_compare_upload_put",
            crate::u3_module_compare_upload_put(Extension(pool.clone()), None)
                .await
                .map(|_| ()),
        ),
        (
            "u3_module_list_put",
            crate::u3_module_list_put(Extension(pool.clone()), None)
                .await
                .map(|_| ()),
        ),
        (
            "u3_module_output_put",
            crate::u3_module_output_put(Extension(pool), None)
                .await
                .map(|_| ()),
        ),
    ];
    assert_eq!(cases.len(), 13, "本批 13 把必须逐把点名");
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
    let json = |uri: &str, body: &str| {
        Request::builder()
            .uri(uri)
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    };
    let json_put = |uri: &str, body: &str| {
        Request::builder()
            .uri(uri)
            .method("PUT")
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
            "collect_add",
            json("/api/program_center/collect", r#"{"title":"t"}"#),
        ),
        (
            "collect_add/add",
            json("/api/program_center/collect/add", r#"{"title":"t"}"#),
        ),
        (
            "collect_create",
            json("/api/program_center/collect/create", r#"{"title":"t"}"#),
        ),
        (
            "u3_config_change_password_post",
            json(
                "/api/program_center/config/change/password",
                r#"{"oldPassword":"o","newPassword":"n"}"#,
            ),
        ),
        (
            "dict_create",
            json("/api/program_center/dict", r#"{"name":"d"}"#),
        ),
        (
            "u3_mpweixin_template_send_post",
            json("/api/program_center/mpweixin/message/template/send", "{}"),
        ),
        (
            "u3_prompterrorlog_create",
            json(
                "/api/program_center/prompterrorlog",
                r#"{"message":"boom"}"#,
            ),
        ),
        (
            "u3_unexpectederrorlog_create",
            json(
                "/api/program_center/unexpectederrorlog",
                r#"{"message":"boom"}"#,
            ),
        ),
        (
            "u3_qiyeweixin_create",
            empty("/api/program_center/qiyeweixin", "POST"),
        ),
        (
            "script_create",
            json("/api/program_center/script", r#"{"name":"s"}"#),
        ),
        (
            "u3_module_compare_upload_put",
            json_put("/api/program_center/module/compare/upload", "{}"),
        ),
        (
            "u3_module_list_put",
            json_put("/api/program_center/module/list", "{}"),
        ),
        (
            "u3_module_output_put",
            json_put("/api/program_center/module/output", "{}"),
        ),
    ];
    assert_eq!(cases.len(), 13, "本批 13 把写路由必须逐条点名");
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
