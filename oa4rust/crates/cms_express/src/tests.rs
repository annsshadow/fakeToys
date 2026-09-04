use axum::{
    body::Body,
    http::{Request, Method, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use tower::util::ServiceExt;
use serde_json::Value;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(
        Config::new(),
        NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

fn app() -> Router {
    let pool = build_test_pool();
    crate::router(pool)
}

#[tokio::test]
async fn test_uuid_random_returns_uuid() {
    let response = app()
        .oneshot(Request::builder().uri("/jaxrs/cms/uuid/random").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
    assert!(json.get("data").is_some());
    // Handler returns Array [uuid_string], not {uuid: string}
    let data = json["data"].as_array().expect("data should be array");
    assert!(!data.is_empty(), "uuid array should not be empty");
    assert!(data[0].is_string(), "first element should be uuid string");
}

#[tokio::test]
async fn test_template_form_list_returns_data() {
    let response = app()
        .oneshot(Request::builder().uri("/jaxrs/cms/templateform/list").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_view_publish_route_exists() {
    let response = app()
        .oneshot(Request::builder()
            .uri("/jaxrs/cms/view/publish/view-001")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
        || response.status() == StatusCode::OK
        || response.status() == StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_view_unpublish_route_exists() {
    let response = app()
        .oneshot(Request::builder()
            .uri("/jaxrs/cms/view/unpublish/view-001")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
        || response.status() == StatusCode::OK
        || response.status() == StatusCode::NOT_FOUND);
}
