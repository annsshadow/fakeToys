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
                    .uri("/api/query/assemble/surface/get/test-id")
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
    // SKIPPED: save_surface requires Session parameter
    #[tokio::test]
    async fn test_importmodel_uuid() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/queryview/importmodel/uuid")
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
                "importmodel_uuid route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_query_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/queryview/query/list")
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
            assert!(!bytes.is_empty(), "query_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_view_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/queryview/view/test-id")
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
            assert!(!bytes.is_empty(), "view_id route should be registered");
        }
    }

    // SKIPPED: list_surfaces not accessible
    // SKIPPED: explorer_list not accessible
    // SKIPPED: delete_surface requires Session parameter
    // SKIPPED: preview_surface not accessible
    // SKIPPED: importmodel_execute_record_recordId not accessible
    // SKIPPED: importmodel_flag_flag_query_queryFlag not accessible
    // SKIPPED: importmodel_flag_flag_query_queryFlag_p4 not accessible
    // SKIPPED: importmodel_list_query_queryFlag not accessible
    // SKIPPED: importmodel_list_query_queryFlag_p2 not accessible
    // SKIPPED: importmodel_list_record_item_paging_page_size_size not accessible
    // SKIPPED: importmodel_list_record_item_paging_page_size_size_p3 not accessible
    // SKIPPED: importmodel_list_record_paging_page_size_size not accessible
    // SKIPPED: importmodel_list_record_paging_page_size_size_p3 not accessible
    // SKIPPED: importmodel_record_recordId not accessible
    // SKIPPED: importmodel_record_recordId_mockdeletetoget not accessible
    // SKIPPED: importmodel_record_recordId_status not accessible
    // SKIPPED: importmodel_record_recordId_status_p2 not accessible
    // SKIPPED: importmodel_id not accessible
    // SKIPPED: importmodel_id_execute requires Session parameter
    // SKIPPED: neural_list_calculate_model_modelFlag_work_workId not accessible
    // SKIPPED: neural_list_calculate_model_modelFlag_work_workId_p3 not accessible
    // SKIPPED: query_list_key_key not accessible
    // SKIPPED: query_list_key_key_p3 not accessible
    // SKIPPED: query_flag not accessible
    // SKIPPED: query_flag_p2 not accessible
    // SKIPPED: table_list_paging_page_size_size not accessible
    // SKIPPED: table_list_paging_page_size_size_p3 not accessible
    // SKIPPED: table_list_table_tableFlag_row_paging_page_size_size not accessible
    // SKIPPED: table_list_table_tableFlag_row_paging_page_size_size_p4 not accessible
    // SKIPPED: table_list_id_next_count not accessible
    // SKIPPED: table_list_id_next_count_p3 not accessible
    // SKIPPED: table_list_id_prev_count not accessible
    // SKIPPED: table_list_tableFlag_row_select not accessible
    // SKIPPED: table_list_tableFlag_row_select_where_where not accessible
    // SKIPPED: table_list_tableFlag_row_id_next_count not accessible
    // SKIPPED: table_list_tableFlag_row_id_next_count_p4 not accessible
    // SKIPPED: table_list_tableFlag_row_id_prev_count not accessible
    // SKIPPED: table_reload_dynamic requires Session parameter
    // SKIPPED: table_flag not accessible
    // SKIPPED: table_tableFlag_row not accessible
    // SKIPPED: table_tableFlag_row_count_where_where not accessible
    // SKIPPED: table_tableFlag_row_delete_all requires Session parameter
    // SKIPPED: table_tableFlag_row_delete_all_mockdeletetoget not accessible
    // SKIPPED: table_tableFlag_row_one not accessible
    // SKIPPED: table_tableFlag_row_id not accessible
    // SKIPPED: table_tableFlag_row_id_mockdeletetoget not accessible
    // SKIPPED: table_tableFlag_row_id_mockputtopost requires Session parameter
    // SKIPPED: table_tableFlag_row_id_part_update requires Session parameter
    // SKIPPED: view_excel_result_flag not accessible
    // SKIPPED: view_excel_result_flag_p2 not accessible
    // SKIPPED: view_flag_flag_query_queryFlag not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_p5 not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_bundle not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_bundle_mockputtopost requires Session parameter
    // SKIPPED: view_flag_flag_query_queryFlag_excel not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_excel_mockputtopost requires Session parameter
    // SKIPPED: view_flag_flag_query_queryFlag_execute not accessible
    // SKIPPED: view_flag_flag_query_queryFlag_execute_mockputtopost requires Session parameter
    // SKIPPED: view_flag_flag_query_queryFlag_execute_v2_page_page_size_size not accessible
    // SKIPPED: view_list_query_queryFlag not accessible
    // SKIPPED: view_id_bundle not accessible
    // SKIPPED: view_id_bundle_mockputtopost requires Session parameter
    // SKIPPED: view_id_bundle_v2 not accessible
    // SKIPPED: view_id_excel not accessible
    // SKIPPED: view_id_excel_mockputtopost requires Session parameter
    // SKIPPED: view_id_execute not accessible
    // SKIPPED: view_id_execute_mockputtopost requires Session parameter
    // SKIPPED: view_id_execute_v2_page_page_size_size not accessible
    // SKIPPED: statement_get_id not accessible
    // SKIPPED: statement_get_format not accessible
    // SKIPPED: statement_list_with_query not accessible
    // SKIPPED: statement_execute not accessible
    // SKIPPED: statement_execute_mode_v2 not accessible
    // SKIPPED: stat_get_id not accessible
    // SKIPPED: stat_get_with_query not accessible
    // SKIPPED: stat_list_with_query not accessible
    // SKIPPED: stat_execute not accessible
    // SKIPPED: search_post not accessible
    // SKIPPED: morelikethis_post not accessible
    // SKIPPED: table_row_delete requires Session parameter
    // SKIPPED: table_row_insert requires Session parameter
    // SKIPPED: table_row_insert_one requires Session parameter
    // SKIPPED: importmodel_record_delete requires Session parameter
    // SKIPPED: importmodel_reexecute_record requires Session parameter
    // SKIPPED: view_bundle_v2_post not accessible
    // SKIPPED: stat_execute_with_query_put not accessible
    // SKIPPED: stat_execute_with_query_mock not accessible
    // SKIPPED: view_flag_query_bundle_put requires Session parameter
    // SKIPPED: view_flag_query_bundle_mock requires Session parameter
    // SKIPPED: view_flag_query_excel_put requires Session parameter
    // SKIPPED: view_flag_query_excel_mock requires Session parameter
    // SKIPPED: view_flag_query_execute_put requires Session parameter
    // SKIPPED: view_flag_query_execute_mock requires Session parameter
    // SKIPPED: view_execute_v2_flag_query not accessible
    // SKIPPED: view_execute_v2_id not accessible
    // SKIPPED: view_id_bundle_put requires Session parameter
    // SKIPPED: view_id_bundle_mock requires Session parameter
    // SKIPPED: view_id_excel_put requires Session parameter
    // SKIPPED: view_id_excel_mock requires Session parameter
    // SKIPPED: view_id_execute_put requires Session parameter
    // SKIPPED: view_id_execute_mock requires Session parameter
    // SKIPPED: table_row_select_post not accessible
}
