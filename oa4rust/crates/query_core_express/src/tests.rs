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
fn test_execute_query_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "query": "SELECT * FROM test",
        "params": {},
        "timeout": 30000,
        "rowCount": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["query"], "SELECT * FROM test");
}

#[test]
fn test_get_query_history_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "limit": 10,
        "count": 1,
        "data": [{"id": "hist-1", "query": "SELECT * FROM test"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_cache_query_result_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "queryId": "query-1",
        "cached": true,
        "ttl": 3600
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["cached"], true);
}

#[test]
fn test_get_cache_status_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "queryId": "query-1",
        "cached": false,
        "hits": 0,
        "misses": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["queryId"], "query-1");
}

#[tokio::test]
async fn test_execute_query_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "query": "SELECT * FROM test",
        "params": {},
        "timeout": 30000
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/core/express/execute")
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
async fn test_get_query_history_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/core/express/history/10")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_cache_query_result_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "ttl": 3600
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/core/express/cache/query-1")
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
async fn test_get_cache_status_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/core/express/cache/status/query-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
