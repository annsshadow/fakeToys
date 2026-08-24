#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;
    use crate::router;

    const TEST_PERSON_ID: &str = "test-person-id-personal";

    #[tokio::test]
    async fn test_user_setting_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/organization/assemble/personal/{}/setting", TEST_PERSON_ID))
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
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/organization/assemble/personal/{}/role/list", TEST_PERSON_ID))
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

    #[tokio::test]
    async fn test_u2_custom_mockputtopost_route() {
        let pool = test_pool();
        let app = router(pool);
        let body = serde_json::to_string(&serde_json::json!({"fieldValue": "v"})).unwrap();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/organization/assemble/personal/custom/test-id/mockputtopost")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_custom_mockputtopost_route_alt() {
        let app = router(test_pool());
        let body = serde_json::to_string(&serde_json::json!({"fieldValue": "v2"})).unwrap();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/jaxrs/organization/assemble/personal/custom/alt-id/mockputtopost")
                    .method(axum::http::Method::POST)
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_router_builds_with_custom() {
        let _ = router(test_pool());
    }
}
