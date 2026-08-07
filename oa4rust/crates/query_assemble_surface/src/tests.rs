use super::*;
use serde_json::json;
use tower::util::ServiceExt;

#[test]
fn test_get_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "name": "Query Surface",
        "query": "",
        "template": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "surface-1");
}

#[test]
fn test_create_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "id": "surface-1",
        "name": "My Surface",
        "query": "select * from test",
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
fn test_save_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "saved": true,
        "name": "My Surface",
        "query": "select * from test",
        "template": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["saved"], true);
}

#[test]
fn test_delete_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "deleted": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["deleted"], true);
}

#[test]
fn test_preview_surface_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "surface-1",
        "preview_url": "/preview/query/surface-1",
        "html": "<div>Query Preview</div>"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["preview_url"], "/preview/query/surface-1");
}

#[tokio::test]
async fn test_get_surface_route_exists() {
    let app = query_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/get/surface-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_create_surface_route_exists() {
    let app = query_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "name": "My Surface",
        "query": "select * from test",
        "template": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/create")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_list_surfaces_route_exists() {
    let app = query_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/list/default")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_save_surface_route_exists() {
    let app = query_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "name": "My Surface",
        "query": "select * from test",
        "template": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/save/surface-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_delete_surface_route_exists() {
    let app = query_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/delete/surface-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_preview_surface_route_exists() {
    let app = query_assemble_surface_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/query/assemble/surface/preview/surface-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}
