use super::*;
use shared::response::ActionResult;
use serde_json::json;
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

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_control_config_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_control_sections_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/section/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_forums_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/forum/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_forum_view_all_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/forum/view/all")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_create_topic_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let body = serde_json::to_string(&json!({"forumId": "f1", "title": "Test", "content": "hello"})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/topic/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_subject_view_id_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/subject/view/sub-001")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_permission_section_section_id_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/permission/section/sec-001")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_shutup_create_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let body = serde_json::to_string(&json!({"person": "test"})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/shutup/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_uuid_generate_route() {
    let pool = build_test_pool();
    let app = crate::routes::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/bbs/assemble/control/uuid")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
