use super::*;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        Config::new(),
        NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[tokio::test]
async fn test_query_list_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/list")
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
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/nonexistent")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_query_express_router_builds() {
    let pool = build_test_pool();
    let _ = crate::query_express_router(pool);
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
    assert!(result.message.is_none());
    assert_eq!(result.data, Some(0));
}

#[tokio::test]
async fn test_create_query_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"测试查询","queryType":"simple"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
        || response.status() == StatusCode::OK);
}