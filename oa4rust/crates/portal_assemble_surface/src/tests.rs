use super::*;
use serde_json::json;
use tower::util::ServiceExt;

#[test]
fn test_get_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "name": "Portal Surface",
        "html": "<div></div>"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "surface-1");
}

#[test]
fn test_create_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "name": "My Surface",
        "template": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_list_surfaces_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "surface-1", "category": "default"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_preview_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "preview_url": "/preview/surface-1",
        "html": "<div>Preview</div>"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["preview_url"], "/preview/surface-1");
}

#[test]
fn test_publish_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "published": true,
        "published_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["published"], true);
}

#[tokio::test]
async fn test_get_surface_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/surface/get/surface-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_create_surface_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "name": "My Surface",
        "template": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/surface/create")
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
async fn test_list_surfaces_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/surface/list/default")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_preview_surface_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/surface/preview/surface-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_publish_surface_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/surface/publish/surface-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_surface_list_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/surface/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_surface_preview_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/surface/surface-1/preview")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_surface_publish_returns_error_without_db() {
    let app = portal_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "id": "surface-1"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
        .uri("/jaxrs/portal/surface/publish")
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
async fn test_u2_surface_gap_routes_exist() {
    let app = portal_assemble_surface_router();
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};

    let cases: Vec<(&str, &str)> = vec![
        ("GET", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1"),
        ("GET", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/data"),
        ("DELETE", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/x/data"),
        ("GET", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/x/data"),
        ("POST", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/x/data"),
        ("PUT", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/x/data"),
        ("GET", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/x/data/mockdeletetoget"),
        ("POST", "/jaxrs/portal/assemble/surface/dict/d1/portal/p1/x/data/mockputtopost"),
        ("GET", "/jaxrs/portal/assemble/surface/file/f1/portal/p1/content"),
        ("GET", "/jaxrs/portal/assemble/surface/file/f1/portal/p1/download"),
        ("GET", "/jaxrs/portal/assemble/surface/page/list/portal/p1"),
        ("GET", "/jaxrs/portal/assemble/surface/page/v2/id1"),
        ("GET", "/jaxrs/portal/assemble/surface/page/v2/id1/mobile"),
        ("GET", "/jaxrs/portal/assemble/surface/page/v2/f1/portal/p1"),
        ("GET", "/jaxrs/portal/assemble/surface/page/v2/f1/portal/p1/mobile"),
        ("GET", "/jaxrs/portal/assemble/surface/page/id1/mobile"),
        ("GET", "/jaxrs/portal/assemble/surface/page/f1/portal/p1"),
        ("GET", "/jaxrs/portal/assemble/surface/page/f1/portal/p1/mobile"),
        ("GET", "/jaxrs/portal/assemble/surface/portal/list/mobile"),
        ("GET", "/jaxrs/portal/assemble/surface/portal/f1/corner/mark"),
        ("GET", "/jaxrs/portal/assemble/surface/portal/id1/icon"),
        ("GET", "/jaxrs/portal/assemble/surface/portal/id1/icon/base64"),
        ("POST", "/jaxrs/portal/assemble/surface/script/portal/p1/name/n1"),
        ("GET", "/jaxrs/portal/assemble/surface/script/portal/p1/name/n1/imported"),
        ("GET", "/jaxrs/portal/assemble/surface/widget/w1/mobile"),
        ("GET", "/jaxrs/portal/assemble/surface/widget/f1/portal/p1"),
        ("GET", "/jaxrs/portal/assemble/surface/widget/f1/portal/p1/mobile"),
    ];

    for (m, uri) in cases {
        let method = Method::from_bytes(m.as_bytes()).unwrap();
        let body = if m == "GET" || m == "DELETE" {
            Body::empty()
        } else {
            Body::from("{}")
        };
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .body(body)
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_ne!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "route missing: {} {}",
            m,
            uri
        );
    }
}
