// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use tower::ServiceExt;


    #[tokio::test]
    async fn test_work_list_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/work/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_work_get_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/work/test-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_task_list_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/task/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_task_get_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/task/test-id")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_ticket_list_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/ticket/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_workcompleted_list_route_exists() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/workcompleted/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_work_list_empty_data_action_result_structure() {
        let result: ActionResult<serde_json::Value> = ActionResult::success(serde_json::json!({
            "count": 0,
            "data": []
        }));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 0);
        assert_eq!(json["data"]["data"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_db_error_returns_internal() {
        let db = shared::testing::mock_db().await;
        let app = crate::processplatform_core_entity_router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/process/work/list")
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
}
