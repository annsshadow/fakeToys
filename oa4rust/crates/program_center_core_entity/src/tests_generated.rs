#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_agent_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "agent_list route should be registered");
    }

    // SKIPPED: agent_create requires Session parameter
    // SKIPPED: agent_update requires Session parameter
    // SKIPPED: agent_delete requires Session parameter
    #[tokio::test]
    async fn test_application_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_list route should be registered");
    }

    // SKIPPED: application_create requires Session parameter
    // SKIPPED: application_update requires Session parameter
    // SKIPPED: application_delete requires Session parameter
    #[tokio::test]
    async fn test_invoke_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/invoke/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "invoke_list route should be registered");
    }

    // SKIPPED: invoke_create requires Session parameter
    // SKIPPED: invoke_update requires Session parameter
    // SKIPPED: invoke_delete requires Session parameter
    #[tokio::test]
    async fn test_script_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/script/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list route should be registered");
    }

    // SKIPPED: script_create requires Session parameter
    // SKIPPED: script_update requires Session parameter
    // SKIPPED: script_delete requires Session parameter
    #[tokio::test]
    async fn test_structure_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/structure/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "structure_list route should be registered");
    }

    // SKIPPED: structure_create requires Session parameter
    // SKIPPED: structure_update requires Session parameter
    // SKIPPED: structure_delete requires Session parameter
    // SKIPPED: program_center_core_entity_router not accessible
    // SKIPPED: router not accessible
}