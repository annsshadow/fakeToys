// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_ann_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai/assemble/control/ann/list")
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
            assert!(!bytes.is_empty(), "ann_list route should be registered");
        }
    }

    // SKIPPED: ann_create requires Session parameter
    // SKIPPED: ann_save requires Session parameter
    // SKIPPED: ann_delete requires Session parameter
    #[tokio::test]
    async fn test_config_get_mcp_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai/config/get/mcp/test-id")
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
                "config_get_mcp_flag route should be registered"
            );
        }
    }

    // SKIPPED: config_save requires Session parameter
    // SKIPPED: file_upload requires Session parameter
    #[tokio::test]
    async fn test_file_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/file/test-id")
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
            assert!(!bytes.is_empty(), "file_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_config_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/ai_assemble_control/config/get")
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
            assert!(!bytes.is_empty(), "config_get route should be registered");
        }
    }

    // SKIPPED: get_ai_control_config not accessible
    // SKIPPED: list_ai_models not accessible
    // SKIPPED: update_ai_control_config requires Session parameter
    // SKIPPED: get_usage_stats not accessible
    // SKIPPED: config_base_config not accessible
    // SKIPPED: config_create_mcp requires Session parameter
    // SKIPPED: config_create_model requires Session parameter
    // SKIPPED: config_delete_mcp_flag requires Session parameter
    // SKIPPED: config_delete_model_flag requires Session parameter
    // SKIPPED: config_get_mcp_ext_flag not accessible
    // SKIPPED: config_get_model_flag not accessible
    // SKIPPED: config_list_enable_model not accessible
    // SKIPPED: config_list_mcp_paging_page_size_size not accessible
    // SKIPPED: config_list_model_paging_page_size_size not accessible
    // SKIPPED: config_update_mcp_flag requires Session parameter
    // SKIPPED: config_update_model_flag requires Session parameter
    // SKIPPED: file_copy_file requires Session parameter
    // SKIPPED: file_delete_flag requires Session parameter
    // SKIPPED: file_list_with_ids not accessible
    // SKIPPED: file_list_paging_page_size_size not accessible
    // SKIPPED: file_id_download not accessible
    // SKIPPED: file_id_download_scale not accessible
    // SKIPPED: index_cms_doc_with_app_appId not accessible
    // SKIPPED: index_cms_doc_docId not accessible
    // SKIPPED: index_delete_flag requires Session parameter
    // SKIPPED: index_list_paging_page_size_size not accessible
    // SKIPPED: index_sync_to_knowledge requires Session parameter
    // SKIPPED: chat_completion requires Session parameter
    // SKIPPED: chat_completion_stream requires Session parameter
    // SKIPPED: chat_list_paging_page_size_size requires Session parameter
    // SKIPPED: chat_list_completion_clue_id_paging_page_size_size not accessible
    // SKIPPED: chat_delete_clue_id requires Session parameter
    // SKIPPED: chat_write_completion_extra requires Session parameter
}
