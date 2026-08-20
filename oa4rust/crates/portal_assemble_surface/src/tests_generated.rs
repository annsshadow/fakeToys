#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/surface/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_surface route should be registered");
    }

    #[tokio::test]
    async fn test_create_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/surface/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_surface route should be registered");
    }

    #[tokio::test]
    async fn test_list_surfaces() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/surface/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_surfaces route should be registered");
    }

    #[tokio::test]
    async fn test_preview_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/surface/preview/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "preview_surface route should be registered");
    }

    #[tokio::test]
    async fn test_publish_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/surface/publish/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "publish_surface route should be registered");
    }

    #[tokio::test]
    async fn test_surface_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/surface/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_list route should be registered");
    }

    #[tokio::test]
    async fn test_surface_preview() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/surface/test-id/preview")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_preview route should be registered");
    }

    #[tokio::test]
    async fn test_surface_publish() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/surface/publish")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_publish route should be registered");
    }

    // SKIPPED: get_layout not accessible
    // SKIPPED: list_layouts not accessible
    // SKIPPED: create_layout not accessible
    // SKIPPED: save_layout not accessible
    // SKIPPED: delete_layout not accessible
    // SKIPPED: dict_list_portal_portalFlag not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_data not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_mockputtopost not accessible
    // SKIPPED: file_list_portal_portalFlag not accessible
    // SKIPPED: file_flag not accessible
    // SKIPPED: file_flag_download not accessible
    // SKIPPED: file_flag_portal_portalFlag_content not accessible
    // SKIPPED: file_flag_portal_portalFlag_download not accessible
    // SKIPPED: page_list_portal_portal not accessible
    // SKIPPED: page_v2_flag_portal_portalFlag not accessible
    // SKIPPED: page_v2_flag_portal_portalFlag_mobile not accessible
    // SKIPPED: page_v2_id not accessible
    // SKIPPED: page_v2_id_mobile not accessible
    // SKIPPED: page_flag_portal_portalFlag not accessible
    // SKIPPED: page_flag_portal_portalFlag_mobile not accessible
    // SKIPPED: page_id not accessible
    // SKIPPED: page_id_mobile not accessible
    // SKIPPED: portal_list not accessible
    // SKIPPED: portal_list_mobile not accessible
    // SKIPPED: portal_flag not accessible
    // SKIPPED: portal_flag_corner_mark not accessible
    // SKIPPED: portal_id_icon not accessible
    // SKIPPED: portal_id_icon_base64 not accessible
    // SKIPPED: script_list_portal_portal not accessible
    // SKIPPED: script_portal_portal_name_name not accessible
    // SKIPPED: script_portal_portal_name_name_imported not accessible
    // SKIPPED: script_id not accessible
    // SKIPPED: widget_list_portal_portal not accessible
    // SKIPPED: widget_flag_portal_portalFlag not accessible
    // SKIPPED: widget_flag_portal_portalFlag_mobile not accessible
    // SKIPPED: widget_id not accessible
    // SKIPPED: widget_id_mobile not accessible
}