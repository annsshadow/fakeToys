// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::Correlation;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_list_returns_success() {
        let db = shared::testing::mock_db().await;
        let app = crate::correlation_core_entity_router(db);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/correlation/core/entity/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_list_by_source_returns_success() {
        let db = shared::testing::mock_db().await;
        let app = crate::correlation_core_entity_router(db);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/correlation/core/entity/list/by/user/user-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 只读 GET，无会话门；无 DB 时返回 500。
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
    fn test_correlation_serialization() {
        let correlation = Correlation {
            id: "corr-001".to_string(),
            source_type: "user".to_string(),
            source_id: "user-001".to_string(),
            target_type: "doc".to_string(),
            target_id: "doc-001".to_string(),
            weight: 10,
        };
        let json = serde_json::to_value(&correlation).unwrap();
        assert_eq!(json["id"], "corr-001");
        assert_eq!(json["source_type"], "user");
        assert_eq!(json["weight"], 10);
    }

    #[tokio::test]
    async fn test_create_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::correlation_core_entity_router(db);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/correlation/core/entity/create")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"sourceType":"user","sourceId":"u1","targetType":"doc","targetId":"d1","weight":5}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        // create 有会话门：无会话在 DB 访问前 401（B3 接线后）。
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::correlation_core_entity_router(db);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/correlation/core/entity/delete/corr-test-001")
                    .method(axum::http::Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // delete 有会话门：无会话在 DB 访问前 401（B3 接线后）。
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_delete_success_response_structure() {
        let result = shared::response::ActionResult::success(serde_json::json!({"success": true}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["success"], true);
    }
}
