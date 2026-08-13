#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_application_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_id route should be registered");
    }

    #[tokio::test]
    async fn test_get_control_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/get/control/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_control_config route should be registered");
    }

    #[tokio::test]
    async fn test_list_control_sections() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/list/control/sections")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_control_sections route should be registered");
    }

    #[tokio::test]
    async fn test_update_control_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/update/control/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "update_control_config route should be registered");
    }

    // SKIPPED: anonymous_document_filter_list_id_next_count not accessible
    // SKIPPED: anonymous_document_filter_list_id_next_count_mockputtopost not accessible
    // SKIPPED: anonymous_document_filter_list_page_size_size not accessible
    // SKIPPED: anonymous_document_filter_list_page_size_size_mockputtopost not accessible
    // SKIPPED: anonymous_document_id_view not accessible
    // SKIPPED: anonymous_fileinfo_list_document_documentId not accessible
    // SKIPPED: appinfo_alias_alias not accessible
    // SKIPPED: appinfo_erase_app_id not accessible
    // SKIPPED: appinfo_erase_app_id_mockdeletetoget not accessible
    // SKIPPED: appinfo_filter_list_id_next_count not accessible
    // SKIPPED: appinfo_filter_list_id_next_count_mockputtopost not accessible
    // SKIPPED: appinfo_filter_list_id_prev_count not accessible
    // SKIPPED: appinfo_filter_list_id_prev_count_mockputtopost not accessible
    // SKIPPED: appinfo_get_user_publish_appId not accessible
    // SKIPPED: appinfo_list_all not accessible
    // SKIPPED: appinfo_list_appType not accessible
    // SKIPPED: appinfo_list_appType_manager not accessible
    // SKIPPED: appinfo_list_has_document not accessible
    // SKIPPED: appinfo_list_has_document_appType not accessible
    // SKIPPED: appinfo_list_has_document_type_appType not accessible
    // SKIPPED: appinfo_list_manage not accessible
    // SKIPPED: appinfo_list_manage_type_appType not accessible
    // SKIPPED: appinfo_list_user_publish not accessible
    // SKIPPED: appinfo_list_user_publish_type_appType not accessible
    // SKIPPED: appinfo_list_user_publish_with_process not accessible
    // SKIPPED: appinfo_list_user_view not accessible
    // SKIPPED: appinfo_list_user_view_all not accessible
    // SKIPPED: appinfo_list_user_view_all_type_appType not accessible
    // SKIPPED: appinfo_list_user_view_article_type_appType not accessible
    // SKIPPED: appinfo_list_user_view_data not accessible
    // SKIPPED: appinfo_list_user_view_data_type_appType not accessible
    // SKIPPED: appinfo_appId_icon_size_size not accessible
    // SKIPPED: appinfo_flag not accessible
    // SKIPPED: appinfo_id not accessible
    // SKIPPED: appinfo_id_control not accessible
    // SKIPPED: appinfo_id_mockdeletetoget not accessible
    // SKIPPED: appinfo_id_permission not accessible
    // SKIPPED: categoryinfo_alias_alias not accessible
    // SKIPPED: categoryinfo_bind_categoryId_view not accessible
    // SKIPPED: categoryinfo_bind_categoryId_view_mockputtopost not accessible
    // SKIPPED: categoryinfo_erase_category_id not accessible
    // SKIPPED: categoryinfo_erase_category_id_mockdeletetoget not accessible
    // SKIPPED: categoryinfo_extContent not accessible
    // SKIPPED: categoryinfo_filter_list_id_next_count_app_appId not accessible
    // SKIPPED: categoryinfo_filter_list_id_next_count_app_appId_mockputtopost not accessible
    // SKIPPED: categoryinfo_filter_list_id_prev_count_app_appId not accessible
    // SKIPPED: categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost not accessible
    // SKIPPED: categoryinfo_filter_list_page_size_size not accessible
    // SKIPPED: categoryinfo_filter_list_page_size_size_mockputtopost not accessible
    // SKIPPED: categoryinfo_list_all not accessible
    // SKIPPED: categoryinfo_list_manage_app_appId not accessible
    // SKIPPED: categoryinfo_list_objects not accessible
    // SKIPPED: categoryinfo_list_publish_app_appId not accessible
    // SKIPPED: categoryinfo_list_view_app_appId not accessible
    // SKIPPED: categoryinfo_list_view_app_appId_all not accessible
    // SKIPPED: categoryinfo_list_view_app_appId_data not accessible
    // SKIPPED: categoryinfo_flag not accessible
    // SKIPPED: categoryinfo_id not accessible
    // SKIPPED: categoryinfo_id_control not accessible
    // SKIPPED: categoryinfo_id_execute_projection not accessible
    // SKIPPED: categoryinfo_id_mockdeletetoget not accessible
    // SKIPPED: categoryinfo_id_permission not accessible
    // SKIPPED: commend_list_paging_page_size_size not accessible
    // SKIPPED: commend_id not accessible
    // SKIPPED: comment_list_id_next_count not accessible
    // SKIPPED: comment_list_id_next_count_mockputtopost not accessible
    // SKIPPED: comment_list_id_prev_count not accessible
    // SKIPPED: comment_list_id_prev_count_mockputtopost not accessible
    // SKIPPED: comment_list_page_size_size not accessible
    // SKIPPED: comment_list_page_size_size_mockputtopost not accessible
    // SKIPPED: comment_id not accessible
    // SKIPPED: comment_id_commend not accessible
    // SKIPPED: comment_id_mockdeletetoget not accessible
    // SKIPPED: comment_id_uncommend not accessible
    // SKIPPED: correlation_doc_docId not accessible
    // SKIPPED: correlation_doc_docId_delete not accessible
    // SKIPPED: correlation_list_doc_docId not accessible
    // SKIPPED: correlation_list_doc_docId_site_site not accessible
    // SKIPPED: correlation_update_doc_docId not accessible
    // SKIPPED: data_document_id not accessible
    // SKIPPED: data_document_id_array_data not accessible
    // SKIPPED: data_document_id_mockdeletetoget not accessible
    // SKIPPED: data_document_id_mockputtopost not accessible
    // SKIPPED: data_document_id_path0 not accessible
    // SKIPPED: data_document_id_path0_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1 not accessible
    // SKIPPED: data_document_id_path0_path1_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1_path2 not accessible
    // SKIPPED: data_document_id_path0_path1_path2_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_path2_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3 not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4 not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5 not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_path6 not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_path6_path7 not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget not accessible
    // SKIPPED: data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost not accessible
    // SKIPPED: design_appdict_list_appInfo_appId not accessible
    // SKIPPED: design_appdict_list_paging_page_size_size not accessible
    // SKIPPED: design_appdict_id not accessible
    // SKIPPED: design_appdict_id_mockdeletetoget not accessible
    // SKIPPED: design_appdict_id_mockputtopost not accessible
    // SKIPPED: designer_search not accessible
    // SKIPPED: document_cipher_filter_list_page_size_size not accessible
    // SKIPPED: document_cipher_filter_list_page_size_size_mockputtopost not accessible
    // SKIPPED: document_cipher_publish_content not accessible
    // SKIPPED: document_cipher_publish_content_mockputtopost not accessible
    // SKIPPED: document_cipher_id_permission_read_person_person not accessible
    // SKIPPED: document_cipher_id_persist_view_record not accessible
    // SKIPPED: file_list_appInfo_appInfoFlag not accessible
    // SKIPPED: file_list_id_next_count not accessible
    // SKIPPED: file_list_id_prev_count not accessible
    // SKIPPED: file_flag not accessible
    // SKIPPED: file_flag_appInfo_appInfoFlag not accessible
    // SKIPPED: file_flag_appInfo_appInfoFlag_content not accessible
    // SKIPPED: file_flag_appInfo_appInfoFlag_download not accessible
    // SKIPPED: file_flag_mockdeletetoget not accessible
    // SKIPPED: file_id not accessible
    // SKIPPED: file_id_content not accessible
    // SKIPPED: file_id_download not accessible
    // SKIPPED: file_id_mockputtopost not accessible
    // SKIPPED: file_id_upload not accessible
    // SKIPPED: anonymous_fileinfo_download_document_id not accessible
    // SKIPPED: anonymous_fileinfo_download_document_id_stream not accessible
    // SKIPPED: fileinfo_batch_download_doc_docId_site_site not accessible
    // SKIPPED: fileinfo_copy_to_doc_docId not accessible
    // SKIPPED: fileinfo_download_document_id not accessible
    // SKIPPED: fileinfo_download_document_id_stream not accessible
    // SKIPPED: fileinfo_download_transfer_flag_flag not accessible
    // SKIPPED: fileinfo_edit_id_doc_docId not accessible
    // SKIPPED: fileinfo_edit_id_doc_docId_mockputtopost not accessible
    // SKIPPED: fileinfo_list_all not accessible
    // SKIPPED: fileinfo_list_document_documentId not accessible
    // SKIPPED: fileinfo_list_filter not accessible
    // SKIPPED: fileinfo_replace_to_doc_docId not accessible
    // SKIPPED: fileinfo_update_document_docId_attachment_id not accessible
    // SKIPPED: fileinfo_update_document_docId_attachment_id_callback_callback not accessible
    // SKIPPED: fileinfo_update_id_content not accessible
    // SKIPPED: fileinfo_upload_doc_docId_save_as_flag not accessible
    // SKIPPED: fileinfo_upload_document_docId not accessible
    // SKIPPED: fileinfo_upload_document_docId_callback_callback not accessible
    // SKIPPED: fileinfo_upload_with_url not accessible
    // SKIPPED: fileinfo_id not accessible
    // SKIPPED: fileinfo_id_binary_base64_size not accessible
    // SKIPPED: fileinfo_id_doc_docId_change_seqnumber_seqNumber not accessible
    // SKIPPED: fileinfo_id_document_documentId not accessible
    // SKIPPED: fileinfo_id_mockdeletetoget not accessible
    // SKIPPED: fileinfo_id_online_info not accessible
    // SKIPPED: fileinfo_id_preview_pdf not accessible
    // SKIPPED: form_filter_list_id_next_count_app_appId not accessible
    // SKIPPED: form_filter_list_id_next_count_app_appId_mockputtopost not accessible
    // SKIPPED: form_filter_list_id_prev_count_app_appId not accessible
    // SKIPPED: form_filter_list_id_prev_count_app_appId_mockputtopost not accessible
    // SKIPPED: form_list_all not accessible
    // SKIPPED: form_list_app_appId not accessible
    // SKIPPED: form_list_formfield_appInfo_appId not accessible
    // SKIPPED: form_list_id_formfield not accessible
    // SKIPPED: anonymous_form_v2_lookup_document_docId not accessible
    // SKIPPED: anonymous_form_v2_lookup_document_docId_mobile not accessible
    // SKIPPED: anonymous_form_v2_id not accessible
    // SKIPPED: anonymous_form_v2_id_mobile not accessible
    // SKIPPED: anonymous_form_id not accessible
    // SKIPPED: form_formFlag_appinfo_appFlag not accessible
    // SKIPPED: form_id not accessible
    // SKIPPED: form_id_mockdeletetoget not accessible
    // SKIPPED: form_id_mockputtopost not accessible
    // SKIPPED: form_v2_lookup_document_docId not accessible
    // SKIPPED: form_v2_lookup_document_docId_mobile not accessible
    // SKIPPED: form_v2_id not accessible
    // SKIPPED: form_v2_id_mobile not accessible
    // SKIPPED: formversion_list_form_formId not accessible
    // SKIPPED: formversion_id not accessible
    // SKIPPED: log_filter_list_id_next_count not accessible
    // SKIPPED: log_filter_list_id_prev_count not accessible
    // SKIPPED: log_list_app_appId not accessible
    // SKIPPED: log_list_category_categoryId not accessible
    // SKIPPED: log_list_document_documentId not accessible
    // SKIPPED: log_list_filter_page_size_size not accessible
    // SKIPPED: log_list_level_operationLevel not accessible
    // SKIPPED: log_id not accessible
    // SKIPPED: output_list not accessible
    // SKIPPED: output_appInfoFlag_select not accessible
    // SKIPPED: output_appInfoFlag_select_mockputtopost not accessible
    // SKIPPED: permission_appInfo_id_manageable not accessible
    // SKIPPED: permission_appInfo_id_managers not accessible
    // SKIPPED: permission_appInfo_id_publishers not accessible
    // SKIPPED: permission_appInfo_id_viewers not accessible
    // SKIPPED: permission_category_id_managers not accessible
    // SKIPPED: permission_category_id_publishers not accessible
    // SKIPPED: permission_category_id_viewers not accessible
    // SKIPPED: permission_categoryInfo_id_manageable not accessible
    // SKIPPED: permission_management_refresh_all not accessible
    // SKIPPED: permission_management_refresh_category_categoryId not accessible
    // SKIPPED: permission_manager_appInfo_id not accessible
    // SKIPPED: permission_manager_categoryInfo_id not accessible
    // SKIPPED: permission_publisher_appInfo_id not accessible
    // SKIPPED: permission_publisher_categoryInfo_id not accessible
    // SKIPPED: permission_viewer_appInfo_id not accessible
    // SKIPPED: permission_viewer_categoryInfo_id not accessible
    // SKIPPED: review_v2_search not accessible
    // SKIPPED: script_list_app_appId_name_name not accessible
    // SKIPPED: script_list_app_flag not accessible
    // SKIPPED: script_list_manager not accessible
    // SKIPPED: script_list_paging_page_size_size not accessible
    // SKIPPED: script_list_id_next_count not accessible
    // SKIPPED: script_list_id_prev_count not accessible
    // SKIPPED: script_flag_appInfo_appInfoFlag not accessible
    // SKIPPED: script_id not accessible
    // SKIPPED: script_id_mockdeletetoget not accessible
    // SKIPPED: script_id_mockputtopost not accessible
    // SKIPPED: script_uniqueName_app_flag not accessible
    // SKIPPED: script_uniqueName_app_flag_imported not accessible
    // SKIPPED: scriptversion_list_script_scriptId not accessible
    // SKIPPED: scriptversion_id not accessible
    // SKIPPED: searchfilter_list_archive_filter_category_categoryId not accessible
    // SKIPPED: searchfilter_list_draft_filter_category_categoryId not accessible
    // SKIPPED: searchfilter_list_publish_filter_category_categoryId not accessible
    // SKIPPED: anonymous_surface_appdict_list_appInfo_appInfoFlag not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data not accessible
    // SKIPPED: anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data not accessible
    // SKIPPED: surface_appdict_list_appInfo_appInfoFlag not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_mockputtopost not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockputtopost not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockputtopost not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockputtopost not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget not accessible
    // SKIPPED: surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost not accessible
    // SKIPPED: templateform_list not accessible
    // SKIPPED: templateform_list_category not accessible
    // SKIPPED: templateform_list_category_mockputtopost not accessible
    // SKIPPED: templateform_id not accessible
    // SKIPPED: templateform_id_mockdeletetoget not accessible
    // SKIPPED: uuid_random not accessible
    // SKIPPED: view_list_all not accessible
    // SKIPPED: view_list_app_appId not accessible
    // SKIPPED: view_list_category_categoryId not accessible
    // SKIPPED: view_list_form_formId not accessible
    // SKIPPED: view_viewdata_list_id_next_count not accessible
    // SKIPPED: view_id not accessible
    // SKIPPED: view_id_mockdeletetoget not accessible
    // SKIPPED: view_id_mockputtopost not accessible
    // SKIPPED: viewcategory_list_all not accessible
    // SKIPPED: viewcategory_list_category_categoryId not accessible
    // SKIPPED: viewcategory_list_view_viewId not accessible
    // SKIPPED: viewcategory_id not accessible
    // SKIPPED: viewcategory_id_mockdeletetoget not accessible
    // SKIPPED: viewfieldconfig_list_all not accessible
    // SKIPPED: viewfieldconfig_list_view_viewId not accessible
    // SKIPPED: viewfieldconfig_id not accessible
    // SKIPPED: viewfieldconfig_id_mockdeletetoget not accessible
    // SKIPPED: viewfieldconfig_id_mockputtopost not accessible
    // SKIPPED: viewrecord_document_docId_filter_list_id_next_count not accessible
    // SKIPPED: viewrecord_document_docId_has_view not accessible
    // SKIPPED: viewrecord_list_install_log_paging_page_size_size not accessible
    // SKIPPED: image_encode_base64 not accessible
    // SKIPPED: image_encode_base64_size_size not accessible
    // SKIPPED: image_resize_id_id_width_width_height_height not accessible
    // SKIPPED: input_compare not accessible
    // SKIPPED: input_compare_mockputtopost not accessible
    // SKIPPED: input_cover not accessible
    // SKIPPED: input_cover_mockputtopost not accessible
    // SKIPPED: input_create not accessible
    // SKIPPED: input_create_mockputtopost not accessible
    // SKIPPED: input_prepare_cover not accessible
    // SKIPPED: input_prepare_cover_mockputtopost not accessible
    // SKIPPED: input_prepare_create not accessible
    // SKIPPED: input_prepare_create_mockputtopost not accessible
    #[tokio::test]
    async fn test_document_id_view_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/test-id/view/count")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_id_view_count route should be registered");
    }

    #[tokio::test]
    async fn test_commend_list_paging() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/commend/list/paging/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "commend_list_paging route should be registered");
    }

    #[tokio::test]
    async fn test_queryview_flag_definition() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/queryview/flag/test-id/definition/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "queryview_flag_definition route should be registered");
    }

}