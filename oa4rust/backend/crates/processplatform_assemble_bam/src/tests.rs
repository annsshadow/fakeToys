// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use serde_json::json;
use tower::util::ServiceExt;

fn build_test_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
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
    // x_bam_config 无 deleted_at 列：delete 拒绝物理删除，返回 error 契约
    let result: ActionResult<serde_json::Value> =
        ActionResult::error("physical delete not supported for this entity");
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "error");
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
                .uri("/api/processplatform/assemble/bam/get/bam-1")
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
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/processplatform/assemble/bam/create")
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
                .uri("/api/processplatform/assemble/bam/list/processplatform")
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
                .uri("/api/processplatform/assemble/bam/delete/bam-1")
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
                .uri("/api/processplatform/assemble/bam/status/bam-1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 新增：o2server 精确路径闭合（42 端点）的测试
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn test_period_predicate_completed_semantics() {
    // 口径：completed = 状态已完成
    assert_eq!(
        period_predicate("task", "completed"),
        "t.task_status = 'completed'"
    );
    assert_eq!(
        period_predicate("work", "completed"),
        "w.work_status = 'completed'"
    );
}

#[test]
fn test_period_predicate_expired_requires_overdue_and_unfinished() {
    // 口径：expired 必须同时满足"有截止时间、已过期、未完成"，防止把未到期的也算超时
    let p = period_predicate("task", "expired");
    assert!(
        p.contains("t.end_time IS NOT NULL"),
        "expired 必须要求 end_time 非空"
    );
    assert!(
        p.contains("t.end_time < NOW()"),
        "expired 必须要求已过截止时间"
    );
    assert!(
        p.contains("IS DISTINCT FROM 'completed'"),
        "expired 必须排除已完成"
    );
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
    let status = bam_u2_route_status(
        Method::GET,
        "/api/processplatform/assemble/bam/period/list/completed/task/applicationstubs",
    )
    .await;
    assert_ne!(
        status,
        StatusCode::NOT_FOUND,
        "applicationstubs 桩端点应注册为 o2server 精确路径"
    );
}

#[tokio::test]
async fn test_bam_count_completed_task_by_unit_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/api/processplatform/assemble/bam/period/list/count/completed/task/application/app1/process/p1/activity/a1/by/unit",
    )
    .await;
    assert_ne!(
        status,
        StatusCode::NOT_FOUND,
        "count...by/unit 精确路径应注册"
    );
}

#[tokio::test]
async fn test_bam_count_start_work_total_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/api/processplatform/assemble/bam/period/list/count/start/work/application/app1/process/p1/unit/u1/person/per1",
    )
    .await;
    assert_ne!(
        status,
        StatusCode::NOT_FOUND,
        "start/work 总数切片应注册且动词为 GET"
    );
}

#[tokio::test]
async fn test_bam_state_category_exact_path_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/api/processplatform/assemble/bam/state/category",
    )
    .await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_bam_state_category_trigger_all_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/api/processplatform/assemble/bam/state/category/trigger",
    )
    .await;
    assert_ne!(
        status,
        StatusCode::NOT_FOUND,
        "/state/category/trigger 应为无参 GET"
    );
}

#[tokio::test]
async fn test_bam_state_applicationtstubs_trigger_registered() {
    let status = bam_u2_route_status(
        Method::GET,
        "/api/processplatform/assemble/bam/state/applicationtstubs/trigger",
    )
    .await;
    assert_ne!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_bam_count_endpoint_rejects_wrong_verb() {
    // o2server 清单中 count 切片是 GET；POST 不应命中同一路径（防止动词漂移回归）
    let status = bam_u2_route_status(
        Method::POST,
        "/api/processplatform/assemble/bam/period/list/count/start/work/application/app1/process/p1/unit/u1/person/per1",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::METHOD_NOT_ALLOWED,
        "GET-only 端点对 POST 应返回 405 而非命中处理"
    );
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

// 回归守卫：每条注册路由的 {param} 槽数必须与 handler 的 Path 提取器元数一致，
// 否则 axum 运行时返回 500「Wrong number of path arguments」。逐条 oneshot 校验无该错误体。
#[tokio::test]
async fn route_path_arity_matches_handlers() {
    let cases: &[(&str, &str)] = &[
        ("GET", "/api/processplatform/assemble/bam/get/x"),
        ("POST", "/api/processplatform/assemble/bam/create"),
        ("GET", "/api/processplatform/assemble/bam/list/x"),
        ("POST", "/api/processplatform/assemble/bam/delete/x"),
        ("GET", "/api/processplatform/assemble/bam/status/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/application/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/application/process/activity/by/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/application/process/activity/x/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/application/process/by/activity/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/application/by/process/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/by/application/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/application/process/by/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/application/process/x/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/application/by/process/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/by/application/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/application/process/activity/by/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/application/process/activity/x/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/application/process/by/activity/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/application/by/process/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/by/application/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/application/process/by/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/application/process/x/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/application/by/process/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/by/application/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/application/process/activity/by/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/application/process/activity/x/x/x/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/application/process/by/activity/x/x/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/application/by/process/x/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/by/application/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/application/process/by/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/application/process/x/x/x/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/application/by/process/x/x/x/x/x/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/by/application/x/x/x/x/x/x/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/application/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/application/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/task/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/application/x/x"),
        ("POST", "/api/processplatform/assemble/bam/period/list/x/x/x"),
        ("POST", "/api/processplatform/assemble/bam/state/trigger/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/applicationstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/task/unitstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/work/applicationstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/completed/work/unitstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/applicationstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/task/unitstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/work/applicationstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/expired/work/unitstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/start/task/applicationstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/start/task/unitstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/start/work/applicationstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/start/work/unitstubs"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/task/application/x/process/x/activity/x/by/unit"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/task/application/x/process/x/activity/x/unit/x/person/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/task/application/x/process/x/unit/x/person/x/by/activity"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/task/application/x/unit/x/person/x/by/process"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/task/unit/x/person/x/by/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/work/application/x/process/x/by/unit"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/work/application/x/process/x/unit/x/person/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/work/application/x/unit/x/person/x/by/process"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/completed/work/unit/x/person/x/by/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/task/application/x/process/x/activity/x/by/unit"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/task/application/x/process/x/activity/x/unit/x/person/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/task/application/x/process/x/unit/x/person/x/by/activity"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/task/application/x/unit/x/person/x/by/process"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/task/unit/x/person/x/by/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/work/application/x/process/x/by/unit"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/work/application/x/process/x/unit/x/person/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/work/application/x/unit/x/person/x/by/process"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/expired/work/unit/x/person/x/by/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/task/application/x/process/x/activity/x/by/unit"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/task/application/x/process/x/activity/x/unit/x/person/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/task/application/x/process/x/unit/x/person/x/by/activity"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/task/application/x/unit/x/person/x/by/process"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/task/unit/x/person/x/by/application"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/work/application/x/process/x/by/unit"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/work/application/x/process/x/unit/x/person/x"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/work/application/x/unit/x/person/x/by/process"),
        ("GET", "/api/processplatform/assemble/bam/period/list/count/start/work/unit/x/person/x/by/application"),
        ("GET", "/api/processplatform/assemble/bam/state/applicationtstubs/trigger"),
        ("GET", "/api/processplatform/assemble/bam/state/category"),
        ("GET", "/api/processplatform/assemble/bam/state/category/trigger"),
        ("GET", "/api/processplatform/assemble/bam/state/summary"),
        ("GET", "/api/processplatform/assemble/bam/state/running"),
        ("GET", "/api/processplatform/assemble/bam/state/organization"),
        ("DELETE", "/api/processplatform/assemble/bam/delete/x"),
    ];
    let mut bad = Vec::new();
    for (method, path) in cases {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let m = Method::from_bytes(method.as_bytes()).unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .method(m)
                    .uri(*path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8_lossy(&bytes);
        if body.contains("Wrong number of path arguments") {
            bad.push((*path).to_string());
        }
    }
    if !bad.is_empty() {
        println!("ARITY-TRAP remaining: {}", bad.len());
        for b in &bad {
            println!("  {}", b);
        }
    }
    assert!(bad.is_empty(), "{} arity traps remain", bad.len());
}

// state_summary 9→2 FILTER-aggregate collapse (优化二轮 25): seed known rows and
// assert the summary buckets move by exactly the seeded deltas, proving the
// per-table FILTER counts still map to the right output keys. Live DB only.
#[tokio::test]
async fn state_summary_filter_buckets_match_seeded_deltas() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.unwrap();

    async fn summary(pool: &Pool) -> serde_json::Value {
        let resp = crate::router(pool.clone())
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/processplatform/assemble/bam/state/summary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()["data"].clone()
    }
    let get = |v: &serde_json::Value, k: &str| v[k].as_i64().unwrap();

    let before = summary(&pool).await;
    // 2 completed + 1 pending + 1 expired works; 1 completed + 1 expired task.
    for (id, st) in [
        ("r25-w-c1", "completed"),
        ("r25-w-c2", "completed"),
        ("r25-w-p1", "pending"),
        ("r25-w-e1", "expired"),
    ] {
        client
            .execute(
                "INSERT INTO x_work (id, title, process, work_status) VALUES ($1, 'r25', 'r25', $2)",
                &[&id, &st],
            )
            .await
            .unwrap();
    }
    for (id, st) in [("r25-t-c1", "completed"), ("r25-t-e1", "expired")] {
        client
            .execute(
                "INSERT INTO x_task (id, work, task_status) VALUES ($1, 'r25-w-c1', $2)",
                &[&id, &st],
            )
            .await
            .unwrap();
    }
    let after = summary(&pool).await;

    assert_eq!(get(&after, "totalWork") - get(&before, "totalWork"), 4);
    assert_eq!(
        get(&after, "completedWork") - get(&before, "completedWork"),
        2
    );
    assert_eq!(get(&after, "pendingWork") - get(&before, "pendingWork"), 1);
    assert_eq!(get(&after, "expiredWork") - get(&before, "expiredWork"), 1);
    assert_eq!(get(&after, "totalTask") - get(&before, "totalTask"), 2);
    assert_eq!(
        get(&after, "completedTask") - get(&before, "completedTask"),
        1
    );
    assert_eq!(get(&after, "expiredTask") - get(&before, "expiredTask"), 1);

    client
        .execute(
            "DELETE FROM x_work WHERE id IN ('r25-w-c1','r25-w-c2','r25-w-p1','r25-w-e1')",
            &[],
        )
        .await
        .unwrap();
    client
        .execute(
            "DELETE FROM x_task WHERE id IN ('r25-t-c1','r25-t-e1')",
            &[],
        )
        .await
        .unwrap();
}

// state_running 5→2 FILTER-aggregate collapse (优化二轮 26): same delta-based
// live assertion as state_summary, for the running-state buckets. Live DB only.
#[tokio::test]
async fn state_running_filter_buckets_match_seeded_deltas() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.unwrap();

    async fn running(pool: &Pool) -> serde_json::Value {
        let resp = crate::router(pool.clone())
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/processplatform/assemble/bam/state/running")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()["data"].clone()
    }
    let get = |v: &serde_json::Value, k: &str| v[k].as_i64().unwrap();

    let before = running(&pool).await;
    // 1 pending + 1 processing work; 1 pending + 1 processing + 2 started task.
    for (id, st) in [("r26-w-p1", "pending"), ("r26-w-r1", "processing")] {
        client
            .execute(
                "INSERT INTO x_work (id, title, process, work_status) VALUES ($1, 'r26', 'r26', $2)",
                &[&id, &st],
            )
            .await
            .unwrap();
    }
    for (id, st) in [
        ("r26-t-p1", "pending"),
        ("r26-t-r1", "processing"),
        ("r26-t-s1", "started"),
        ("r26-t-s2", "started"),
    ] {
        client
            .execute(
                "INSERT INTO x_task (id, work, task_status) VALUES ($1, 'r26-w-p1', $2)",
                &[&id, &st],
            )
            .await
            .unwrap();
    }
    let after = running(&pool).await;

    assert_eq!(get(&after, "pendingWork") - get(&before, "pendingWork"), 1);
    assert_eq!(
        get(&after, "processingWork") - get(&before, "processingWork"),
        1
    );
    assert_eq!(get(&after, "pendingTask") - get(&before, "pendingTask"), 1);
    assert_eq!(
        get(&after, "processingTask") - get(&before, "processingTask"),
        1
    );
    assert_eq!(get(&after, "startedTask") - get(&before, "startedTask"), 2);

    client
        .execute(
            "DELETE FROM x_work WHERE id IN ('r26-w-p1','r26-w-r1')",
            &[],
        )
        .await
        .unwrap();
    client
        .execute(
            "DELETE FROM x_task WHERE id IN ('r26-t-p1','r26-t-r1','r26-t-s1','r26-t-s2')",
            &[],
        )
        .await
        .unwrap();
}

// state_organization 3 跨表 COUNT → 单条标量子查询 (优化二轮 27): one round-trip
// instead of three. Seed person/unit/group rows and assert the three totals move
// by the seeded deltas. Live DB only.
#[tokio::test]
async fn state_organization_scalar_subqueries_match_seeded_deltas() {
    if !shared::testing::is_db_available().await {
        return;
    }
    let pool = shared::testing::test_pool();
    let client = pool.get().await.unwrap();

    async fn org(pool: &Pool) -> serde_json::Value {
        let resp = crate::router(pool.clone())
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/processplatform/assemble/bam/state/organization")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()["data"].clone()
    }
    let get = |v: &serde_json::Value, k: &str| v[k].as_i64().unwrap();

    let before = org(&pool).await;
    for (id, name) in [("r27-p1", "r27a"), ("r27-p2", "r27b")] {
        client
            .execute(
                "INSERT INTO x_org_person (id, name) VALUES ($1, $2)",
                &[&id, &name],
            )
            .await
            .unwrap();
    }
    client
        .execute(
            "INSERT INTO x_org_unit (id, name) VALUES ('r27-u1', 'r27u')",
            &[],
        )
        .await
        .unwrap();
    client
        .execute(
            "INSERT INTO x_org_group (id, name) VALUES ('r27-g1', 'r27g')",
            &[],
        )
        .await
        .unwrap();
    let after = org(&pool).await;

    assert_eq!(
        get(&after, "totalPersons") - get(&before, "totalPersons"),
        2
    );
    assert_eq!(get(&after, "totalUnits") - get(&before, "totalUnits"), 1);
    assert_eq!(get(&after, "totalGroups") - get(&before, "totalGroups"), 1);

    client
        .execute(
            "DELETE FROM x_org_person WHERE id IN ('r27-p1','r27-p2')",
            &[],
        )
        .await
        .unwrap();
    client
        .execute("DELETE FROM x_org_unit WHERE id = 'r27-u1'", &[])
        .await
        .unwrap();
    client
        .execute("DELETE FROM x_org_group WHERE id = 'r27-g1'", &[])
        .await
        .unwrap();
}
