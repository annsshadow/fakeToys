use super::*;
use axum::body::Body;
use axum::http::{Request, Method};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use serde_json::json;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        Config::new(),
        NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

#[test]
fn test_get_status_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "status": "running",
        "version": "1.0.0",
        "uptime": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["status"], "running");
}

#[test]
fn test_get_logs_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "type": "error",
        "count": 1,
        "data": [{"level": "info", "message": "Log entry 1"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_send_message_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "sent": true,
        "token": "test-token",
        "message": "Hello"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["sent"], true);
}

#[test]
fn test_clear_cache_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "cleared": true,
        "type": "all",
        "cleared_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["cleared"], true);
}

#[test]
fn test_get_metric_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "name": "cpu_usage",
        "value": 42,
        "unit": "count"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["value"], 42);
}

#[tokio::test]
async fn test_get_status_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/console/status")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_logs_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/console/logs/error")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_send_message_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "token": "test-token",
        "message": "Hello world"
    })).unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/console/send/message")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_clear_cache_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/console/cache/clear/all")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_metric_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/console/metric/cpu_usage")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}
