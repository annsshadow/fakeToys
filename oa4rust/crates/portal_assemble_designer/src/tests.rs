use super::*;
use serde_json::json;
use tower::util::ServiceExt;

#[test]
fn test_create_design_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "name": "Test Portal",
        "description": "A test portal design"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
    assert_eq!(json["data"]["name"], "Test Portal");
}

#[test]
fn test_get_design_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "design-1",
        "name": "Portal Design",
        "components": []
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "design-1");
}

#[test]
fn test_list_designs_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::java_success(
        json!([{"id": "design-1", "name": "Design 1"}]),
        1,
        0,
    );
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["count"], 1);
    assert_eq!(json["data"].as_array().unwrap().len(), 1);
}

#[test]
fn test_save_design_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "design-1",
        "saved": true,
        "updated_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["saved"], true);
}

#[tokio::test]
async fn test_create_design_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "name": "My Portal",
        "description": "A new portal"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/designer/create")
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
async fn test_get_design_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/designer/get/design-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_designs_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/designer/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_save_design_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "layout": "grid",
        "widgets": []
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/assemble/designer/save/design-1")
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
async fn test_design_list_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/design/list")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_design_get_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/portal/design/design-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_design_save_returns_error_without_db() {
    let app = portal_assemble_designer_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "id": "design-1",
        "content": {"layout": "grid", "widgets": []}
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
        .uri("/jaxrs/portal/design/save")
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
async fn test_u2_designer_gap_routes_exist() {
    let app = portal_assemble_designer_router();
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};

    let cases: Vec<(&str, &str)> = vec![
        ("POST", "/jaxrs/portal/assemble/designer/page"),
        ("GET", "/jaxrs/portal/assemble/designer/page/list/portal/p1"),
        ("DELETE", "/jaxrs/portal/assemble/designer/page/id1"),
        ("PUT", "/jaxrs/portal/assemble/designer/page/id1"),
        ("GET", "/jaxrs/portal/assemble/designer/pageversion/list/page/pid1"),
        ("POST", "/jaxrs/portal/assemble/designer/portal"),
        ("GET", "/jaxrs/portal/assemble/designer/portal/list/summary"),
        ("POST", "/jaxrs/portal/assemble/designer/portal/list/summary/v2"),
        ("DELETE", "/jaxrs/portal/assemble/designer/portal/id1"),
        ("PUT", "/jaxrs/portal/assemble/designer/portal/id1"),
        ("PUT", "/jaxrs/portal/assemble/designer/portal/id1/icon"),
        ("POST", "/jaxrs/portal/assemble/designer/portal/id1/permission"),
        ("POST", "/jaxrs/portal/assemble/designer/templatepage"),
        ("GET", "/jaxrs/portal/assemble/designer/templatepage/list"),
        ("GET", "/jaxrs/portal/assemble/designer/templatepage/list/category"),
        ("PUT", "/jaxrs/portal/assemble/designer/templatepage/list/category"),
        ("DELETE", "/jaxrs/portal/assemble/designer/templatepage/id1"),
        ("POST", "/jaxrs/portal/assemble/designer/widget"),
        ("DELETE", "/jaxrs/portal/assemble/designer/widget/id1"),
        ("PUT", "/jaxrs/portal/assemble/designer/widget/id1"),
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
