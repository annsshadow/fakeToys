// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use serde_json::json;
use shared::response::ActionResult;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/component_assemble_control/get/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_control_categories_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/component_assemble_control/list/control/categories")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
#[ignore = "handler requires DB connection, returns 500 with mock pool"]
async fn test_update_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req_body =
        serde_json::to_string(&json!({"enabled": true, "maxComponentCount": 200})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/component_assemble_control/update/control/config")
                .method(Method::GET)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_u2_post_component_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let req_body = serde_json::to_string(&json!({"name": "demo", "type": "system"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/component_assemble_control/component")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_u2_delete_component_by_id_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/component_assemble_control/component/test-id")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_u2_put_component_by_id_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let req_body = serde_json::to_string(&json!({"name": "demo", "type": "system"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/component_assemble_control/component/test-id")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

// list_control_categories N+1→GROUP BY collapse: the fold maps grouped
// (type, count) rows to category objects. system→"System Components",
// anything else→"Custom Components", enabled = count > 0.
#[test]
fn test_fold_control_categories_maps_names_and_enabled() {
    let cats = crate::fold_control_categories(vec![
        (Some("system".to_string()), 3i64),
        (Some("widget".to_string()), 1i64),
        (Some("empty".to_string()), 0i64),
    ]);
    assert_eq!(cats.len(), 3);
    assert_eq!(cats[0]["id"], json!("system"));
    assert_eq!(cats[0]["name"], json!("System Components"));
    assert_eq!(cats[0]["enabled"], json!(true));
    assert_eq!(cats[1]["name"], json!("Custom Components"));
    assert_eq!(cats[1]["enabled"], json!(true));
    assert_eq!(cats[2]["enabled"], json!(false));
}
