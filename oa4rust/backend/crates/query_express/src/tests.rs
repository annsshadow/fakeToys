// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use tower::util::ServiceExt;

/// O9：router 改注 sea_orm DatabaseConnection（修复 DatabaseConnection 从未注入、
/// /api/query/* 生产恒定 500 的接线缺陷）；路由存在性测试用不可达懒连接即可。
async fn build_test_db() -> sea_orm::DatabaseConnection {
    shared::testing::mock_db().await
}

#[tokio::test]
async fn test_query_list_route_exists() {
    let app = crate::router(build_test_db().await);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_unknown_route_returns_404() {
    let app = crate::router(build_test_db().await);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/nonexistent")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_query_express_router_builds() {
    let _ = crate::query_express_router(build_test_db().await);
}

// ── ActionResult serialization ─────────────────────────────────────────────

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 2);
}

#[test]
fn test_action_result_error_serialization() {
    let result: ActionResult<&str> = ActionResult::error("query failed");
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "error");
    assert_eq!(json["message"], "query failed");
    assert!(json["data"].is_null());
}

#[test]
fn test_action_result_success_with_null_message() {
    let result: ActionResult<i32> = ActionResult::success(0);
    assert_eq!(result.r#type, Some("success".to_string()));
    // o2server 成功信封 message 恒为空串（2026-08-25 行为对比结论）
    assert_eq!(result.message, Some(String::new()));
    assert_eq!(result.data, Some(0));
}

#[tokio::test]
async fn test_create_query_route_exists() {
    let app = crate::router(build_test_db().await);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/query/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"测试查询","queryType":"simple"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    // O9 接线修复后：扩展注入成功，无会话 → 401（会话门先于 DB 访问）；
    // 带会话且 DB 不可达 → 500。路由缺失则是 404。
    assert!(
        response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::OK
            || response.status() == StatusCode::UNAUTHORIZED
    );
}
