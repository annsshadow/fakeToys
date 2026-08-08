use super::*;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
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
fn test_process_query_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "queryType": "sql",
        "params": {},
        "processed": true,
        "resultCount": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["processed"], true);
}

#[test]
fn test_batch_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "total": 2,
        "results": [
            {"queryType": "sql", "processed": true},
            {"queryType": "rest", "processed": true}
        ]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["total"], 2);
}

#[test]
fn test_get_service_status_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "status": "running",
        "activeConnections": 0,
        "queuedRequests": 0,
        "processedCount": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["status"], "running");
}

#[test]
fn test_reset_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "reset": true,
        "resetAt": "2024-01-01T00:00:00Z",
        "clearedCache": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["reset"], true);
}

#[tokio::test]
async fn test_process_query_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "query_type": "sql",
        "params": {},
        "options": {}
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/process")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_batch_process_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "queries": [
            {"query_type": "sql", "params": {}},
            {"query_type": "rest", "params": {}}
        ]
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/batch")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_service_status_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/status")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_reset_service_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/service/processing/reset")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
