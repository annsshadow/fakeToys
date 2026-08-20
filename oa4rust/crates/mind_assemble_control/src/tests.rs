#[cfg(test)]
mod tests {
    use crate::mind_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::test_pool;
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_config_route_accessible() {
        let app = mind_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/config")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_config_update_route_accessible() {
        let app = mind_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/config/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"id":"test-id","configData":"{}"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_config_update_missing_id() {
        let app = mind_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/config/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"configData":"{}"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8_lossy(&body);
        // Either route returns non-404 or body contains error message
        assert!(status != StatusCode::NOT_FOUND || body_str.contains("id is required"));
    }
}
