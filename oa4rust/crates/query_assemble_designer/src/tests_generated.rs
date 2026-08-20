#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_designer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/designer/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_designer route should be registered");
    }

    #[tokio::test]
    async fn test_create_designer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/designer/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_designer route should be registered");
    }

    #[tokio::test]
    async fn test_list_designers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/designer/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_designers route should be registered");
    }

    #[tokio::test]
    async fn test_save_designer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/designer/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_designer route should be registered");
    }

    #[tokio::test]
    async fn test_delete_designer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/query/assemble/designer/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_designer route should be registered");
    }

    // SKIPPED: designer_search not accessible
    // SKIPPED: id_count not accessible
    // SKIPPED: importmodel_list_query_flag not accessible
    // SKIPPED: importmodel_id not accessible
    // SKIPPED: importmodel_id_permission not accessible
    // SKIPPED: input_compare not accessible
    // SKIPPED: input_cover not accessible
    // SKIPPED: input_create not accessible
    // SKIPPED: input_prepare_cover not accessible
    // SKIPPED: input_prepare_create not accessible
    // SKIPPED: neural_generate_model_modelFlag not accessible
    // SKIPPED: neural_learn_model_modelFlag not accessible
    // SKIPPED: neural_list_model not accessible
    // SKIPPED: neural_model not accessible
    // SKIPPED: neural_model_modelFlag not accessible
    // SKIPPED: neural_model_modelFlag_reset_status not accessible
    // SKIPPED: neural_stop_generating_model_modelFlag not accessible
    // SKIPPED: neural_stop_learn_model_modelFlag not accessible
    // SKIPPED: output_list not accessible
    // SKIPPED: output_flag_select_file not accessible
    // SKIPPED: output_queryFlag_select not accessible
    // SKIPPED: query_entity_entity_category_entityCategory_properties not accessible
    // SKIPPED: query_list_all not accessible
    // SKIPPED: query_list_querycategory_queryCategory not accessible
    // SKIPPED: query_list_summary not accessible
    // SKIPPED: query_list_summary_querycategory_queryCategory not accessible
    // SKIPPED: query_querycategory_list not accessible
    // SKIPPED: query_flag not accessible
    // SKIPPED: query_flag_icon not accessible
    // SKIPPED: query_id_permission not accessible
    // SKIPPED: stat_list_query_flag not accessible
    // SKIPPED: stat_list_id_next_count not accessible
    // SKIPPED: stat_list_id_prev_count not accessible
    // SKIPPED: stat_id not accessible
    // SKIPPED: stat_id_permission not accessible
    // SKIPPED: stat_id_simulate not accessible
    // SKIPPED: table_export_tableFlag_count_count not accessible
    // SKIPPED: table_list_manage not accessible
    // SKIPPED: table_list_query_flag not accessible
    // SKIPPED: table_list_tableFlag_row_select_where_where not accessible
    // SKIPPED: table_list_tableFlag_row_id_next_count not accessible
    // SKIPPED: table_list_tableFlag_row_id_prev_count not accessible
    // SKIPPED: table_query_query_build not accessible
    // SKIPPED: table_reload_dynamic not accessible
    // SKIPPED: table_flag not accessible
    // SKIPPED: table_flag_execute not accessible
    // SKIPPED: table_flag_status_build not accessible
    // SKIPPED: table_flag_status_draft not accessible
    // SKIPPED: table_id_permission not accessible
    // SKIPPED: table_query_build_dispatch not accessible
    // SKIPPED: table_tableFlag_row not accessible
    // SKIPPED: table_tableFlag_row_count_where_where not accessible
    // SKIPPED: table_tableFlag_row_delete_all not accessible
    // SKIPPED: table_tableFlag_row_save not accessible
    // SKIPPED: table_tableFlag_row_id not accessible
    // SKIPPED: view_list_query_flag not accessible
    // SKIPPED: view_list_id_next_count not accessible
    // SKIPPED: view_list_id_prev_count not accessible
    // SKIPPED: view_id not accessible
    // SKIPPED: view_id_bundle not accessible
    // SKIPPED: view_id_permission not accessible
    // SKIPPED: view_id_simulate not accessible
}