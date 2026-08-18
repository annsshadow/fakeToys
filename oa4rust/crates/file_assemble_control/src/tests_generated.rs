#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    // SKIPPED: get_control_config not accessible
    // SKIPPED: list_storage_pools not accessible
    // SKIPPED: update_control_config not accessible
    // SKIPPED: list_control_categories not accessible
    #[tokio::test]
    async fn test_list_files() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_files route should be registered");
    }

    #[tokio::test]
    async fn test_get_file() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_file route should be registered");
    }

    // SKIPPED: upload_file requires Session parameter
    // SKIPPED: create_file requires Session parameter
    #[tokio::test]
    async fn test_delete_file() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/assemble/control/file/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_file route should be registered");
    }

    #[tokio::test]
    async fn test_create_file_entity() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/core/entity/file/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_file_entity route should be registered");
    }

    // SKIPPED: update_file_entity requires Session parameter
    // SKIPPED: delete_file_entity requires Session parameter
    // SKIPPED: anonymous_file_id_download not accessible
    #[tokio::test]
    async fn test_anonymous_file_id_download_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/file/test-id/download/stream")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_file_id_download_stream route should be registered");
    }

    // SKIPPED: attachment_list_editor_owner requires Session parameter
    // SKIPPED: attachment_list_folder_folderId not accessible
    // SKIPPED: attachment_list_share_owner requires Session parameter
    // SKIPPED: attachment_list_top not accessible
    // SKIPPED: attachment_upload_folder_folderId requires Session parameter
    // SKIPPED: attachment_upload_folder_folderId_callback_callback requires Session parameter
    // SKIPPED: attachment_id not accessible
    // SKIPPED: attachment_id_binary_base64 not accessible
    // SKIPPED: attachment_id_download not accessible
    #[tokio::test]
    async fn test_attachment_id_download_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attachment/download/test-id/stream")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "attachment_id_download_stream route should be registered");
    }

    // SKIPPED: attachment_id_image_scale_scale_binary_base64 not accessible
    // SKIPPED: attachment_id_image_width_width_height_height_binary_base64 not accessible
    // SKIPPED: attachment_id_update requires Session parameter
    // SKIPPED: attachment_id_update_callback_callback requires Session parameter
    // SKIPPED: attachment2_exist_file_fileMd5 not accessible
    // SKIPPED: attachment2_list_editor_owner requires Session parameter
    // SKIPPED: attachment2_list_filter_name not accessible
    // SKIPPED: attachment2_list_folder_folderId not accessible
    // SKIPPED: attachment2_list_share_owner requires Session parameter
    // SKIPPED: attachment2_list_top not accessible
    // SKIPPED: attachment2_list_type_page_size_size not accessible
    // SKIPPED: attachment2_upload_folder_folderId requires Session parameter
    // SKIPPED: attachment2_user_capacity requires Session parameter
    // SKIPPED: attachment2_id not accessible
    // SKIPPED: attachment2_id_binary_base64 not accessible
    // SKIPPED: attachment2_id_download not accessible
    // SKIPPED: attachment2_id_download_image_width_width_height_height not accessible
    // SKIPPED: attachment2_id_download_stream not accessible
    // SKIPPED: attachment2_id_image_scale_scale_binary_base64 not accessible
    // SKIPPED: attachment2_id_image_width_width_height_height_binary_base64 not accessible
    // SKIPPED: attachment2_id_office_preview_type_type requires Session parameter
    // SKIPPED: complex_folder_id not accessible
    // SKIPPED: complex_top not accessible
    // SKIPPED: config_is_file_manager requires Session parameter
    // SKIPPED: config_system_config not accessible
    // SKIPPED: editor_list not accessible
    // SKIPPED: file_clean_unused_referencetype_cmsdocument_manage not accessible
    // SKIPPED: file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale not accessible
    // SKIPPED: file_list_referencetype not accessible
    // SKIPPED: file_list_referencetype_referenceType_reference_reference not accessible
    // SKIPPED: file_list_unused_referencetype_cmsdocument_manage not accessible
    // SKIPPED: file_list_id_next_count not accessible
    // SKIPPED: file_list_id_next_count_all not accessible
    // SKIPPED: file_list_id_next_count_referencetype_referenceType not accessible
    // SKIPPED: file_list_id_prev_count not accessible
    // SKIPPED: file_list_id_prev_count_all not accessible
    // SKIPPED: file_list_id_prev_count_referencetype_referenceType not accessible
    // SKIPPED: file_referencetype_referenceType_reference_reference not accessible
    // SKIPPED: file_upload_referencetype_referenceType_reference_reference_scale_scale requires Session parameter
    // SKIPPED: file_upload_referencetype_referenceType_reference_reference_scale_scale_callback_callback requires Session parameter
    // SKIPPED: file_upload_with_url requires Session parameter
    // SKIPPED: file_id not accessible
    // SKIPPED: file_id_binary_base64 not accessible
    // SKIPPED: file_id_download not accessible
    #[tokio::test]
    async fn test_file_id_download_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/test-id/download/stream")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_id_download_stream route should be registered");
    }

    // SKIPPED: folder_list_top not accessible
    // SKIPPED: folder_list_id not accessible
    // SKIPPED: folder_id not accessible
    // SKIPPED: folder2_batch_download not accessible
    // SKIPPED: folder2_list_top not accessible
    // SKIPPED: folder2_list_id not accessible
    // SKIPPED: folder2_id not accessible
    // SKIPPED: folder2_id_download not accessible
    // SKIPPED: recycle_empty requires Session parameter
    // SKIPPED: recycle_list requires Session parameter
    // SKIPPED: recycle_id not accessible
    // SKIPPED: recycle_id_delete requires Session parameter
    // SKIPPED: recycle_id_resume requires Session parameter
    // SKIPPED: share_download_share_shareId_file_fileId not accessible
    // SKIPPED: share_list requires Session parameter
    // SKIPPED: share_list_att_share_shareId_folder_folderId not accessible
    // SKIPPED: share_list_folder_share_shareId_folder_folderId not accessible
    // SKIPPED: share_list_my requires Session parameter
    // SKIPPED: share_list_my2_shareType_fileType requires Session parameter
    // SKIPPED: share_list_to_me requires Session parameter
    // SKIPPED: share_list_to_me2_fileType requires Session parameter
    // SKIPPED: share_share_shareId_file_fileId_folder_folderId not accessible
    // SKIPPED: share_shield_id not accessible
    // SKIPPED: share_id not accessible
    // SKIPPED: share_id_password_password not accessible
}