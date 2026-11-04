// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    // SKIPPED: create_design requires Session parameter
    #[tokio::test]
    async fn test_get_design() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/designer/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "get_design route should be registered");
        }
    }

    #[tokio::test]
    async fn test_list_designs() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/designer")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "list_designs route should be registered");
        }
    }

    // SKIPPED: save_design requires Session parameter
    #[tokio::test]
    async fn test_get_page() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/designer/page/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "get_page route should be registered");
        }
    }

    #[tokio::test]
    async fn test_design_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/design/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "design_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_design_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/design/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "design_get route should be registered");
        }
    }

    // SKIPPED: design_save requires Session parameter
    #[tokio::test]
    async fn test_dict_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/designer/dict/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "dict_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_page_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/designer/page/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "page_list route should be registered");
        }
    }

    // SKIPPED: list_pages_by_category not accessible
    // SKIPPED: create_page requires Session parameter
    // SKIPPED: save_page requires Session parameter
    // SKIPPED: delete_page requires Session parameter
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
    // SKIPPED: file_id_upload requires Session parameter
    // SKIPPED: id_count not accessible
    // SKIPPED: input_compare not accessible
    // SKIPPED: input_cover requires Session parameter
    // SKIPPED: input_create requires Session parameter
    // SKIPPED: input_prepare_cover not accessible
    // SKIPPED: input_prepare_create requires Session parameter
    // SKIPPED: output_list not accessible
    // SKIPPED: output_flag_select_file not accessible
    // SKIPPED: output_portalFlag_select not accessible
    // SKIPPED: page_list_portal_portalId not accessible
    // SKIPPED: page_id not accessible
    // SKIPPED: pageversion_list_page_pageId not accessible
    // SKIPPED: page_list_portal_page_portalId not accessible
    // SKIPPED: pageversion_list_page_page_pageId not accessible
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
    // SKIPPED: create_portal requires Session parameter
    // SKIPPED: delete_portal requires Session parameter
    // SKIPPED: update_portal requires Session parameter
    // SKIPPED: update_portal_icon requires Session parameter
    // SKIPPED: portal_id_permission_post requires Session parameter
    // SKIPPED: create_templatepage requires Session parameter
    // SKIPPED: update_templatepage_category not accessible
    // SKIPPED: delete_templatepage requires Session parameter
    // SKIPPED: create_widget requires Session parameter
    // SKIPPED: delete_widget requires Session parameter
    // SKIPPED: update_widget requires Session parameter
    // SKIPPED: dict_create requires Session parameter
    // SKIPPED: dict_save requires Session parameter
    // SKIPPED: dict_delete requires Session parameter
    // SKIPPED: widget_list not accessible
    // SKIPPED: widget_create requires Session parameter
    // SKIPPED: widget_save requires Session parameter
    // SKIPPED: widget_delete requires Session parameter
    // SKIPPED: create requires Session parameter
    // SKIPPED: update requires Session parameter
    // SKIPPED: delete requires Session parameter
}
