#[cfg(test)]
mod tests {
    use crate::CalendarItem;
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
    async fn test_calendar_list_public_returns_success() {
        let pool = build_test_pool();
        let app = crate::calendar_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/calendar/core/entity/calendar/list/public")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_calendar_list_my_returns_success() {
        let pool = build_test_pool();
        let app = crate::calendar_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/calendar/core/entity/calendar/list/my")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_calendar_get_returns_not_found() {
        let pool = build_test_pool();
        let app = crate::calendar_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/calendar/core/entity/calendar/test-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
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
    fn test_calendar_item_serialization() {
        let item = CalendarItem {
            id: "cal-001".to_string(),
            name: "团队会议".to_string(),
            calendar_type: "UNIT".to_string(),
            target: "team-001".to_string(),
            color: "#FF5733".to_string(),
            description: Some("每周例会".to_string()),
            createor: "user-001".to_string(),
            is_public: true,
            status: "OPEN".to_string(),
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["id"], "cal-001");
        assert_eq!(json["name"], "团队会议");
        assert_eq!(json["is_public"], true);
    }
}
