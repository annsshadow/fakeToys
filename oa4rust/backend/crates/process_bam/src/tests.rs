// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> deadpool_postgres::Pool {
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap()
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"totalProcesses": 128}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[tokio::test]
    async fn test_state_summary() {
        // F-4（2026-10-26 第三轮）：本测试原先用**无连接池**断言 500——那断的是
        // 「连不上库」，不是行为；x_process_bam.status 缺失导致的 500 它同样照过
        // （F-1，CI PG 日志实证）。改为：无库显式跳过；有库断行为——迁移 112 补了
        // status 列后应为 200，且四个计数键必须在场。
        if !shared::testing::is_db_available().await {
            eprintln!("skipping test_state_summary: DB not reachable");
            return;
        }
        let app = crate::router(shared::testing::test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/state/summary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let data = &json["data"];
        for key in ["totalProcesses", "running", "completed", "expired"] {
            assert!(data.get(key).is_some(), "summary 缺键 {key}: {json}");
        }
    }

    #[tokio::test]
    async fn test_state_running() {
        // 同 test_state_summary：原无库断言 500 断的不是行为。
        if !shared::testing::is_db_available().await {
            eprintln!("skipping test_state_running: DB not reachable");
            return;
        }
        let app = crate::router(shared::testing::test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/state/running")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let data = &json["data"];
        for key in ["runningCount", "applications"] {
            assert!(data.get(key).is_some(), "running 缺键 {key}: {json}");
        }
    }

    #[tokio::test]
    async fn test_state_organization() {
        // 原无库断言 500。该查询（x_process_bam JOIN x_process_organization）
        // 不涉及 F-1 的 status 列，fresh DB 上本就应 200。
        if !shared::testing::is_db_available().await {
            eprintln!("skipping test_state_organization: DB not reachable");
            return;
        }
        let app = crate::router(shared::testing::test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/state/organization")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert!(
            json["data"].get("organizations").is_some(),
            "organization 缺 organizations 键: {json}"
        );
    }
}
