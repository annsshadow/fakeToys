#[cfg(test)]
mod tests {
    use crate::hotpic_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::response::ActionResult;
    use shared::testing::test_pool;
    use tower::ServiceExt;

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_exists_check_with_db() {
        let pool = test_pool();
        let app = hotpic_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/user/hotpic/exists/check")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_by_id_with_db() {
        let pool = test_pool();
        let app = hotpic_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/user/hotpic/hotpic-001")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_list_by_application_and_info_id_with_db() {
        let pool = test_pool();
        let app = hotpic_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/hotpic/user/hotpic/CMS/doc-123")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
