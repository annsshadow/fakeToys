#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    // SKIPPED: bind_init requires Session parameter
    // SKIPPED: qiyeweixin_bind_callback not accessible
    // SKIPPED: qiyeweixin_bind_confirm requires Session parameter
    // SKIPPED: qiyeweixin_login requires Session parameter
    // SKIPPED: dingding_bind_callback not accessible
    // SKIPPED: dingding_bind_confirm requires Session parameter
    // SKIPPED: dingding_login requires Session parameter
    // SKIPPED: zhengwudingding_bind_callback not accessible
    // SKIPPED: zhengwudingding_bind_confirm requires Session parameter
    // SKIPPED: zhengwudingding_login requires Session parameter
    #[tokio::test]
    async fn test_person_id_icon() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/person/test-id/icon")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "person_id_icon route should be registered");
    }

    #[tokio::test]
    async fn test_identity_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/authentication/identity/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "identity_id route should be registered");
    }

}