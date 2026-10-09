// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::{attendance_assemble_control_router, ControlRule};
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use shared::response::ActionResult;
    use shared::testing::test_pool;
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_rule_list_route_accessible() {
        let app = attendance_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/rule/list")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_rule_toggle_route_accessible() {
        let app = attendance_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/rule/test-id/toggle")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"enabled":true}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        // 参数化路由 {id} 在 axum 0.8 下可匹配（0.7 的 :param/{param} 混用会导致 404）；
        // handler 因测试未提供 Extension(pool) 而返回 500，断言 500 证明路由已匹配
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_control_rule_serialization() {
        let rule = ControlRule {
            id: "rule-001".to_string(),
            rule_name: "迟到检查".to_string(),
            rule_type: "LATE".to_string(),
            enabled: true,
            description: Some("检查员工迟到情况".to_string()),
        };

        let json = serde_json::to_value(&rule).unwrap();
        assert_eq!(json["ruleName"], "迟到检查");
        assert_eq!(json["ruleType"], "LATE");
        assert_eq!(json["enabled"], true);
        assert_eq!(json["description"], "检查员工迟到情况");
    }

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
    }

    #[test]
    fn test_toggle_response_shape() {
        let result = ActionResult::success(serde_json::json!({
            "id": "rule-001",
            "enabled": true,
            "updated": true
        }));
        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["id"], "rule-001");
        assert_eq!(data["enabled"], true);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_attend() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/attendanceadmin/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_rule_l() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/rule/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_selfho() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/selfholidaysimple/docId/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_statis() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/statisticshow/filter/personMonth/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_uuid_r() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/uuid/random")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_workpl() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/workplace/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_attendance_assemble_control_attend() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/attendanceappealInfo/appeal/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_attendance_assemble_control_rule_i() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/rule/test-id/toggle")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_attendance_assemble_control_statis() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/statistic/do")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_attendance_assemble_control_rule_create() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/rule/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_attendance_assemble_control_statistics_list() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attendance/assemble/control/statistics/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    // import_checkin_rows N+1→prefetch+UNNEST collapse (优化二轮 23): verifies
    // the batched import dedups against both pre-existing rows and earlier rows
    // in the same batch, preserves id order, and counts only new inserts.
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn import_checkin_rows_dedups_and_batches() {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS x_attendance_v2_checkin_record (id TEXT PRIMARY KEY, user_id TEXT, record_date_string TEXT, source_type TEXT, check_in_result TEXT, check_in_type TEXT, description TEXT, creator_person TEXT, create_time TIMESTAMP, update_time TIMESTAMP)",
                &[],
            )
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM x_attendance_v2_checkin_record WHERE user_id = $1",
                &[&"u2-imp-user@P"],
            )
            .await
            .unwrap();
        let session = shared::session::Session {
            token: "imp-tok".to_string(),
            person_unique: "u2-imp-user@P".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
        };
        let rows = vec![
            serde_json::json!({"userId":"u2-imp-user@P","recordDateString":"2026-10-09","checkInType":"onDuty"}),
            serde_json::json!({"userId":"u2-imp-user@P","recordDateString":"2026-10-09","checkInType":"onDuty"}),
            serde_json::json!({"userId":"u2-imp-user@P","recordDateString":"2026-10-09","checkInType":"offDuty"}),
        ];
        let (inserted, ids) = crate::import_checkin_rows(&pool, &session, &rows, "导入")
            .await
            .unwrap();
        // two distinct keys inserted; intra-batch duplicate (#1) reuses #0's id.
        assert_eq!(inserted, 2, "only two distinct keys are new");
        assert_eq!(ids.len(), 3, "one id per input row, in order");
        assert_eq!(ids[0], ids[1], "intra-batch duplicate reuses the same id");
        assert_ne!(ids[0], ids[2], "distinct key gets a distinct id");
        let n: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_attendance_v2_checkin_record WHERE user_id = $1",
                &[&"u2-imp-user@P"],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 2, "exactly two rows persisted");
        // re-import the same batch: all keys now pre-exist → zero new inserts.
        let (inserted2, ids2) = crate::import_checkin_rows(&pool, &session, &rows, "导入")
            .await
            .unwrap();
        assert_eq!(inserted2, 0, "re-import inserts nothing");
        assert_eq!(ids2[0], ids[0], "re-import returns the pre-existing id");
        client
            .execute(
                "DELETE FROM x_attendance_v2_checkin_record WHERE user_id = $1",
                &[&"u2-imp-user@P"],
            )
            .await
            .unwrap();
    }
}
