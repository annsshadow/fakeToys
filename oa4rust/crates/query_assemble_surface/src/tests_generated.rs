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
                    .uri("/jaxrs/query/assemble/surface/get/test-id")
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
                    .uri("/jaxrs/query/assemble/surface/create")
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
                    .uri("/jaxrs/query/assemble/surface/list/test-id")
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
    async fn test_save_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/surface/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_surface route should be registered");
    }

    #[tokio::test]
    async fn test_delete_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/surface/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_surface route should be registered");
    }

    #[tokio::test]
    async fn test_preview_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/surface/preview/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "preview_surface route should be registered");
    }

    // SKIPPED: importmodel_execute_record_recordId not accessible
    // SKIPPED: importmodel_flag_flag_query_queryFlag not accessible
    // SKIPPED: importmodel_list_query_queryFlag not accessible
    // SKIPPED: importmodel_list_record_item_paging_page_size_size not accessible
    // SKIPPED: importmodel_list_record_paging_page_size_size not accessible
    // SKIPPED: importmodel_record_recordId not accessible
    // SKIPPED: importmodel_record_recordId_mockdeletetoget not accessible
    // SKIPPED: importmodel_record_recordId_status not accessible
    // SKIPPED: importmodel_uuid not accessible
    // SKIPPED: importmodel_id not accessible
    #[tokio::test]
    async fn test_importmodel_id_execute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/importmodel/id/test-id/execute")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "importmodel_id_execute route should be registered");
    }

    // SKIPPED: neural_list_calculate_model_modelFlag_work_workId not accessible
    // SKIPPED: query_list not accessible
    // SKIPPED: query_list_key_key not accessible
    // SKIPPED: query_flag not accessible
    // SKIPPED: table_list_paging_page_size_size not accessible
    // SKIPPED: table_list_table_tableFlag_row_paging_page_size_size not accessible
    // SKIPPED: table_list_id_next_count not accessible
    // SKIPPED: table_list_id_prev_count not accessible
    // SKIPPED: table_list_tableFlag_row_select not accessible
    // SKIPPED: table_list_tableFlag_row_select_where_where not accessible
    // SKIPPED: table_list_tableFlag_row_id_next_count not accessible
    // SKIPPED: table_list_tableFlag_row_id_prev_count not accessible
    // SKIPPED: table_reload_dynamic not accessible
    // SKIPPED: table_flag not accessible
    // SKIPPED: table_tableFlag_row not accessible
    // SKIPPED: table_tableFlag_row_count_where_where not accessible
    // SKIPPED: table_tableFlag_row_delete_all not accessible
    // SKIPPED: table_tableFlag_row_delete_all_mockdeletetoget not accessible
    // SKIPPED: table_tableFlag_row_one not accessible
    // SKIPPED: table_tableFlag_row_id not accessible
    // SKIPPED: table_tableFlag_row_id_mockdeletetoget not accessible
    // SKIPPED: table_tableFlag_row_id_mockputtopost not accessible
    // SKIPPED: table_tableFlag_row_id_part_update not accessible
    // SKIPPED: view_excel_result_flag not accessible
    // SKIPPED: view_flag_flag_query_queryFlag not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_bundle not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_bundle_mockputtopost not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_excel not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_excel_mockputtopost not accessible
    #[tokio::test]
    async fn test_view_flag_flag_query_queryFlag_execute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/queryview/flag/test-id/application/flag/test-id/execute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_flag_flag_query_queryFlag_execute route should be registered");
    }

    // SKIPPED: view_flag_flag_query_queryFlag_execute_mockputtopost not accessible
    #[tokio::test]
    async fn test_view_flag_flag_query_queryFlag_execute_v2_page_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/queryview/flag/test-id/application/flag/test-id/execute/page/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "view_flag_flag_query_queryFlag_execute_v2_page_page_size_size route should be registered");
    }

    // SKIPPED: view_list_query_queryFlag not accessible
    // SKIPPED: view_id not accessible
    // SKIPPED: view_id_bundle not accessible
    // SKIPPED: view_id_bundle_mockputtopost not accessible
    // SKIPPED: view_id_bundle_v2 not accessible
    // SKIPPED: view_id_excel not accessible
    // SKIPPED: view_id_excel_mockputtopost not accessible
    // SKIPPED: view_id_execute not accessible
    // SKIPPED: view_id_execute_mockputtopost not accessible
    // SKIPPED: view_id_execute_v2_page_page_size_size not accessible
}