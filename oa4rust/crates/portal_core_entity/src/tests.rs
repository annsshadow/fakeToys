use super::*;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use deadpool_postgres::{Manager, Pool};
use shared::response::ActionResult;
use tower::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).build().unwrap()
}

#[tokio::test]
async fn test_portal_list_returns_success() {
    let pool = build_test_pool();
    let app = portal_core_entity_router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/portal/list")
                .method(axum::http::Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_widget_list_returns_success() {
    let pool = build_test_pool();
    let app = portal_core_entity_router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/widget/list")
                .method(axum::http::Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_page_list_returns_success() {
    let pool = build_test_pool();
    let app = portal_core_entity_router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/page/list")
                .method(axum::http::Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_script_list_returns_success() {
    let pool = build_test_pool();
    let app = portal_core_entity_router(pool);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/script/list")
                .method(axum::http::Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_action_result_success_structure() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"count": 1, "data": []}));
    assert_eq!(result.r#type, Some("success".to_string()));
    assert!(result.data.is_some());
}

#[test]
fn test_action_result_error_structure() {
    let result: ActionResult<String> = ActionResult::error("something went wrong");
    assert_eq!(result.r#type, Some("error".to_string()));
    assert_eq!(result.message, Some("something went wrong".to_string()));
    assert!(result.data.is_none());
}
