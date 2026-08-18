#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use tower::util::ServiceExt;
    use crate::router;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_user_setting_route_exists() {
        let pool = build_test_pool();
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
    async fn test_user_role_list_route_exists() {
        let pool = build_test_pool();
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
        let pool = build_test_pool();
        let _ = router(pool);
    }
}
