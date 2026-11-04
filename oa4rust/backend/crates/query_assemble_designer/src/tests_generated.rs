// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    // SKIPPED: create_table_definition requires Session parameter
    // SKIPPED: update_table_definition requires Session parameter
    // SKIPPED: execute_table_definition requires Session parameter
    #[tokio::test]
    async fn test_get_designer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/get/test-id")
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
            assert!(!bytes.is_empty(), "get_designer route should be registered");
        }
    }

    // SKIPPED: create_designer requires Session parameter
    #[tokio::test]
    async fn test_list_designers() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/list/test-id")
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
                "list_designers route should be registered"
            );
        }
    }

    // SKIPPED: save_designer requires Session parameter
    // SKIPPED: delete_designer requires Session parameter
    #[tokio::test]
    async fn test_list_designers_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/list")
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
                "list_designers_all route should be registered"
            );
        }
    }

    // SKIPPED: save_designer_bare requires Session parameter
    // SKIPPED: delete_designer_bare requires Session parameter
    #[tokio::test]
    async fn test_designer_search() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/search")
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
                "designer_search route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_input_compare() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/input/compare")
                    .method("PUT")
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
                "input_compare route should be registered"
            );
        }
    }

    // SKIPPED: input_cover requires Session parameter
    // SKIPPED: input_create requires Session parameter
    #[tokio::test]
    async fn test_input_prepare_cover() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/input/prepare/cover")
                    .method("PUT")
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
                "input_prepare_cover route should be registered"
            );
        }
    }

    // SKIPPED: input_prepare_create requires Session parameter
    #[tokio::test]
    async fn test_neural_list_model() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/neural/list/model")
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
                "neural_list_model route should be registered"
            );
        }
    }

    // SKIPPED: neural_model requires Session parameter
    #[tokio::test]
    async fn test_output_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/output/list")
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
            assert!(!bytes.is_empty(), "output_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_query_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/list/all")
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
                "query_list_all route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_query_list_summary() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/list/summary")
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
                "query_list_summary route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_query_querycategory_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/querycategory/list")
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
                "query_querycategory_list route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_query_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/query/test-id")
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
            assert!(!bytes.is_empty(), "query_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_stat_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/stat/list/test-id/prev/test-id")
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
                "stat_list_id_prev_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_table_list_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/table/list/manage")
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
                "table_list_manage route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_table_list_tableFlag_row_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/table/list/row/test-id/test-id/prev/test-id")
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
                "table_list_tableFlag_row_id_prev_count route should be registered"
            );
        }
    }

    // SKIPPED: table_query_query_build requires Session parameter
    // SKIPPED: table_reload_dynamic requires Session parameter
    // SKIPPED: table_tableFlag_row_delete_all requires Session parameter
    // SKIPPED: table_tableFlag_row_save requires Session parameter
    #[tokio::test]
    async fn test_view_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/view/list/test-id/prev/test-id")
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
                "view_list_id_prev_count route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_view_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/view/test-id")
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

    #[tokio::test]
    async fn test_view_id_permission() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/view/permission/test-id")
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
                "view_id_permission route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_importer_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/importer/list")
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
                "importer_list route should be registered"
            );
        }
    }

    // SKIPPED: importer_create requires Session parameter
    // SKIPPED: importer_save requires Session parameter
    // SKIPPED: importer_delete requires Session parameter
    #[tokio::test]
    async fn test_stat_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/assemble/designer/stat/list")
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
            assert!(!bytes.is_empty(), "stat_list route should be registered");
        }
    }

    // SKIPPED: stat_create requires Session parameter
    // SKIPPED: stat_save requires Session parameter
    // SKIPPED: stat_delete requires Session parameter
    // SKIPPED: require_row_owner requires Session parameter
    // SKIPPED: id_count not accessible
    // SKIPPED: importmodel_list_query_flag not accessible
    // SKIPPED: importmodel_list_query_flag_p2 not accessible
    // SKIPPED: importmodel_id not accessible
    // SKIPPED: importmodel_id_permission not accessible
    // SKIPPED: neural_generate_model_modelFlag requires Session parameter
    // SKIPPED: neural_learn_model_modelFlag requires Session parameter
    // SKIPPED: neural_model_modelFlag not accessible
    // SKIPPED: neural_model_modelFlag_reset_status requires Session parameter
    // SKIPPED: neural_model_modelFlag_reset_status_p2 requires Session parameter
    // SKIPPED: neural_stop_generating_model_modelFlag requires Session parameter
    // SKIPPED: neural_stop_learn_model_modelFlag requires Session parameter
    // SKIPPED: output_flag_select_file not accessible
    // SKIPPED: output_queryFlag_select not accessible
    // SKIPPED: query_entity_entity_category_entityCategory_properties not accessible
    // SKIPPED: query_entity_entity_category_entityCategory_properties_p3 not accessible
    // SKIPPED: query_list_querycategory_queryCategory not accessible
    // SKIPPED: query_list_querycategory_queryCategory_p2 not accessible
    // SKIPPED: query_list_summary_querycategory_queryCategory not accessible
    // SKIPPED: query_list_summary_querycategory_queryCategory_p2 not accessible
    // SKIPPED: query_flag_icon not accessible
    // SKIPPED: query_id_permission not accessible
    // SKIPPED: stat_list_query_flag not accessible
    // SKIPPED: stat_list_query_flag_p2 not accessible
    // SKIPPED: stat_list_id_next_count not accessible
    // SKIPPED: stat_list_id_next_count_p3 not accessible
    // SKIPPED: stat_id not accessible
    // SKIPPED: stat_id_permission not accessible
    // SKIPPED: stat_id_simulate not accessible
    // SKIPPED: table_export_tableFlag_count_count not accessible
    // SKIPPED: table_export_tableFlag_count_count_p3 not accessible
    // SKIPPED: table_list_query_flag not accessible
    // SKIPPED: table_list_query_flag_p2 not accessible
    // SKIPPED: table_list_tableFlag_row_select_where_where not accessible
    // SKIPPED: table_list_tableFlag_row_id_next_count not accessible
    // SKIPPED: table_list_tableFlag_row_id_next_count_p4 not accessible
    // SKIPPED: table_flag not accessible
    // SKIPPED: table_flag_execute not accessible
    // SKIPPED: table_flag_status_build requires Session parameter
    // SKIPPED: table_flag_status_build_p2 requires Session parameter
    // SKIPPED: table_flag_status_draft requires Session parameter
    // SKIPPED: table_flag_status_draft_p2 requires Session parameter
    // SKIPPED: table_id_permission not accessible
    // SKIPPED: table_query_build_dispatch requires Session parameter
    // SKIPPED: table_tableFlag_row not accessible
    // SKIPPED: table_tableFlag_row_count_where_where not accessible
    // SKIPPED: table_tableFlag_row_id not accessible
    // SKIPPED: view_list_query_flag not accessible
    // SKIPPED: view_list_query_flag_p3 not accessible
    // SKIPPED: view_list_id_next_count not accessible
    // SKIPPED: view_list_id_next_count_p4 not accessible
    // SKIPPED: view_id_bundle not accessible
    // SKIPPED: view_id_simulate not accessible
    // SKIPPED: statement_create requires Session parameter
    // SKIPPED: statement_edit requires Session parameter
    // SKIPPED: statement_delete requires Session parameter
    // SKIPPED: statement_get_flag not accessible
    // SKIPPED: statement_manage_list not accessible
    // SKIPPED: statement_list_with_query not accessible
    // SKIPPED: statement_permission requires Session parameter
    // SKIPPED: statement_execute_v2 not accessible
    // SKIPPED: statement_execute_mode_v2 not accessible
    // SKIPPED: importmodel_create requires Session parameter
    // SKIPPED: importmodel_edit requires Session parameter
    // SKIPPED: importmodel_delete requires Session parameter
    // SKIPPED: neural_delete_model_modelFlag requires Session parameter
    // SKIPPED: neural_update_model_modelFlag requires Session parameter
    // SKIPPED: stat_edit requires Session parameter
    // SKIPPED: table_create not accessible
    // SKIPPED: table_edit not accessible
    // SKIPPED: table_delete requires Session parameter
    // SKIPPED: table_tableFlag_row_insert requires Session parameter
    // SKIPPED: table_tableFlag_row_update requires Session parameter
    // SKIPPED: table_tableFlag_row_delete requires Session parameter
    // SKIPPED: view_create requires Session parameter
    // SKIPPED: view_edit requires Session parameter
    // SKIPPED: view_delete requires Session parameter
    // SKIPPED: query_set_icon requires Session parameter
    // SKIPPED: designer_search_v2 not accessible
    // SKIPPED: id_generate not accessible
    // SKIPPED: importmodel_get_flag not accessible
    // SKIPPED: importmodel_edit_flag requires Session parameter
    // SKIPPED: importmodel_delete_flag requires Session parameter
    // SKIPPED: importmodel_permission_set requires Session parameter
    // SKIPPED: output_select_put requires Session parameter
    // SKIPPED: query_create_v2 requires Session parameter
    // SKIPPED: query_delete_flag requires Session parameter
    // SKIPPED: query_edit_flag requires Session parameter
    // SKIPPED: query_icon_set requires Session parameter
    // SKIPPED: query_permission_set requires Session parameter
    // SKIPPED: stat_permission_set requires Session parameter
    // SKIPPED: table_permission_set requires Session parameter
    // SKIPPED: view_permission_set requires Session parameter
    // SKIPPED: table_build_dispatch_flag requires Session parameter
    // SKIPPED: stat_simulate_put requires Session parameter
    // SKIPPED: view_bundle_put requires Session parameter
    // SKIPPED: view_simulate_put requires Session parameter
    // SKIPPED: designer_execute not accessible
    // SKIPPED: stat_do not accessible
}
