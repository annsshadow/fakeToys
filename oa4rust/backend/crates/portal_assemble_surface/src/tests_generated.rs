// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/surface/get/test-id")
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
            assert!(!bytes.is_empty(), "get_surface route should be registered");
        }
    }

    // SKIPPED: create_surface requires Session parameter
    #[tokio::test]
    async fn test_list_surfaces() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/surface/list/test-id")
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
            assert!(
                !bytes.is_empty(),
                "list_surfaces route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_preview_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/surface/preview/test-id")
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
            assert!(
                !bytes.is_empty(),
                "preview_surface route should be registered"
            );
        }
    }

    // SKIPPED: publish_surface requires Session parameter
    #[tokio::test]
    async fn test_surface_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/surface/list")
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
            assert!(!bytes.is_empty(), "surface_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_surface_preview() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/surface/test-id/preview")
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
            assert!(
                !bytes.is_empty(),
                "surface_preview route should be registered"
            );
        }
    }

    // SKIPPED: surface_publish requires Session parameter
    // SKIPPED: save_layout requires Session parameter
    // SKIPPED: delete_layout requires Session parameter
    #[tokio::test]
    async fn test_dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/portal/assemble/surface/dict/portal/path/data/mockdeletetoget/test-id/test-id")
                    .method("DELETE")
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
            assert!(!bytes.is_empty(),
                "dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget route should be registered");
        }
    }

    // SKIPPED: page_id_delete requires Session parameter
    // SKIPPED: get_layout not accessible
    // SKIPPED: list_layouts not accessible
    // SKIPPED: create_layout requires Session parameter
    // SKIPPED: dict_list_portal_portalFlag not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_data not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_short not accessible
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_mockputtopost requires Session parameter
    // SKIPPED: file_list_portal_portalFlag not accessible
    // SKIPPED: file_flag not accessible
    // SKIPPED: file_flag_download not accessible
    // SKIPPED: file_flag_portal_portalFlag_content not accessible
    // SKIPPED: file_flag_portal_portalFlag_download not accessible
    // SKIPPED: page_list_portal_portal not accessible
    // SKIPPED: page_v2_flag_portal_portalFlag not accessible
    // SKIPPED: page_v2_flag_portal_portalFlag_p3 not accessible
    // SKIPPED: page_v2_flag_portal_portalFlag_mobile not accessible
    // SKIPPED: page_v2_flag_portal_portalFlag_mobile_p3 not accessible
    // SKIPPED: page_v2_id not accessible
    // SKIPPED: page_v2_id_p2 not accessible
    // SKIPPED: page_v2_id_mobile not accessible
    // SKIPPED: page_v2_id_mobile_p2 not accessible
    // SKIPPED: page_flag_portal_portalFlag not accessible
    // SKIPPED: page_flag_portal_portalFlag_p3 not accessible
    // SKIPPED: page_flag_portal_portalFlag_mobile not accessible
    // SKIPPED: page_flag_portal_portalFlag_mobile_p3 not accessible
    // SKIPPED: page_id not accessible
    // SKIPPED: page_id_mobile not accessible
    // SKIPPED: page_id_mobile_p2 not accessible
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
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_delete requires Session parameter
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_post requires Session parameter
    // SKIPPED: dict_dictFlag_portal_portalFlag_path_data_put requires Session parameter
    // SKIPPED: script_portal_portal_name_post requires Session parameter
}
