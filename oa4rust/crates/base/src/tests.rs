#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::Value;
    use tower::util::ServiceExt;

    use crate::base_router;
    use shared::testing::test_pool;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_echo_get_endpoint_returns_success() {
        let pool = test_pool();
        let app = base_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/echo/get")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_openapi_info_endpoint_returns_success() {
        let pool = test_pool();
        let app = base_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/openapi/info")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::TEMPORARY_REDIRECT);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_echo_get_response_body() {
        let pool = test_pool();
        let app = base_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/echo/get")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body["type"], "success");
        assert_eq!(body["data"]["type"], "echo");
        assert_eq!(body["data"]["message"], "pong");
    }
}
