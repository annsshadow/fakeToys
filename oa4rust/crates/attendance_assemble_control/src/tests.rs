#[cfg(test)]
mod tests {
    use crate::{attendance_assemble_control_router, ControlRule};
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
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
                    .uri("/jaxrs/attendance/assemble/control/rule/list")
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
                    .uri("/jaxrs/attendance/assemble/control/rule/test-id/toggle")
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
    async fn test_get_jaxrs_attendance_assemble_control_attend() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceadmin/list/all")
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
    async fn test_get_jaxrs_attendance_assemble_control_rule_l() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/rule/list")
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
    async fn test_get_jaxrs_attendance_assemble_control_selfho() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/selfholidaysimple/docId/test-id")
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
    async fn test_get_jaxrs_attendance_assemble_control_statis() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/test-id/next/test-id")
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
    async fn test_get_jaxrs_attendance_assemble_control_uuid_r() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/uuid/random")
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
    async fn test_get_jaxrs_attendance_assemble_control_workpl() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/workplace/list/all")
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
    async fn test_post_jaxrs_attendance_assemble_control_attend() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/test-id")
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
    async fn test_post_jaxrs_attendance_assemble_control_rule_i() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/rule/test-id/toggle")
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
    async fn test_post_jaxrs_attendance_assemble_control_statis() {
        let pool = test_pool();
        let app = crate::attendance_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statistic/do")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }


}
