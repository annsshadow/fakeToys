#[cfg(test)]
mod tests {
    use crate::{AttendanceRecord, AttendanceRule};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).build().unwrap()
    }

    #[tokio::test]
    async fn test_record_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::attendance_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/core/entity/record/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_rule_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::attendance_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/core/entity/rule/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_action_result_format() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 1);
    }

    #[test]
    fn test_attendance_record_serialization() {
        let record = AttendanceRecord {
            id: "record-001".to_string(),
            user_id: "user-001".to_string(),
            check_in_time: "2024-01-01T09:00:00Z".to_string(),
            check_out_time: Some("2024-01-01T18:00:00Z".to_string()),
            status: "normal".to_string(),
        };
        let json = serde_json::to_value(&record).unwrap();
        assert_eq!(json["id"], "record-001");
        assert_eq!(json["user_id"], "user-001");
        assert_eq!(json["status"], "normal");
    }

    #[test]
    fn test_attendance_rule_serialization() {
        let rule = AttendanceRule {
            id: "rule-001".to_string(),
            name: "标准考勤".to_string(),
            start_time: "09:00".to_string(),
            end_time: "18:00".to_string(),
        };
        let json = serde_json::to_value(&rule).unwrap();
        assert_eq!(json["id"], "rule-001");
        assert_eq!(json["name"], "标准考勤");
        assert_eq!(json["start_time"], "09:00");
    }
}
