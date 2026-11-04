// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    // SKIPPED: update_control_config requires Session parameter
    #[tokio::test]
    async fn test_list_files() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/list/test-id")
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
            assert!(!bytes.is_empty(), "list_files route should be registered");
        }
    }

    #[tokio::test]
    async fn test_get_file() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/test-id")
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
            assert!(!bytes.is_empty(), "get_file route should be registered");
        }
    }

    // SKIPPED: upload_file requires Session parameter
    // SKIPPED: create_file requires Session parameter
    // SKIPPED: delete_file requires Session parameter
    // SKIPPED: create_file_entity requires Session parameter
    // SKIPPED: update_file_entity requires Session parameter
    // SKIPPED: delete_file_entity requires Session parameter
    #[tokio::test]
    async fn test_anonymous_file_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/anonymous/file/test-id/download")
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
                "anonymous_file_id_download route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_anonymous_file_id_download_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/anonymous/file/test-id/download/stream")
                    .method("POST")
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
                "anonymous_file_id_download_stream route should be registered"
            );
        }
    }

    // SKIPPED: attachment_list_editor_owner requires Session parameter
    #[tokio::test]
    async fn test_attachment_list_folder_folderId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment/list/folder/test-id")
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
                "attachment_list_folder_folderId route should be registered"
            );
        }
    }

    // SKIPPED: attachment_list_share_owner requires Session parameter
    #[tokio::test]
    async fn test_attachment_list_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment/list/top")
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
                "attachment_list_top route should be registered"
            );
        }
    }

    // SKIPPED: attachment_id requires Session parameter
    // SKIPPED: attachment_id_binary_base64 requires Session parameter
    // SKIPPED: attachment_id_download requires Session parameter
    // SKIPPED: attachment_id_download_stream requires Session parameter
    // SKIPPED: attachment_id_image_scale_scale_binary_base64 requires Session parameter
    // SKIPPED: attachment_id_image_width_width_height_height_binary_base64 requires Session parameter
    // SKIPPED: attachment_id_update requires Session parameter
    // SKIPPED: attachment_id_update_callback_callback requires Session parameter
    #[tokio::test]
    async fn test_attachment2_exist_file_fileMd5() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment2/exist/file/test-id")
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
                "attachment2_exist_file_fileMd5 route should be registered"
            );
        }
    }

    // SKIPPED: attachment2_list_editor_owner requires Session parameter
    #[tokio::test]
    async fn test_attachment2_list_filter_name() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment2/list/filter/test-id")
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
                "attachment2_list_filter_name route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_attachment2_list_folder_folderId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment2/list/folder/test-id")
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
                "attachment2_list_folder_folderId route should be registered"
            );
        }
    }

    // SKIPPED: attachment2_list_share_owner requires Session parameter
    #[tokio::test]
    async fn test_attachment2_list_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/attachment2/list/top")
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
                "attachment2_list_top route should be registered"
            );
        }
    }

    // SKIPPED: attachment2_user_capacity requires Session parameter
    // SKIPPED: attachment2_id requires Session parameter
    // SKIPPED: attachment2_id_binary_base64 requires Session parameter
    // SKIPPED: attachment2_id_download requires Session parameter
    // SKIPPED: attachment2_id_download_image_width_width_height_height requires Session parameter
    // SKIPPED: attachment2_id_download_stream requires Session parameter
    // SKIPPED: attachment2_id_image_scale_scale_binary_base64 requires Session parameter
    // SKIPPED: attachment2_id_image_width_width_height_height_binary_base64 requires Session parameter
    // SKIPPED: attachment2_id_office_preview_type_type requires Session parameter
    #[tokio::test]
    async fn test_complex_folder_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/complex/folder/test-id")
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
                "complex_folder_id route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_complex_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/complex/top")
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
            assert!(!bytes.is_empty(), "complex_top route should be registered");
        }
    }

    // SKIPPED: config_is_file_manager requires Session parameter
    #[tokio::test]
    async fn test_config_system_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/config/system")
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
                "config_system_config route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_editor_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/editor/list")
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
            assert!(!bytes.is_empty(), "editor_list route should be registered");
        }
    }

    // SKIPPED: file_clean_unused_referencetype_cmsdocument_manage requires Session parameter
    // SKIPPED: file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale requires Session parameter
    // SKIPPED: file_list_referencetype_referenceType_reference_reference requires Session parameter
    #[tokio::test]
    async fn test_file_list_unused_referencetype_cmsdocument_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/assemble/control/file/list/unused/referencetype/cmsdocument/manage")
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
                "file_list_unused_referencetype_cmsdocument_manage route should be registered"
            );
        }
    }

    // SKIPPED: file_list_id_next_count requires Session parameter
    // SKIPPED: file_list_id_next_count_all requires Session parameter
    // SKIPPED: file_list_id_next_count_referencetype_referenceType requires Session parameter
    // SKIPPED: file_list_id_prev_count requires Session parameter
    // SKIPPED: file_list_id_prev_count_all requires Session parameter
    // SKIPPED: file_list_id_prev_count_referencetype_referenceType requires Session parameter
    // SKIPPED: file_upload_with_url requires Session parameter
    // SKIPPED: file_id_binary_base64 requires Session parameter
    // SKIPPED: file_id_download requires Session parameter
    // SKIPPED: file_id_download_stream requires Session parameter
    #[tokio::test]
    async fn test_folder_list_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder/list/top")
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
                "folder_list_top route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_folder_list_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder/list/test-id")
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
                "folder_list_id route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_folder_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder/test-id")
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
            assert!(!bytes.is_empty(), "folder_id route should be registered");
        }
    }

    // SKIPPED: folder2_batch_download requires Session parameter
    #[tokio::test]
    async fn test_folder2_list_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder2/list/top")
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
                "folder2_list_top route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_folder2_list_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder2/list/test-id")
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
                "folder2_list_id route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_folder2_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder2/test-id")
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
            assert!(!bytes.is_empty(), "folder2_id route should be registered");
        }
    }

    #[tokio::test]
    async fn test_folder2_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/folder2/test-id/download")
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
                "folder2_id_download route should be registered"
            );
        }
    }

    // SKIPPED: recycle_empty requires Session parameter
    // SKIPPED: recycle_list requires Session parameter
    #[tokio::test]
    async fn test_recycle_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/recycle/test-id")
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
            assert!(!bytes.is_empty(), "recycle_id route should be registered");
        }
    }

    // SKIPPED: recycle_id_delete requires Session parameter
    // SKIPPED: recycle_id_resume requires Session parameter
    #[tokio::test]
    async fn test_share_download_share_shareId_file_fileId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/share/download/share/test-id/file/test-id")
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
                "share_download_share_shareId_file_fileId route should be registered"
            );
        }
    }

    // SKIPPED: share_list requires Session parameter
    #[tokio::test]
    async fn test_share_list_att_share_shareId_folder_folderId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/share/list/att/share/test-id/folder/test-id")
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
                "share_list_att_share_shareId_folder_folderId route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_share_list_folder_share_shareId_folder_folderId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/share/list/folder/share/test-id/folder/test-id")
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
                "share_list_folder_share_shareId_folder_folderId route should be registered"
            );
        }
    }

    // SKIPPED: share_list_my requires Session parameter
    // SKIPPED: share_list_to_me requires Session parameter
    // SKIPPED: share_share_shareId_file_fileId_folder_folderId requires Session parameter
    // SKIPPED: share_shield_id requires Session parameter
    #[tokio::test]
    async fn test_share_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/share/test-id")
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
            assert!(!bytes.is_empty(), "share_id route should be registered");
        }
    }

    // SKIPPED: u2_file_upload_octet_stream requires Session parameter
    // SKIPPED: u2_file_upload_callback requires Session parameter
    // SKIPPED: u2_attachment_update_content requires Session parameter
    // SKIPPED: u2_attachment_update_content_callback requires Session parameter
    // SKIPPED: u2_file_delete_by_id requires Session parameter
    // SKIPPED: u2_file_delete_by_reference requires Session parameter
    // SKIPPED: u2_file_list_reference_types requires Session parameter
    // SKIPPED: u2_folder_create requires Session parameter
    // SKIPPED: u2_share_create requires Session parameter
    #[tokio::test]
    async fn test_u2_share_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/share/test-id")
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
            assert!(!bytes.is_empty(), "u2_share_get route should be registered");
        }
    }

    // SKIPPED: u2_share_shield requires Session parameter
    #[tokio::test]
    async fn test_u2_share_password_verify() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/share/password/verify")
                    .method("POST")
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
                "u2_share_password_verify route should be registered"
            );
        }
    }

    // SKIPPED: u2_share_save_to_folder requires Session parameter
    // SKIPPED: u2_attachment2_list_type_page_size_size requires Session parameter
    // SKIPPED: u2_config_save_system_config requires Session parameter
    // SKIPPED: get_control_config not accessible
    // SKIPPED: list_storage_pools not accessible
    // SKIPPED: list_control_categories not accessible
    // SKIPPED: attachment_upload_folder_folderId requires Session parameter
    // SKIPPED: attachment_upload_folder_folderId_callback_callback requires Session parameter
    // SKIPPED: attachment_upload_folder_callback_callback_legacy requires Session parameter
    // SKIPPED: attachment2_list_type_page_size_size requires Session parameter
    // SKIPPED: attachment2_upload_folder_folderId requires Session parameter
    // SKIPPED: file_list_referencetype not accessible
    // SKIPPED: file_upload_referencetype_referenceType_reference_reference_scale_scale requires Session parameter
    // SKIPPED: file_upload_referencetype_referenceType_reference_reference_scale_scale_callback_callback requires Session parameter
    // SKIPPED: file_id not accessible
    // SKIPPED: share_list_my2_shareType_fileType requires Session parameter
    // SKIPPED: share_list_to_me2_fileType requires Session parameter
    // SKIPPED: u2_file_upload_multipart requires Session parameter
    // SKIPPED: u2_attachment_delete requires Session parameter
    // SKIPPED: u2_attachment2_update requires Session parameter
    // SKIPPED: u2_attachment2_delete requires Session parameter
    // SKIPPED: u2_folder_rename requires Session parameter
    // SKIPPED: u2_folder_delete requires Session parameter
    // SKIPPED: u2_share_delete requires Session parameter
}
