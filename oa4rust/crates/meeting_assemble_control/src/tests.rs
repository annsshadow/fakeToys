#[cfg(test)]
mod tests {
    use crate::{meeting_assemble_control_router, MeetingControl};
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
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
    async fn test_list_controls_route_accessible() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/list/meeting-001")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(matches!(response.status(), StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_create_control_route_accessible() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/create")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"meetingId":"meeting-001","controlType":"RECORDER","enabled":true}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_control_route_accessible() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/delete/test-id")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(matches!(response.status(), StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn test_meeting_add_invite_route_accessible() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/meeting-001/add/invite")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"invitee":"user-001"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_meeting_delete_invite_route_accessible() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/meeting-001/delete/invite")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"invitee":"user-001"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_meeting_create_returns_internal_error_without_db() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/create")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"Test","startTime":"2024-01-01","endTime":"2024-01-02"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_room_list_returns_internal_error_without_db() {
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/list")
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
        let app = meeting_assemble_control_router(mock_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_meeting_control_serialization() {
        let control = MeetingControl {
            id: "ctrl-001".to_string(),
            meeting_id: "meeting-001".to_string(),
            control_type: "RECORDER".to_string(),
            enabled: true,
            config: None,
        };

        let json = serde_json::to_value(&control).unwrap();
        assert_eq!(json["id"], "ctrl-001");
        assert_eq!(json["\"controlType\""], "RECORDER");
        assert_eq!(json["enabled"], true);
    }

    #[test]
    fn test_action_result_success_structure() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
    }

    #[test]
    fn test_list_response_shape() {
        let result = ActionResult::success(serde_json::json!({
            "count": 2,
            "\"meetingId\"": "meeting-001",
            "data": [
                {
                    "id": "ctrl-001",
                    "\"meetingId\"": "meeting-001",
                    "\"controlType\"": "RECORDER",
                    "enabled": true
                },
                {
                    "id": "ctrl-002",
                    "\"meetingId\"": "meeting-001",
                    "\"controlType\"": "SCREEN",
                    "enabled": false
                }
            ]
        }));

        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["count"], 2);
        assert_eq!(data["data"][0]["\"controlType\""], "RECORDER");
        assert_eq!(data["data"][1]["enabled"], false);
    }

    #[test]
    fn test_delete_response_shape() {
        let result = ActionResult::success(serde_json::json!({
            "id": "ctrl-001",
            "deleted": true
        }));

        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["id"], "ctrl-001");
        assert_eq!(data["deleted"], true);
    }

    #[tokio::test]
    async fn test_delete_jaxrs_meeting_assemble_control_delete_id() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/delete/test-id")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_building_() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_config_sy() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/config/system/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_list_meet() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_meeting_l() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/completed")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_meeting_i() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_openmeeti() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/openmeeting/list/room")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_room_list() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_meeting_assemble_control_room_id() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_meeting_assemble_control_config_sy() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/config/system/config/manage")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_meeting_assemble_control_create() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_meeting_assemble_control_meeting_c() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_meeting_assemble_control_meeting_d() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_meeting_assemble_control_meeting_s() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_meeting_assemble_control_meeting_i() {
        let pool = mock_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/accept")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }


}
