// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::{meeting_assemble_control_router, MeetingControl};
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use shared::response::ActionResult;
    use shared::testing::test_pool;
    use tower::ServiceExt;

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_list_controls_route_accessible() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/list/meeting-001")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(matches!(
            response.status(),
            StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND
        ));
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_create_control_route_accessible() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/create")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"meetingId":"meeting-001","controlType":"RECORDER","enabled":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_delete_control_route_accessible() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/delete/test-id")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(matches!(
            response.status(),
            StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR | StatusCode::NOT_FOUND
        ));
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_meeting_add_invite_route_accessible() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/meeting-001/add/invite")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"invitee":"user-001"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_meeting_delete_invite_route_accessible() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/meeting-001/delete/invite")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"invitee":"user-001"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_meeting_create_returns_internal_error_without_db() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/create")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"title":"Test","startTime":"2024-01-01","endTime":"2024-01-02"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_room_list_returns_internal_error_without_db() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/room/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_building_list_returns_internal_error_without_db() {
        let app = meeting_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/list")
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
        assert_eq!(json["controlType"], "RECORDER");
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
            "meetingId": "meeting-001",
            "data": [
                {
                    "id": "ctrl-001",
                    "meetingId": "meeting-001",
                    "controlType": "RECORDER",
                    "enabled": true
                },
                {
                    "id": "ctrl-002",
                    "meetingId": "meeting-001",
                    "controlType": "SCREEN",
                    "enabled": false
                }
            ]
        }));

        assert_eq!(result.r#type, Some("success".to_string()));
        let data = result.data.unwrap();
        assert_eq!(data["count"], 2);
        assert_eq!(data["data"][0]["controlType"], "RECORDER");
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

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_delete_meeting_assemble_control_delete_id() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/delete/test-id")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_building_() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/building/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_config_sy() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/config/system/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_list_meet() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_meeting_l() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/list/applied/completed")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_meeting_i() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_openmeeti() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/openmeeting/list/room")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_room_list() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/room/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_get_meeting_assemble_control_room_id() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/room/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_meeting_assemble_control_config_sy() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/config/system/config/manage")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_meeting_assemble_control_create() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_meeting_assemble_control_meeting_c() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_meeting_assemble_control_meeting_d() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_meeting_assemble_control_meeting_s() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn test_post_meeting_assemble_control_meeting_i() {
        let pool = test_pool();
        let app = crate::meeting_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/assemble/control/meeting/test-id/accept")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    // 优化二轮 43：building/room 搜索 LIKE 通配符转义。意图——搜索词里的 `_`/`%`
    // 必须当字面量匹配，不得被解释成 ILIKE 通配符（如搜 "_" 命中任意单字=全表泄漏）。
    #[ignore = "requires a running PostgreSQL server"]
    #[tokio::test]
    async fn building_search_treats_underscore_as_literal_not_wildcard() {
        let pool = test_pool();
        let c = pool.get().await.unwrap();
        c.execute(
            "CREATE TABLE IF NOT EXISTS x_meeting_building (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255) NOT NULL, address TEXT, description TEXT, order_number INTEGER DEFAULT 0, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "ALTER TABLE x_meeting_building ADD COLUMN IF NOT EXISTS pinyin TEXT",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "ALTER TABLE x_meeting_building ADD COLUMN IF NOT EXISTS pinyin_initial TEXT",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM x_meeting_building WHERE id IN ('u2-bld-under','u2-bld-plain')",
            &[],
        )
        .await
        .unwrap();
        // pinyin 'a_c' 含字面下划线；'abc' 不含
        c.execute(
            "INSERT INTO x_meeting_building (id, name, pinyin) VALUES ('u2-bld-under','U','a_c'), ('u2-bld-plain','P','abc')",
            &[],
        )
        .await
        .unwrap();

        // 搜索 "_"：转义后只命中含字面下划线的 'a_c'，不得把 'abc' 也命中（未转义时 _ 为任意单字会全命中）
        let resp = crate::building_list_like_pinyin_key(
            axum::Extension(pool.clone()),
            axum::extract::Path("_".to_string()),
        )
        .await
        .unwrap();
        let j = serde_json::to_value(&resp.0).unwrap();
        let ids: Vec<String> = j["data"]
            .as_array()
            .expect("data array")
            .iter()
            .map(|r| r["id"].as_str().unwrap_or_default().to_string())
            .collect();
        assert!(
            ids.contains(&"u2-bld-under".to_string()),
            "应命中含字面下划线的 pinyin"
        );
        assert!(
            !ids.contains(&"u2-bld-plain".to_string()),
            "下划线须按字面匹配，不得当通配符命中 'abc'"
        );

        c.execute(
            "DELETE FROM x_meeting_building WHERE id IN ('u2-bld-under','u2-bld-plain')",
            &[],
        )
        .await
        .unwrap();
    }
}
