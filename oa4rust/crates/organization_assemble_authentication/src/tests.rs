#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;
    use crate::router;

    const TEST_PERSON_ID: &str = "test-person-id";
    const TEST_IDENTITY_ID: &str = "test-identity-id";

    #[tokio::test]
    async fn test_person_id_icon_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/organization/assemble/authentication/person/{}/icon", TEST_PERSON_ID))
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_identity_id_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/jaxrs/organization/assemble/authentication/identity/{}", TEST_IDENTITY_ID))
                    .method(axum::http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_qiyeweixin_login_route_exists() {
        let pool = test_pool();
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/qiyeweixin/login/testcode")
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
