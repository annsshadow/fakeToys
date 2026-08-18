#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;
    use crate::router;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_user_setting_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/personal/test-id/setting")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_user_role_list_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/personal/test-id/role/list")
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_router_builds() {
        let pool = test_pool();
        let _ = router(pool);
    }
}
