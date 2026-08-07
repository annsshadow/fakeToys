use super::*;
use serde_json::json;
use tower::util::ServiceExt;

#[test]
fn test_get_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "proc-1",
        "name": "Process",
        "status": "active"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "proc-1");
}

#[test]
fn test_create_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "name": "My Process",
        "description": "A process",
        "category": "default"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_list_processes_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "proc-1", "category": "default"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_execute_process_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "proc-1",
        "executed": true,
        "execution_id": "exec-1",
        "status": "running"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["executed"], true);
}

#[test]
fn test_get_process_instance_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "executionId": "exec-1",
        "status": "running",
        "current_node": "start"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["executionId"], "exec-1");
}

#[test]
fn test_cancel_process_instance_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "executionId": "exec-1",
        "cancelled": true,
        "cancelled_at": "2024-01-01T00:00:00Z"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["cancelled"], true);
}

#[tokio::test]
async fn test_get_process_route_exists() {
    let app = processplatform_service_processing_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/get/proc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_create_process_route_exists() {
    let app = processplatform_service_processing_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let req = serde_json::to_string(&json!({
        "name": "My Process",
        "description": "A new process",
        "category": "default"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/create")
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
async fn test_list_processes_route_exists() {
    let app = processplatform_service_processing_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/list/default")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_execute_process_route_exists() {
    let app = processplatform_service_processing_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/execute/proc-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_get_process_instance_route_exists() {
    let app = processplatform_service_processing_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/instance/exec-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_cancel_process_instance_route_exists() {
    let app = processplatform_service_processing_router();

    use axum::body::Body;
    use axum::http::{Request, Method};

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/service/processing/cancel/exec-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}
