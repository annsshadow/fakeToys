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

    #[tokio::test]
    async fn test_anonymous_document_filter_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/document/filter/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_document_filter_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_document_filter_list_id_next_count_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/document/filter/list/test-id/next/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_document_filter_list_id_next_count_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_document_filter_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/document/filter/list/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_document_filter_list_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_document_filter_list_page_size_size_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/document/filter/list/test-id/size/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_document_filter_list_page_size_size_mockputtopost route should be registered");
    }

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

    #[tokio::test]
    async fn test_anonymous_fileinfo_list_document_documentId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/fileinfo/list/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_fileinfo_list_document_documentId route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_alias_alias() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/alias/test-id")
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
                    .uri("/jaxrs/appinfo/erase/app/test-id")
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
                    .uri("/jaxrs/appinfo/erase/app/test-id/mockdeletetoget")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_erase_app_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_filter_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/filter/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_filter_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_filter_list_id_next_count_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/filter/list/test-id/next/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_filter_list_id_next_count_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_filter_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/filter/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_filter_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_filter_list_id_prev_count_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/filter/list/test-id/prev/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_filter_list_id_prev_count_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_get_user_publish_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/get/user/publish/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_get_user_publish_appId route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/appType")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_appType_manager() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/appType/manager")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_appType_manager route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_has_document() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/has/document")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_has_document route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_has_document_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/has/document/appType")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_has_document_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_has_document_type_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/has/document/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_has_document_type_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/manage")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_manage route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_manage_type_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/manage/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_manage_type_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_publish() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/publish")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_publish route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_publish_type_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/publish/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_publish_type_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_publish_with_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/publish/with/process")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_publish_with_process route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_view() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/view")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_view route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_view_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/view/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_view_all route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_view_all_type_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/view/all/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_view_all_type_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_view_article_type_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/view/article/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_view_article_type_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_view_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/view/data")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_view_data route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_list_user_view_data_type_appType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/list/user/view/data/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_list_user_view_data_type_appType route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_appId_icon_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/test-id/icon/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_appId_icon_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/flag")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_flag route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "appinfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_appinfo_id_control() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/appinfo/test-id/control")
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
                    .uri("/jaxrs/appinfo/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/appinfo/test-id/permission")
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
                    .uri("/jaxrs/categoryinfo/alias/test-id")
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
                    .uri("/jaxrs/categoryinfo/bind/test-id/view")
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
                    .uri("/jaxrs/categoryinfo/bind/test-id/view/mockputtopost")
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
                    .uri("/jaxrs/categoryinfo/erase/category/test-id")
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
                    .uri("/jaxrs/categoryinfo/erase/category/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/categoryinfo/extContent")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_extContent route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_filter_list_id_next_count_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/filter/list/test-id/next/test-id/app/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_filter_list_id_next_count_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_filter_list_id_next_count_app_appId_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/filter/list/test-id/next/test-id/app/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_filter_list_id_next_count_app_appId_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_filter_list_id_prev_count_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/filter/list/test-id/prev/test-id/app/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_filter_list_id_prev_count_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/filter/list/test-id/prev/test-id/app/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_filter_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/filter/list/test-id/size/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_filter_list_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_filter_list_page_size_size_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/filter/list/test-id/size/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_filter_list_page_size_size_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_manage_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/manage/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_manage_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_objects() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/objects")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_objects route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_publish_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/publish/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_publish_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_view_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/view/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_view_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_view_app_appId_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/view/app/test-id/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_view_app_appId_all route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_list_view_app_appId_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/list/view/app/test-id/data")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_list_view_app_appId_data route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/flag")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_flag route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_categoryinfo_id_control() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/categoryinfo/test-id/control")
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
                    .uri("/jaxrs/categoryinfo/test-id/execute/projection")
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
                    .uri("/jaxrs/categoryinfo/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/categoryinfo/test-id/permission")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "categoryinfo_id_permission route should be registered");
    }

    #[tokio::test]
    async fn test_commend_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/commend/list/paging/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "commend_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_commend_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/commend/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "commend_id route should be registered");
    }

    #[tokio::test]
    async fn test_comment_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/list/test-id/next/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_comment_list_id_next_count_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/list/test-id/next/count/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_list_id_next_count_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_comment_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/list/test-id/prev/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_comment_list_id_prev_count_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/list/test-id/prev/count/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_list_id_prev_count_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_comment_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/list/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_list_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_comment_list_page_size_size_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/list/test-id/size/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_list_page_size_size_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_comment_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_id route should be registered");
    }

    #[tokio::test]
    async fn test_comment_id_commend() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/comment/test-id/commend")
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
                    .uri("/jaxrs/comment/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/comment/test-id/uncommend")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "comment_id_uncommend route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/doc/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_doc_docId route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_doc_docId_delete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/doc/test-id/delete")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_doc_docId_delete route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_list_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/list/doc/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_list_doc_docId route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_list_doc_docId_site_site() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/list/doc/test-id/site/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_list_doc_docId_site_site route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_update_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/correlation/update/doc/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_update_doc_docId route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_document_id route should be registered");
    }

    #[tokio::test]
    async fn test_data_document_id_array_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/data/document/test-id/array/data")
                    .method("POST")
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

    #[tokio::test]
    async fn test_design_appdict_list_appInfo_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/design/appdict/list/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_appdict_list_appInfo_appId route should be registered");
    }

    #[tokio::test]
    async fn test_design_appdict_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/design/appdict/list/paging/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_appdict_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_design_appdict_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/design/appdict/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "design_appdict_id route should be registered");
    }

    #[tokio::test]
    async fn test_design_appdict_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/design/appdict/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/design/appdict/test-id/mockputtopost")
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
    #[tokio::test]
    async fn test_document_cipher_filter_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/cipher/filter/list/test-id/size/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_filter_list_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_document_cipher_filter_list_page_size_size_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/cipher/filter/list/test-id/size/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_filter_list_page_size_size_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_document_cipher_publish_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/cipher/publish/content")
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
                    .uri("/jaxrs/document/cipher/publish/content/mockputtopost")
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
                    .uri("/jaxrs/document/cipher/test-id/permission/read/person/test-id")
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
                    .uri("/jaxrs/document/cipher/test-id/persist/view/record")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "document_cipher_id_persist_view_record route should be registered");
    }

    #[tokio::test]
    async fn test_file_list_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/list/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_list_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_file_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/list/test-id/next/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_file_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/list/test-id/prev/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/flag")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/flag/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag_appInfo_appInfoFlag_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/test-id/appInfo/test-id/content")
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
                    .uri("/jaxrs/file/test-id/appInfo/test-id/download")
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
                    .uri("/jaxrs/file/test-id/mockdeletetoget")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_file_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_id route should be registered");
    }

    #[tokio::test]
    async fn test_file_id_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/file/test-id/content")
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
                    .uri("/jaxrs/file/test-id/download")
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
                    .uri("/jaxrs/file/test-id/mockputtopost")
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
                    .uri("/jaxrs/file/test-id/upload")
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
                    .uri("/jaxrs/anonymous/fileinfo/download/document/test-id/stream")
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
                    .uri("/jaxrs/fileinfo/batch/download/doc/test-id/site/test-id")
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
                    .uri("/jaxrs/fileinfo/copy/to/doc/test-id")
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
                    .uri("/jaxrs/fileinfo/download/document/test-id/stream")
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
                    .uri("/jaxrs/fileinfo/download/transfer/flag/test-id")
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
                    .uri("/jaxrs/fileinfo/edit/test-id/doc/test-id")
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
                    .uri("/jaxrs/fileinfo/edit/test-id/doc/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_edit_id_doc_docId_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_list_document_documentId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/list/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_list_document_documentId route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_list_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/list/filter")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_list_filter route should be registered");
    }

    #[tokio::test]
    async fn test_fileinfo_replace_to_doc_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/fileinfo/replace/to/doc/test-id")
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
                    .uri("/jaxrs/fileinfo/update/document/test-id/attachment/test-id")
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
                    .uri("/jaxrs/fileinfo/update/document/test-id/attachment/test-id/callback/test-id")
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
                    .uri("/jaxrs/fileinfo/update/test-id/content")
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
                    .uri("/jaxrs/fileinfo/upload/doc/test-id/save/as/test-id")
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
                    .uri("/jaxrs/fileinfo/upload/document/test-id/callback/test-id")
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
                    .uri("/jaxrs/fileinfo/upload/with/url")
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
                    .uri("/jaxrs/fileinfo/test-id/binary/base64/test-id")
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
                    .uri("/jaxrs/fileinfo/test-id/doc/test-id/change/seqnumber/test-id")
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
                    .uri("/jaxrs/fileinfo/test-id/online/info")
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
                    .uri("/jaxrs/fileinfo/test-id/preview/pdf")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fileinfo_id_preview_pdf route should be registered");
    }

    #[tokio::test]
    async fn test_form_filter_list_id_next_count_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/filter/list/test-id/next/count/app/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_filter_list_id_next_count_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_form_filter_list_id_next_count_app_appId_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/filter/list/test-id/next/count/app/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_filter_list_id_next_count_app_appId_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_form_filter_list_id_prev_count_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/filter/list/test-id/prev/count/app/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_filter_list_id_prev_count_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_form_filter_list_id_prev_count_app_appId_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/filter/list/test-id/prev/count/app/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_filter_list_id_prev_count_app_appId_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_form_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_form_list_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/list/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_list_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_form_list_formfield_appInfo_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/list/formfield/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_list_formfield_appInfo_appId route should be registered");
    }

    #[tokio::test]
    async fn test_form_list_id_formfield() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/list/test-id/formfield")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_list_id_formfield route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_form_v2_lookup_document_docId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/form/v2/lookup/document/test-id")
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
                    .uri("/jaxrs/anonymous/form/v2/lookup/document/test-id/mobile")
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
                    .uri("/jaxrs/anonymous/form/v2/test-id")
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
                    .uri("/jaxrs/anonymous/form/v2/test-id/mobile")
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
                    .uri("/jaxrs/anonymous/form/test-id")
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
                    .uri("/jaxrs/form/test-id/appinfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_formFlag_appinfo_appFlag route should be registered");
    }

    #[tokio::test]
    async fn test_form_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_id route should be registered");
    }

    #[tokio::test]
    async fn test_form_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/form/test-id/mockputtopost")
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
                    .uri("/jaxrs/form/v2/lookup/document/test-id")
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
                    .uri("/jaxrs/form/v2/lookup/document/test-id/mobile")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_document_docId_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/v2/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_id route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_id_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/form/v2/test-id/mobile")
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
                    .uri("/jaxrs/formversion/list/form/test-id")
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
                    .uri("/jaxrs/formversion/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "formversion_id route should be registered");
    }

    #[tokio::test]
    async fn test_log_filter_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/filter/list/test-id/next/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_filter_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_log_filter_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/filter/list/test-id/prev/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_filter_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_log_list_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/list/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_list_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_log_list_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/list/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_list_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_log_list_document_documentId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/list/document/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_list_document_documentId route should be registered");
    }

    #[tokio::test]
    async fn test_log_list_filter_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/list/filter/test-id/size/test-id")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_list_filter_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_log_list_level_operationLevel() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/list/level/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_list_level_operationLevel route should be registered");
    }

    #[tokio::test]
    async fn test_log_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/log/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "log_id route should be registered");
    }

    #[tokio::test]
    async fn test_output_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/output/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "output_list route should be registered");
    }

    #[tokio::test]
    async fn test_output_appInfoFlag_select() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/output/test-id/select")
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
                    .uri("/jaxrs/output/test-id/select/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "output_appInfoFlag_select_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_permission_appInfo_id_manageable() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/appInfo/test-id/manageable")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_appInfo_id_manageable route should be registered");
    }

    #[tokio::test]
    async fn test_permission_appInfo_id_managers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/appInfo/test-id/managers")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_appInfo_id_managers route should be registered");
    }

    #[tokio::test]
    async fn test_permission_appInfo_id_publishers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/appInfo/test-id/publishers")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_appInfo_id_publishers route should be registered");
    }

    #[tokio::test]
    async fn test_permission_appInfo_id_viewers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/appInfo/test-id/viewers")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_appInfo_id_viewers route should be registered");
    }

    #[tokio::test]
    async fn test_permission_category_id_managers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/category/test-id/managers")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_category_id_managers route should be registered");
    }

    #[tokio::test]
    async fn test_permission_category_id_publishers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/category/test-id/publishers")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_category_id_publishers route should be registered");
    }

    #[tokio::test]
    async fn test_permission_category_id_viewers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/category/test-id/viewers")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_category_id_viewers route should be registered");
    }

    #[tokio::test]
    async fn test_permission_categoryInfo_id_manageable() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/categoryInfo/test-id/manageable")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_categoryInfo_id_manageable route should be registered");
    }

    #[tokio::test]
    async fn test_permission_management_refresh_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/management/refresh/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_management_refresh_all route should be registered");
    }

    #[tokio::test]
    async fn test_permission_management_refresh_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/management/refresh/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_management_refresh_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_permission_manager_appInfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/manager/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_manager_appInfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_permission_manager_categoryInfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/manager/categoryInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_manager_categoryInfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_permission_publisher_appInfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/publisher/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_publisher_appInfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_permission_publisher_categoryInfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/publisher/categoryInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_publisher_categoryInfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_permission_viewer_appInfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/viewer/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_viewer_appInfo_id route should be registered");
    }

    #[tokio::test]
    async fn test_permission_viewer_categoryInfo_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/permission/viewer/categoryInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "permission_viewer_categoryInfo_id route should be registered");
    }

    // SKIPPED: review_v2_search not accessible
    #[tokio::test]
    async fn test_script_list_app_appId_name_name() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/list/app/test-id/name/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list_app_appId_name_name route should be registered");
    }

    #[tokio::test]
    async fn test_script_list_app_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/list/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list_app_flag route should be registered");
    }

    #[tokio::test]
    async fn test_script_list_manager() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/list/manager")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list_manager route should be registered");
    }

    #[tokio::test]
    async fn test_script_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/list/paging/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_script_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/list/test-id/next/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_script_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/list/test-id/prev/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_script_flag_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/flag/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_flag_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_script_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_id route should be registered");
    }

    #[tokio::test]
    async fn test_script_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/script/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/script/test-id/mockputtopost")
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
                    .uri("/jaxrs/script/test-id/app/test-id")
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
                    .uri("/jaxrs/script/test-id/app/test-id/imported")
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
                    .uri("/jaxrs/scriptversion/list/script/test-id")
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
                    .uri("/jaxrs/scriptversion/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "scriptversion_id route should be registered");
    }

    #[tokio::test]
    async fn test_searchfilter_list_archive_filter_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/searchfilter/list/archive/filter/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "searchfilter_list_archive_filter_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_searchfilter_list_draft_filter_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/searchfilter/list/draft/filter/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "searchfilter_list_draft_filter_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_searchfilter_list_publish_filter_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/searchfilter/list/publish/filter/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "searchfilter_list_publish_filter_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_list_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/surface/appdict/list/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_list_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/data")
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
                    .uri("/jaxrs/anonymous/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/p7/data")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_list_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/surface/appdict/list/appInfo/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_list_appInfo_appInfoFlag route should be registered");
    }

    #[tokio::test]
    async fn test_surface_appdict_appDictFlag_appInfo_appInfoFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/mockputtopost")
                    .method("POST")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/data/mockputtopost")
                    .method("POST")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/data/mockputtopost")
                    .method("POST")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/data/mockputtopost")
                    .method("POST")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/data/mockputtopost")
                    .method("POST")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/data/mockputtopost")
                    .method("POST")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/p7/data")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/p7/data/mockdeletetoget")
                    .method("GET")
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
                    .uri("/jaxrs/surface/appdict/dict-flag/appInfo/app-flag/p0/p1/p2/p3/p4/p5/p6/p7/data/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_templateform_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/templateform/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "templateform_list route should be registered");
    }

    #[tokio::test]
    async fn test_templateform_list_category() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/templateform/list/category")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "templateform_list_category route should be registered");
    }

    #[tokio::test]
    async fn test_templateform_list_category_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/templateform/list/category/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "templateform_list_category_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_templateform_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/templateform/test-id")
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
                    .uri("/jaxrs/templateform/test-id/mockdeletetoget")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "templateform_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_uuid_random() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/uuid/random")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "uuid_random route should be registered");
    }

    #[tokio::test]
    async fn test_view_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_view_list_app_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/list/app/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_list_app_appId route should be registered");
    }

    #[tokio::test]
    async fn test_view_list_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/list/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_list_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_view_list_form_formId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/list/form/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_list_form_formId route should be registered");
    }

    #[tokio::test]
    async fn test_view_viewdata_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/viewdata/list/test-id/next/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_viewdata_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_view_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_id route should be registered");
    }

    #[tokio::test]
    async fn test_view_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/view/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/view/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_viewcategory_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewcategory/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewcategory_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_viewcategory_list_category_categoryId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewcategory/list/category/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewcategory_list_category_categoryId route should be registered");
    }

    #[tokio::test]
    async fn test_viewcategory_list_view_viewId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewcategory/list/view/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewcategory_list_view_viewId route should be registered");
    }

    #[tokio::test]
    async fn test_viewcategory_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewcategory/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewcategory_id route should be registered");
    }

    #[tokio::test]
    async fn test_viewcategory_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewcategory/test-id/mockdeletetoget")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewcategory_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_viewfieldconfig_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewfieldconfig/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewfieldconfig_list_all route should be registered");
    }

    #[tokio::test]
    async fn test_viewfieldconfig_list_view_viewId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewfieldconfig/list/view/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewfieldconfig_list_view_viewId route should be registered");
    }

    #[tokio::test]
    async fn test_viewfieldconfig_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewfieldconfig/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewfieldconfig_id route should be registered");
    }

    #[tokio::test]
    async fn test_viewfieldconfig_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewfieldconfig/test-id/mockdeletetoget")
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
                    .uri("/jaxrs/viewfieldconfig/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewfieldconfig_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_viewrecord_document_docId_filter_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewrecord/document/test-id/filter/list/test-id/next/count")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewrecord_document_docId_filter_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_viewrecord_document_docId_has_view() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewrecord/document/test-id/has/view")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewrecord_document_docId_has_view route should be registered");
    }

    #[tokio::test]
    async fn test_viewrecord_list_install_log_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/viewrecord/list/install/log/paging/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "viewrecord_list_install_log_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_image_encode_base64() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/image/encode/base64")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "image_encode_base64 route should be registered");
    }

    #[tokio::test]
    async fn test_image_encode_base64_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/image/encode/base64/size/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "image_encode_base64_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_image_resize_id_id_width_width_height_height() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/image/resize/id/test-id/width/test-id/height/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "image_resize_id_id_width_width_height_height route should be registered");
    }

    #[tokio::test]
    async fn test_input_compare() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/compare")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_compare route should be registered");
    }

    #[tokio::test]
    async fn test_input_compare_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/compare/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_compare_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_input_cover() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/cover")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_cover route should be registered");
    }

    #[tokio::test]
    async fn test_input_cover_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/cover/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_cover_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_input_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/create")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_create route should be registered");
    }

    #[tokio::test]
    async fn test_input_create_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/create/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_create_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_input_prepare_cover() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/prepare/cover")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_prepare_cover route should be registered");
    }

    #[tokio::test]
    async fn test_input_prepare_cover_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/prepare/cover/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_prepare_cover_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_input_prepare_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/prepare/create")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_prepare_create route should be registered");
    }

    #[tokio::test]
    async fn test_input_prepare_create_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/input/prepare/create/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "input_prepare_create_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_document_id_view_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        // 该端点对不存在的文档返回 AppError::NotFound(404)；
        // 先种入目标行，避免依赖外部数据库状态。
        if let Ok(client) = pool.get().await {
            let _ = client
                .execute(
                    "INSERT INTO x_cms_document (id) VALUES ('test-id') ON CONFLICT (id) DO NOTHING",
                    &[],
                )
                .await;
        }
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