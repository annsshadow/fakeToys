use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tower::util::ServiceExt;

use crate::cms_express_router;

fn app() -> Router {
    cms_express_router()
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
    assert!(json["data"]["uuid"].is_string());
}

#[tokio::test]
async fn test_template_form_list_returns_data() {
    let response = app()
        .oneshot(Request::builder().uri("/jaxrs/cms/templateform/list").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
    assert!(json.get("data").is_some());
    assert!(json["data"]["count"].is_number());
    assert!(json["data"]["data"].is_array());
}
