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
fn test_send_message_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "sent": true,
        "from": "user1",
        "to": "user2"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["sent"], true);
}

#[test]
fn test_receive_list_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "msg-1", "status": "unread"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_mark_read_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "msg-1",
        "marked_read": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["marked_read"], true);
}

#[tokio::test]
async fn test_send_message_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "from": "sender",
        "to": "receiver",
        "content": "hello"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/message/assemble/communicate/send")
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
async fn test_receive_list_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/message/assemble/communicate/receive/consumer1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_mark_read_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/message/assemble/communicate/mark_read/msg-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
