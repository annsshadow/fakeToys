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
        "id": "q-1",
        "name": "Test Query",
        "queryType": "sql",
        "count": 10,
        "processed": true,
        "params": {}
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["processed"], true);
    assert_eq!(json["data"]["count"], 10);
}

#[test]
fn test_batch_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "total": 2,
        "results": [
            {"id": "q-1", "name": "Query 1", "queryType": "sql", "count": 5, "processed": true},
            {"id": "q-2", "name": "Query 2", "queryType": "rest", "count": 3, "processed": true}
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
        "activeConnections": 1,
        "queuedRequests": 0,
        "processedCount": 10
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["status"], "running");
    assert!(json["data"]["activeConnections"].is_number());
    assert!(json["data"]["queuedRequests"].is_number());
}

#[test]
fn test_reset_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "reset": true,
        "resetAt": "2024-06-01T00:00:00Z",
        "clearedCache": true,
        "processedCount": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["reset"], true);
    assert!(json["data"]["resetAt"].is_string());
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
