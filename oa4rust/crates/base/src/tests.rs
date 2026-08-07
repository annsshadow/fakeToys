#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::Value;
    use tower::util::ServiceExt;

    use crate::base_router;

    #[tokio::test]
    async fn test_echo_get_endpoint_returns_success() {
        let pool = mock_pool();
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
    async fn test_openapi_info_endpoint_returns_success() {
        let pool = mock_pool();
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

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_echo_get_response_body() {
        let pool = mock_pool();
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

    fn mock_pool() -> deadpool_postgres::Pool {
        use deadpool_postgres::{Manager, Pool};
        use deadpool_postgres::tokio_postgres::{Config, NoTls};

        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }
}
