#[cfg(test)]
mod tests {
    use crate::{Building, OpenMeetingRoom, Room, meeting_router};
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
        let app = meeting_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/room/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_building_list_returns_internal_error_without_db() {
        let pool = build_test_pool();
        let app = meeting_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/building/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_openmeeting_list_room_returns_internal_error_without_db() {
        let pool = build_test_pool();
        let app = meeting_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/openmeeting/list/room")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_room_list_response() {
        let rooms = vec![
            Room {
                id: "room-001".to_string(),
                name: "第一会议室".to_string(),
                building_id: Some("building-001".to_string()),
                floor: Some("3F".to_string()),
                capacity: Some(20),
                equipment: None,
                description: Some("大型会议室".to_string()),
                photo: None,
                order_number: Some(1),
            },
        ];

        let result: ActionResult<Vec<Room>> = ActionResult::success(rooms);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, "room-001");
        assert_eq!(data[0].name, "第一会议室");
    }

    #[test]
    fn test_building_list_response() {
        let buildings = vec![
            Building {
                id: "building-001".to_string(),
                name: "总部大楼".to_string(),
                address: Some("北京市朝阳区".to_string()),
                description: Some("公司总部".to_string()),
                order_number: Some(1),
            },
        ];

        let result: ActionResult<Vec<Building>> = ActionResult::success(buildings);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].name, "总部大楼");
    }

    #[test]
    fn test_openmeeting_room_list_response() {
        let rooms = vec![OpenMeetingRoom {
            id: "open-001".to_string(),
            name: "开放式讨论区A".to_string(),
            url: Some("https://meeting.example.com/room/open-001".to_string()),
        }];

        let result: ActionResult<Vec<OpenMeetingRoom>> = ActionResult::success(rooms);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        let data = result.data.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].url, Some("https://meeting.example.com/room/open-001".to_string()));
    }

    #[test]
    fn test_room_serialization() {
        let room = Room {
            id: "room-001".to_string(),
            name: "Test Room".to_string(),
            building_id: Some("b-001".to_string()),
            floor: Some("1F".to_string()),
            capacity: Some(10),
            equipment: None,
            description: None,
            photo: None,
            order_number: None,
        };

        let json = serde_json::to_value(&room).unwrap();
        assert_eq!(json["id"], "room-001");
        assert_eq!(json["name"], "Test Room");
        assert_eq!(json["capacity"], 10);
    }

    #[test]
    fn test_building_serialization() {
        let building = Building {
            id: "b-001".to_string(),
            name: "Test Building".to_string(),
            address: None,
            description: None,
            order_number: None,
        };

        let json = serde_json::to_value(&building).unwrap();
        assert_eq!(json["id"], "b-001");
        assert_eq!(json["name"], "Test Building");
    }
}
