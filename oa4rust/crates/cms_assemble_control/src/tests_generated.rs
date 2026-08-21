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
    #[tokio::test]
    async fn test_anonymous_document_id_view() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/document/test-id/view")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_document_id_view route should be registered");
    }

    // SKIPPED: anonymous_fileinfo_list_document_documentId not accessible
    #[tokio::test]
    async fn test_appinfo_alias_alias() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/alias/alias")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_alias_alias route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_erase_app_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/erase/app/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_erase_app_id route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_erase_app_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/erase/app/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_erase_app_id_mockdeletetoget route should be registered");
    }

    // SKIPPED: appinfo_filter_list_id_next_count not accessible
    // SKIPPED: appinfo_filter_list_id_next_count_mockputtopost not accessible
    // SKIPPED: appinfo_filter_list_id_prev_count not accessible
    // SKIPPED: appinfo_filter_list_id_prev_count_mockputtopost not accessible
    #[tokio::test]
    async fn test_appinfo_get_user_publish_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/get/user/publish/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_get_user_publish_appId route should be registered");
    }

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
    #[tokio::test]
    async fn test_appinfo_appId_icon_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/icon/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_appId_icon_size_size route should be registered");
    }

    // SKIPPED: appinfo_flag not accessible
    // SKIPPED: appinfo_id not accessible
    #[tokio::test]
    async fn test_appinfo_id_control() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/control/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_id_control route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_id_permission() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/appinfo/permission/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_id_permission route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_alias_alias() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/alias/alias")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_alias_alias route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_bind_categoryId_view() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/bind/view/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_bind_categoryId_view route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_bind_categoryId_view_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/bind/view/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_bind_categoryId_view_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_erase_category_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/erase/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_erase_category_id route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_erase_category_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/erase/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_erase_category_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_extContent() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_extContent route should be registered");
    }

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
    #[tokio::test]
    async fn test_categoryinfo_id_control() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/control/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_id_control route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_id_execute_projection() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/execute/projection/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_id_execute_projection route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_id_permission() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/categoryinfo/permission/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_id_permission route should be registered");
    }

    // SKIPPED: commend_list_paging_page_size_size not accessible
    // SKIPPED: commend_id not accessible
    // SKIPPED: comment_list_id_next_count not accessible
    // SKIPPED: comment_list_id_next_count_mockputtopost not accessible
    // SKIPPED: comment_list_id_prev_count not accessible
    // SKIPPED: comment_list_id_prev_count_mockputtopost not accessible
    // SKIPPED: comment_list_page_size_size not accessible
    // SKIPPED: comment_list_page_size_size_mockputtopost not accessible
    // SKIPPED: comment_id not accessible
    #[tokio::test]
    async fn test_comment_id_commend() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/comment/commend/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_id_commend route should be registered");
    }

    #[tokio::test]
    async fn test_comment_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/comment/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_comment_id_uncommend() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/comment/uncommend/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_id_uncommend route should be registered");
    }

    // SKIPPED: correlation_doc_docId not accessible
    #[tokio::test]
    async fn test_correlation_doc_docId_delete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/correlation/doc/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_doc_docId_delete route should be registered");
    }

    // SKIPPED: correlation_list_doc_docId not accessible
    // SKIPPED: correlation_list_doc_docId_site_site not accessible
    #[tokio::test]
    async fn test_correlation_update_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/correlation/update/doc/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_update_doc_docId route should be registered");
    }

    // SKIPPED: data_document_id not accessible
    #[tokio::test]
    async fn test_data_document_id_array_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/array/data")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_array_data route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_path6() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/path6")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_path6 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/path6/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/path6/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_path6_path7() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/path6/path7")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_path6_path7 route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/path6/path7/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost route should be registered");
    }

    // SKIPPED: design_appdict_list_appInfo_appId not accessible
    // SKIPPED: design_appdict_list_paging_page_size_size not accessible
    // SKIPPED: design_appdict_id not accessible
    #[tokio::test]
    async fn test_design_appdict_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/design/appdict/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_appdict_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_design_appdict_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/design/appdict/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_appdict_id_mockputtopost route should be registered");
    }

    // SKIPPED: designer_search not accessible
    // SKIPPED: document_cipher_filter_list_page_size_size not accessible
    // SKIPPED: document_cipher_filter_list_page_size_size_mockputtopost not accessible
    #[tokio::test]
    async fn test_document_cipher_publish_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/document/cipher/publish/content")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_publish_content route should be registered");
    }

    #[tokio::test]
    async fn test_document_cipher_publish_content_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/document/cipher/publish/content/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_publish_content_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_document_cipher_id_permission_read_person_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/document/cipher/permission/read/person/person/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_id_permission_read_person_person route should be registered");
    }

    #[tokio::test]
    async fn test_document_cipher_id_persist_view_record() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/document/cipher/persist/view/record/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_id_persist_view_record route should be registered");
    }

    // SKIPPED: file_list_appInfo_appInfoFlag not accessible
    // SKIPPED: file_list_id_next_count not accessible
    // SKIPPED: file_list_id_prev_count not accessible
    // SKIPPED: file_flag not accessible
    // SKIPPED: file_flag_appInfo_appInfoFlag not accessible
    #[tokio::test]
    async fn test_file_flag_appInfo_appInfoFlag_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/content/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_appInfo_appInfoFlag_content route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag_appInfo_appInfoFlag_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/download/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_appInfo_appInfoFlag_download route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_mockdeletetoget route should be registered");
    }

    // SKIPPED: file_id not accessible
    #[tokio::test]
    async fn test_file_id_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/content/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_id_content route should be registered");
    }

    #[tokio::test]
    async fn test_file_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/download/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_id_download route should be registered");
    }

    #[tokio::test]
    async fn test_file_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_file_id_upload() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/file/upload/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_id_upload route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_fileinfo_download_document_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/fileinfo/download/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_fileinfo_download_document_id route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_fileinfo_download_document_id_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/fileinfo/download/document/stream/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_fileinfo_download_document_id_stream route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_batch_download_doc_docId_site_site() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/batch/download/doc/site/site/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_batch_download_doc_docId_site_site route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_copy_to_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/copy/to/doc/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_copy_to_doc_docId route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_download_document_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/download/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_download_document_id route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_download_document_id_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/download/document/stream/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_download_document_id_stream route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_download_transfer_flag_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/download/transfer/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_download_transfer_flag_flag route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_edit_id_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/edit/doc/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_edit_id_doc_docId route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_edit_id_doc_docId_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/edit/doc/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_edit_id_doc_docId_mockputtopost route should be registered");
    }

    // SKIPPED: fileinfo_list_all not accessible
    // SKIPPED: fileinfo_list_document_documentId not accessible
    // SKIPPED: fileinfo_list_filter not accessible
    #[tokio::test]
    async fn test_fileinfo_replace_to_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/replace/to/doc/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_replace_to_doc_docId route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_update_document_docId_attachment_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/update/document/attachment/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_update_document_docId_attachment_id route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_update_document_docId_attachment_id_callback_callback() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/update/document/attachment/callback/callback/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_update_document_docId_attachment_id_callback_callback route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_update_id_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/update/content/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_update_id_content route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_upload_doc_docId_save_as_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/upload/doc/save/as/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_upload_doc_docId_save_as_flag route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_upload_document_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/upload/document/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_upload_document_docId route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_upload_document_docId_callback_callback() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/upload/document/callback/callback/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_upload_document_docId_callback_callback route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_upload_with_url() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/upload/with/url")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_upload_with_url route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id_binary_base64_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/binary/base64/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_binary_base64_size route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id_doc_docId_change_seqnumber_seqNumber() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/doc/change/seqnumber/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_doc_docId_change_seqnumber_seqNumber route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id_document_documentId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/test-id/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_document_documentId route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/test-id/mockdeletetoget")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id_online_info() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/online/info/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_online_info route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_id_preview_pdf() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/fileinfo/preview/pdf/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_preview_pdf route should be registered");
    }

    // SKIPPED: form_filter_list_id_next_count_app_appId not accessible
    // SKIPPED: form_filter_list_id_next_count_app_appId_mockputtopost not accessible
    // SKIPPED: form_filter_list_id_prev_count_app_appId not accessible
    // SKIPPED: form_filter_list_id_prev_count_app_appId_mockputtopost not accessible
    // SKIPPED: form_list_all not accessible
    // SKIPPED: form_list_app_appId not accessible
    // SKIPPED: form_list_formfield_appInfo_appId not accessible
    // SKIPPED: form_list_id_formfield not accessible
    #[tokio::test]
    async fn test_anonymous_form_v2_lookup_document_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/form/v2/lookup/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_form_v2_lookup_document_docId route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_form_v2_lookup_document_docId_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/form/v2/lookup/document/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_form_v2_lookup_document_docId_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_form_v2_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/form/v2/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_form_v2_id route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_form_v2_id_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/form/v2/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_form_v2_id_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_form_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/form/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_form_id route should be registered");
    }

    #[tokio::test]
    async fn test_form_formFlag_appinfo_appFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/form/appinfo/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_formFlag_appinfo_appFlag route should be registered");
    }

    // SKIPPED: form_id not accessible
    #[tokio::test]
    async fn test_form_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/form/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_form_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/form/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_lookup_document_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/form/v2/lookup/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_document_docId route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_lookup_document_docId_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/form/v2/lookup/document/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_document_docId_mobile route should be registered");
    }

    // SKIPPED: form_v2_id not accessible
    #[tokio::test]
    async fn test_form_v2_id_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/form/v2/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_id_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_formversion_list_form_formId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/formversion/list/form/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "formversion_list_form_formId route should be registered");
    }

    #[tokio::test]
    async fn test_formversion_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/formversion/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "formversion_id route should be registered");
    }

    // SKIPPED: log_filter_list_id_next_count not accessible
    // SKIPPED: log_filter_list_id_prev_count not accessible
    // SKIPPED: log_list_app_appId not accessible
    // SKIPPED: log_list_category_categoryId not accessible
    // SKIPPED: log_list_document_documentId not accessible
    // SKIPPED: log_list_filter_page_size_size not accessible
    // SKIPPED: log_list_level_operationLevel not accessible
    // SKIPPED: log_id not accessible
    // SKIPPED: output_list not accessible
    #[tokio::test]
    async fn test_output_appInfoFlag_select() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/output/select/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "output_appInfoFlag_select route should be registered");
    }

    #[tokio::test]
    async fn test_output_appInfoFlag_select_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/output/select/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "output_appInfoFlag_select_mockputtopost route should be registered");
    }

    // SKIPPED: permission_appInfo_id_manageable not accessible
    // SKIPPED: permission_appInfo_id_managers not accessible
    // SKIPPED: permission_appInfo_id_publishers not accessible
    // SKIPPED: permission_appInfo_id_viewers not accessible
    // SKIPPED: permission_category_id_managers not accessible
    // SKIPPED: permission_category_id_publishers not accessible
    // SKIPPED: permission_category_id_viewers not accessible
    // SKIPPED: permission_categoryInfo_id_manageable not accessible
    // SKIPPED: permission_management_refresh_all not accessible
    #[tokio::test]
    async fn test_permission_management_refresh_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/permission/management/refresh/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_management_refresh_category_categoryId route should be registered");
    }

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
    #[tokio::test]
    async fn test_script_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/script/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_script_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/script/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_script_uniqueName_app_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/script/app/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_uniqueName_app_flag route should be registered");
    }

    #[tokio::test]
    async fn test_script_uniqueName_app_flag_imported() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/script/app/imported/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_uniqueName_app_flag_imported route should be registered");
    }

    #[tokio::test]
    async fn test_scriptversion_list_script_scriptId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/scriptversion/list/script/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "scriptversion_list_script_scriptId route should be registered");
    }

    #[tokio::test]
    async fn test_scriptversion_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/scriptversion/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "scriptversion_id route should be registered");
    }

    // SKIPPED: searchfilter_list_archive_filter_category_categoryId not accessible
    // SKIPPED: searchfilter_list_draft_filter_category_categoryId not accessible
    // SKIPPED: searchfilter_list_publish_filter_category_categoryId not accessible
    // SKIPPED: anonymous_surface_appdict_list_appInfo_appInfoFlag not accessible
    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/path5/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/anonymous/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data route should be registered");
    }

    // SKIPPED: surface_appdict_list_appInfo_appInfoFlag not accessible
    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/data/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/data/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/data/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/data/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/data/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/mockdeletetoget/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/surface/appdict/path0/path1/path2/path3/path4/path5/path6/path7/data/mockputtopost/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost route should be registered");
    }

    // SKIPPED: templateform_list not accessible
    // SKIPPED: templateform_list_category not accessible
    // SKIPPED: templateform_list_category_mockputtopost not accessible
    #[tokio::test]
    async fn test_templateform_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/templateform/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "templateform_id route should be registered");
    }

    #[tokio::test]
    async fn test_templateform_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/templateform/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "templateform_id_mockdeletetoget route should be registered");
    }

    // SKIPPED: uuid_random not accessible
    // SKIPPED: view_list_all not accessible
    // SKIPPED: view_list_app_appId not accessible
    // SKIPPED: view_list_category_categoryId not accessible
    // SKIPPED: view_list_form_formId not accessible
    // SKIPPED: view_viewdata_list_id_next_count not accessible
    // SKIPPED: view_id not accessible
    #[tokio::test]
    async fn test_view_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/view/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_view_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/view/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_id_mockputtopost route should be registered");
    }

    // SKIPPED: viewcategory_list_all not accessible
    // SKIPPED: viewcategory_list_category_categoryId not accessible
    // SKIPPED: viewcategory_list_view_viewId not accessible
    // SKIPPED: viewcategory_id not accessible
    #[tokio::test]
    async fn test_viewcategory_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/viewcategory/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewcategory_id_mockdeletetoget route should be registered");
    }

    // SKIPPED: viewfieldconfig_list_all not accessible
    // SKIPPED: viewfieldconfig_list_view_viewId not accessible
    // SKIPPED: viewfieldconfig_id not accessible
    #[tokio::test]
    async fn test_viewfieldconfig_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/viewfieldconfig/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewfieldconfig_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_viewfieldconfig_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/viewfieldconfig/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewfieldconfig_id_mockputtopost route should be registered");
    }

    // SKIPPED: viewrecord_document_docId_filter_list_id_next_count not accessible
    #[tokio::test]
    async fn test_viewrecord_document_docId_has_view() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms/assemble/control/viewrecord/document/has/view/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewrecord_document_docId_has_view route should be registered");
    }

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
    async fn test_document_search() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/document/search")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_search route should be registered");
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