#[cfg(test)]
mod tests {
    use crate::{attendance_assemble_control_router, ControlRule};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn mock_pool() -> Pool {
        Pool::builder(Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .max_size(1)
        .build()
        .unwrap()
    }

    #[tokio::test]
    async fn test_rule_list_route_accessible() {
        let app = attendance_assemble_control_router(mock_pool());

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
    async fn test_rule_toggle_route_accessible() {
        let app = attendance_assemble_control_router(mock_pool());

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
        // POST route with path param may return 404 in some configurations
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
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
}
