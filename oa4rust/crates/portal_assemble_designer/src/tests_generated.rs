#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_create_design() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_design route should be registered");
    }

    #[tokio::test]
    async fn test_get_design() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_design route should be registered");
    }

    #[tokio::test]
    async fn test_list_designs() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_designs route should be registered");
    }

    #[tokio::test]
    async fn test_save_design() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_design route should be registered");
    }

    #[tokio::test]
    async fn test_list_pages_by_category() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/page/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_pages_by_category route should be registered");
    }

    #[tokio::test]
    async fn test_get_page() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/page/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_page route should be registered");
    }

    #[tokio::test]
    async fn test_create_page() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/page/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_page route should be registered");
    }

    #[tokio::test]
    async fn test_save_page() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/page/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_page route should be registered");
    }

    #[tokio::test]
    async fn test_delete_page() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/assemble/designer/page/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_page route should be registered");
    }

    #[tokio::test]
    async fn test_design_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/design/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_list route should be registered");
    }

    #[tokio::test]
    async fn test_design_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/design/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_get route should be registered");
    }

    #[tokio::test]
    async fn test_design_save() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/portal/design/save")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_save route should be registered");
    }

    // SKIPPED: designer_search not accessible
    // SKIPPED: dict_list_paging_page_size_size not accessible
    // SKIPPED: dict_list_portal_portalId not accessible
    // SKIPPED: dict_id not accessible
    // SKIPPED: file_list_application_applicationFlag not accessible
    // SKIPPED: file_list_id_next_count not accessible
    // SKIPPED: file_list_id_prev_count not accessible
    // SKIPPED: file_flag not accessible
    // SKIPPED: file_id not accessible
    // SKIPPED: file_id_download not accessible
    // SKIPPED: file_id_upload not accessible
    // SKIPPED: id_count not accessible
    // SKIPPED: input_compare not accessible
    // SKIPPED: input_cover not accessible
    // SKIPPED: input_create not accessible
    // SKIPPED: input_prepare_cover not accessible
    // SKIPPED: input_prepare_create not accessible
    // SKIPPED: output_list not accessible
    // SKIPPED: output_flag_select_file not accessible
    // SKIPPED: output_portalFlag_select not accessible
    // SKIPPED: page_list_portal_portalId not accessible
    // SKIPPED: page_id not accessible
    // SKIPPED: pageversion_list_page_pageId not accessible
    // SKIPPED: pageversion_id not accessible
    // SKIPPED: portal_list not accessible
    // SKIPPED: portal_list_portalcategory_portalCategory not accessible
    // SKIPPED: portal_list_summary not accessible
    // SKIPPED: portal_list_summary_portalcategory_portalCategory not accessible
    // SKIPPED: portal_list_summary_v2 not accessible
    // SKIPPED: portal_id not accessible
    // SKIPPED: portal_id_icon not accessible
    // SKIPPED: portal_id_permission not accessible
    // SKIPPED: portalcategory_list not accessible
    // SKIPPED: script_list_manager not accessible
    // SKIPPED: script_list_paging_page_size_size not accessible
    // SKIPPED: script_list_portal_portalId not accessible
    // SKIPPED: script_id not accessible
    // SKIPPED: scriptversion_list_script_scriptId not accessible
    // SKIPPED: scriptversion_id not accessible
    // SKIPPED: templatepage_list not accessible
    // SKIPPED: templatepage_list_category not accessible
    // SKIPPED: templatepage_id not accessible
    // SKIPPED: widget_list_portal_portalId not accessible
    // SKIPPED: widget_id not accessible
}