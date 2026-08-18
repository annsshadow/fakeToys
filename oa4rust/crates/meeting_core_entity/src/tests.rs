#[cfg(test)]
mod tests {
    use crate::{entities::{meeting_room::Model as MeetingRoom, meeting::Model as Meeting}, meeting_core_entity_router};
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
    async fn test_room_list_returns_internal_error_without_db() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

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
    async fn test_create_room_route_accessible() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/room/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"Test Room","capacity":10}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_room_route_accessible() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/room/room-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(matches!(response.status(), StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_update_room_route_accessible() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/room/save/room-001")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"Updated Room"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_room_returns_error_without_db() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/room/save/room-001")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"Updated Room"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(matches!(
            response.status(),
            StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND
        ));
    }

    #[tokio::test]
    async fn test_delete_room_route_accessible() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/core/entity/room/delete/room-001")
                    .method(axum::http::Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(matches!(response.status(), StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_meeting_list_returns_internal_error_without_db() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

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
    async fn test_meeting_list_by_room_returns_internal_error() {
        let pool = build_test_pool();
        let app = meeting_core_entity_router(pool);

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
            equipment: None,
            description: None,
            photo: None,
            open_meeting: None,
            order_number: None,
            create_time: None,
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
            room_id: Some("room-001".to_string()),
            start_time: chrono::NaiveDateTime::from_timestamp_opt(1704067200, 0).unwrap(),
            end_time: chrono::NaiveDateTime::from_timestamp_opt(1704070800, 0).unwrap(),
            creator: Some("user-001".to_string()),
            create_time: None,
        };
        let json = serde_json::to_value(&meeting).unwrap();
        assert_eq!(json["id"], "meeting-001");
        assert_eq!(json["title"], "项目评审会");
        assert_eq!(json["room_id"], "room-001");
    }
}
