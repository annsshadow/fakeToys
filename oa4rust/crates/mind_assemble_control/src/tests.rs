#[cfg(test)]
mod tests {
    use crate::mind_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
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
    async fn test_config_route_accessible() {
        let app = mind_assemble_control_router(mock_pool());

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
    async fn test_config_update_route_accessible() {
        let app = mind_assemble_control_router(mock_pool());

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
    async fn test_config_update_missing_id() {
        let app = mind_assemble_control_router(mock_pool());

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
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("id is required"));
    }
}
