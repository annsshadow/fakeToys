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
fn test_link_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "linked": true,
        "source_type": "message",
        "source_id": "msg-1",
        "target_type": "process",
        "target_id": "proc-1"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["linked"], true);
}

#[test]
fn test_get_link_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "sourceType": "message",
        "sourceId": "msg-1",
        "targetType": "process",
        "targetId": "proc-1"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["sourceId"], "msg-1");
}

#[test]
fn test_list_links_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"sourceType": "message", "targetId": "proc-1"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_unlink_service_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "unlinked": true,
        "sourceType": "message",
        "sourceId": "msg-1",
        "targetType": "process",
        "targetId": "proc-1"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["unlinked"], true);
}

#[tokio::test]
async fn test_link_service_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "source_type": "message",
        "source_id": "msg-1",
        "target_type": "process",
        "target_id": "proc-1"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/link")
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
async fn test_get_link_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/link/message/msg-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_links_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/list/message")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_unlink_service_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/unlink/message/msg-1/process/proc-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
