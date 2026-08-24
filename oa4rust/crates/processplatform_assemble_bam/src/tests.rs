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
fn test_get_bam_config_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "bam-1",
        "name": "BAM Config",
        "enabled": true,
        "definition": ""
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["id"], "bam-1");
}

#[test]
fn test_create_bam_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "created": true,
        "id": "bam-1",
        "name": "My BAM",
        "definition": "process-def"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["created"], true);
}

#[test]
fn test_list_bams_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "bam-1", "category": "processplatform"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_delete_bam_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "bam-1",
        "deleted": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["deleted"], true);
}

#[test]
fn test_get_bam_status_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "bam-1",
        "status": "running",
        "activeMetrics": 0
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["status"], "running");
}

#[tokio::test]
async fn test_get_bam_config_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/bam/get/bam-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_create_bam_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "name": "My BAM",
        "definition": "process-def"
    })).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/bam/create")
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
async fn test_list_bams_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/bam/list/processplatform")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_delete_bam_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/bam/delete/bam-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_bam_status_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/processplatform/assemble/bam/status/bam-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 新增：Java 精确路径闭合（42 端点）的测试
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn test_period_predicate_completed_semantics() {
    // 口径：completed = 状态已完成
    assert_eq!(period_predicate("task", "completed"), "t.task_status = 'completed'");
    assert_eq!(period_predicate("work", "completed"), "w.work_status = 'completed'");
}

#[test]
fn test_period_predicate_expired_requires_overdue_and_unfinished() {
    // 口径：expired 必须同时满足"有截止时间、已过期、未完成"，防止把未到期的也算超时
    let p = period_predicate("task", "expired");
    assert!(p.contains("t.end_time IS NOT NULL"), "expired 必须要求 end_time 非空");
    assert!(p.contains("t.end_time < NOW()"), "expired 必须要求已过截止时间");
    assert!(p.contains("IS DISTINCT FROM 'completed'"), "expired 必须排除已完成");
}

#[test]
fn test_period_predicate_start_means_started_not_finished() {
    let p = period_predicate("work", "start");
    assert!(p.contains("w.start_time IS NOT NULL"));
    assert!(p.contains("IS DISTINCT FROM 'completed'"));
}

async fn bam_u2_route_status(method: Method, uri: &str) -> axum::http::StatusCode {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri(uri)
                .method(method)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    response.status()
}

#[tokio::test]
async fn test_bam_stubs_completed_task_applicationstubs_registered() {
    let status = bam_u2_route_status(Method::GET, "/jaxrs/processplatform/assemble/bam/period/list/completed/task/applicationstubs").await;
    assert_ne!(status, StatusCode::NOT_FOUND, "applicationstubs 桩端点应注册为 Java 精确路径");
}

#[tokio::test]
async fn test_bam_count_completed_task_by_unit_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/app1/process/p1/activity/a1/by/unit",
    )
    .await;
    assert_ne!(status, StatusCode::NOT_FOUND, "count...by/unit 精确路径应注册");
}

#[tokio::test]
async fn test_bam_count_start_work_total_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/app1/process/p1/unit/u1/person/per1",
    )
    .await;
    assert_ne!(status, StatusCode::NOT_FOUND, "start/work 总数切片应注册且动词为 GET");
}

#[tokio::test]
async fn test_bam_state_category_exact_path_registered() {
    let status = bam_u2_route_status(Method::GET, "/jaxrs/processplatform/assemble/bam/state/category").await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_bam_state_category_trigger_all_registered() {
    let status = bam_u2_route_status(Method::GET, "/jaxrs/processplatform/assemble/bam/state/category/trigger").await;
    assert_ne!(status, StatusCode::NOT_FOUND, "/state/category/trigger 应为无参 GET");
}

#[tokio::test]
async fn test_bam_state_applicationtstubs_trigger_registered() {
    let status = bam_u2_route_status(Method::GET, "/jaxrs/processplatform/assemble/bam/state/applicationtstubs/trigger").await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_bam_count_endpoint_rejects_wrong_verb() {
    // Java 清单中 count 切片是 GET；POST 不应命中同一路径（防止动词漂移回归）
    let status = bam_u2_route_status(
        Method::POST,
        "/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/app1/process/p1/unit/u1/person/per1",
    )
    .await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED, "GET-only 端点对 POST 应返回 405 而非命中处理");
}

#[test]
fn test_period_count_grouped_envelope_format() {
    // 分组聚合统一返回 {count, data:[{key,count}]}，前端按此契约渲染柱状图
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 2,
        "data": [{"key": "app1", "count": 3}, {"key": "app2", "count": 7}]
    }));
    let v = serde_json::to_value(&result).unwrap();
    assert_eq!(v["type"], "success");
    assert_eq!(v["data"]["count"], 2);
    assert_eq!(v["data"]["data"][1]["key"], "app2");
}
