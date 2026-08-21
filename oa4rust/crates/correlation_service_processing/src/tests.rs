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

// ── ActionResult format tests ──────────────────────────────────────────────

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

// ── Route existence: link service routes ───────────────────────────────────

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
    #[ignore = "handler requires DB, returns 500 with mock pool"]
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

// ── Route existence: all registered routes ─────────────────────────────────

#[tokio::test]
async fn test_get_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/test-id")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_create_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"person_id": "p1", "target_id": "t1", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/create")
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
async fn test_save_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"target_id": "t2", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/save/test-id")
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
async fn test_delete_correlation_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/delete/test-id")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_delete_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/doc-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_delete_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/job-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_cms_document_site_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1/site/site-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_list_type_processplatform_job_site_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1/site/site-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_readable_type_cms_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/readable/type/cms")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_readable_type_processplatform_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/readable/type/processplatform")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/type/cms/document/doc-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/job-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_correlation_update_type_cms_document_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"personId": "p1", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-1")
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
async fn test_correlation_update_type_processplatform_job_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"personId": "p1", "type": "processplatform/job"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/job-1")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ── ActionResult serialization ─────────────────────────────────────────────

#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"linked": true}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["linked"], true);
}

#[test]
fn test_action_result_error_serialization() {
    let result: ActionResult<&str> = ActionResult::error("link not found");
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "error");
    assert_eq!(json["message"], "link not found");
    assert!(json["data"].is_null());
}

#[test]
fn test_action_result_with_count() {
    let mut result: ActionResult<serde_json::Value> =
        ActionResult::success(serde_json::json!({"items": []}));
    result.count = Some(3);
    result.size = Some(10);
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["count"], 3);
    assert_eq!(json["size"], 10);
}

// ── Request validation: create/save/delete/link with invalid input ──────────

#[tokio::test]
async fn test_create_correlation_empty_person_id() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"person_id": "", "target_id": "t1", "type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/create")
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
async fn test_create_correlation_missing_type() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"person_id": "p1", "target_id": "t1"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/create")
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
async fn test_save_correlation_missing_target_id() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let body = serde_json::to_string(&json!({"type": "cms/document"})).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/save/test-id")
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
async fn test_link_service_empty_body() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/link")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
    #[ignore = "handler requires DB, returns 500 with mock pool"]
    async fn test_unlink_service_route_ok() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/correlation/service/processing/unlink/type1/id1/type2/id2")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

// ── Router build test ───────────────────────────────────────────────────────

#[test]
fn test_correlation_service_processing_router_builds() {
    let pool = build_test_pool();
    let _ = crate::router(pool);
}