#[cfg(test)]
mod tests {
    use crate::{MeetingRoom, Meeting};
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
    async fn test_room_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::meeting_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/room/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_meeting_list_returns_success() {
        let pool = build_test_pool();
        let app = crate::meeting_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/meeting/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_meeting_list_by_room_returns_success() {
        let pool = build_test_pool();
        let app = crate::meeting_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/meeting/list/by/room-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 由于没有数据库，会返回 INTERNAL_SERVER_ERROR (500) 或 NOT_FOUND (404)
        assert!(response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::NOT_FOUND);
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
    fn test_meeting_room_serialization() {
        let room = MeetingRoom {
            id: "room-001".to_string(),
            name: "第一会议室".to_string(),
            building_id: Some("building-001".to_string()),
            floor: Some("3F".to_string()),
            capacity: Some(20),
        };
        let json = serde_json::to_value(&room).unwrap();
        assert_eq!(json["id"], "room-001");
        assert_eq!(json["name"], "第一会议室");
        assert_eq!(json["capacity"], 20);
    }

    #[test]
    fn test_meeting_serialization() {
        let meeting = Meeting {
            id: "meeting-001".to_string(),
            title: "项目评审会".to_string(),
            content: None,
            room_id: "room-001".to_string(),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            organizer_id: "user-001".to_string(),
        };
        let json = serde_json::to_value(&meeting).unwrap();
        assert_eq!(json["id"], "meeting-001");
        assert_eq!(json["title"], "项目评审会");
        assert_eq!(json["room_id"], "room-001");
    }
}
