// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    // SKIPPED: create_flow requires Session parameter
    #[tokio::test]
    async fn test_get_flow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/designer/get/test-id")
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
            assert!(!bytes.is_empty(), "get_flow route should be registered");
        }
    }

    #[tokio::test]
    async fn test_list_flows() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/designer/list/test-id")
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
            assert!(!bytes.is_empty(), "list_flows route should be registered");
        }
    }

    #[tokio::test]
    async fn test_designer_bare_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/designer")
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
                "designer_bare_list route should be registered"
            );
        }
    }

    // SKIPPED: save_flow requires Session parameter
    // SKIPPED: delete_flow requires Session parameter
    #[tokio::test]
    async fn test_preview_flow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/assemble/designer/preview/test-id")
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
            assert!(!bytes.is_empty(), "preview_flow route should be registered");
        }
    }

    // SKIPPED: application_list not accessible
    // SKIPPED: application_list_applicationcategory_applicationCategory not accessible
    // SKIPPED: application_list_summary not accessible
    // SKIPPED: application_list_summary_applicationcategory_applicationCategory not accessible
    // SKIPPED: application_id not accessible
    // SKIPPED: application_id_icon not accessible
    // SKIPPED: application_id_permission not accessible
    // SKIPPED: application_id_onlyRemoveNotCompleted not accessible
    // SKIPPED: applicationcategory_list not accessible
    // SKIPPED: applicationdict_list_application_applicationId not accessible
    // SKIPPED: applicationdict_list_paging_page_size_size not accessible
    // SKIPPED: applicationdict_id not accessible
    // SKIPPED: elementtool_applicationdict_orphan not accessible
    // SKIPPED: elementtool_form_orphan not accessible
    // SKIPPED: elementtool_process_orphan not accessible
    // SKIPPED: elementtool_script_orphan not accessible
    // SKIPPED: file_list_application_applicationFlag not accessible
    // SKIPPED: file_list_id_next_count not accessible
    // SKIPPED: file_list_id_prev_count not accessible
    // SKIPPED: file_flag not accessible
    // SKIPPED: file_flag_application_applicationFlag not accessible
    // SKIPPED: file_id not accessible
    // SKIPPED: file_id_content not accessible
    // SKIPPED: file_id_download not accessible
    // SKIPPED: file_id_upload requires Session parameter
    // SKIPPED: form_list_application_applicationId not accessible
    // SKIPPED: form_list_formfield_application_applicationId not accessible
    // SKIPPED: form_list_id_formfield not accessible
    // SKIPPED: form_list_id_next_count not accessible
    // SKIPPED: form_list_id_prev_count not accessible
    // SKIPPED: form_id not accessible
    // SKIPPED: formversion_list_form_formId not accessible
    // SKIPPED: formversion_id not accessible
    // SKIPPED: id_count not accessible
    // SKIPPED: input_compare not accessible
    // SKIPPED: input_cover not accessible
    // SKIPPED: input_create not accessible
    // SKIPPED: input_prepare_cover not accessible
    // SKIPPED: input_prepare_create not accessible
    // SKIPPED: item_access_delete_process_processId_path_path requires Session parameter
    // SKIPPED: item_access_path_path not accessible
    // SKIPPED: item_access_process_processId not accessible
    // SKIPPED: item_access_process_processId_path_path not accessible
    // SKIPPED: item_access_id not accessible
    // SKIPPED: mapping_list_application_applicationFlag not accessible
    // SKIPPED: mapping_list_id_next_count not accessible
    // SKIPPED: mapping_list_id_prev_count not accessible
    // SKIPPED: mapping_flag not accessible
    // SKIPPED: mapping_flag_execute not accessible
    // SKIPPED: mergeitemplan_estimate not accessible
    // SKIPPED: mergeitemplan_list_application_applicationId_paging_page_size_size not accessible
    // SKIPPED: mergeitemplan_list_paging_page_size_size not accessible
    // SKIPPED: mergeitemplan_id not accessible
    // SKIPPED: output_list not accessible
    // SKIPPED: output_applicationFlag_select not accessible
    // SKIPPED: process_activity_flag_activityType_activityType not accessible
    // SKIPPED: process_application_applicationId not accessible
    // SKIPPED: process_application_applicationId_disable_edition not accessible
    // SKIPPED: process_application_applicationId_edition_edition not accessible
    // SKIPPED: process_form_formId not accessible
    // SKIPPED: process_upgrade_all requires Session parameter
    // SKIPPED: process_id not accessible
    // SKIPPED: process_id_disable requires Session parameter
    // SKIPPED: process_id_enable requires Session parameter
    // SKIPPED: process_id_enabled not accessible
    // SKIPPED: process_id_execute_projection not accessible
    // SKIPPED: process_id_lead_out not accessible
    // SKIPPED: process_id_list_element not accessible
    // SKIPPED: process_id_permission not accessible
    // SKIPPED: process_id_process not accessible
    // SKIPPED: process_id_upgrade requires Session parameter
    // SKIPPED: process_id_onlyRemoveNotCompleted not accessible
    // SKIPPED: process_id_onlyRemoveNotCompleted_edition not accessible
    // SKIPPED: processversion_list_process_processId not accessible
    // SKIPPED: processversion_id not accessible
    // SKIPPED: script_application_applicationId not accessible
    // SKIPPED: script_application_applicationId_name_name not accessible
    // SKIPPED: script_list_manager not accessible
    // SKIPPED: script_list_paging_page_size_size not accessible
    // SKIPPED: script_list_id_next_count not accessible
    // SKIPPED: script_list_id_prev_count not accessible
    // SKIPPED: script_id not accessible
    // SKIPPED: scriptversion_list_script_scriptId not accessible
    // SKIPPED: scriptversion_id not accessible
    // SKIPPED: templateform_list not accessible
    // SKIPPED: templateform_list_category not accessible
    // SKIPPED: templateform_id not accessible
    // SKIPPED: workcompleted_application_applicationFlag_merge_data not accessible
    // SKIPPED: workcompleted_process_processFlag_merge_data not accessible
    // SKIPPED: application_id_update requires Session parameter
    // SKIPPED: application_id_icon_update requires Session parameter
    // SKIPPED: application_id_permission_save requires Session parameter
    // SKIPPED: applicationdict_create requires Session parameter
    // SKIPPED: applicationdict_paging_post not accessible
    // SKIPPED: applicationdict_id_delete requires Session parameter
    // SKIPPED: file_flag_in_application not accessible
    // SKIPPED: form_list_next_exact not accessible
    // SKIPPED: form_list_prev_exact not accessible
    // SKIPPED: item_access_create requires Session parameter
    // SKIPPED: item_access_bach_save requires Session parameter
    // SKIPPED: item_access_delete_exact requires Session parameter
    // SKIPPED: item_access_path_list not accessible
    // SKIPPED: item_access_process_path_list not accessible
    // SKIPPED: mapping_create requires Session parameter
    // SKIPPED: mapping_list_next_exact not accessible
    // SKIPPED: mapping_list_prev_exact not accessible
    // SKIPPED: mapping_id_update requires Session parameter
    // SKIPPED: mapping_id_delete requires Session parameter
    // SKIPPED: mergeitemplan_create requires Session parameter
    // SKIPPED: mergeitemplan_paging_by_application not accessible
    // SKIPPED: mergeitemplan_paging_exact not accessible
    // SKIPPED: mergeitemplan_id_update requires Session parameter
    // SKIPPED: mergeitemplan_id_delete requires Session parameter
    // SKIPPED: process_disable_edition_list not accessible
    // SKIPPED: process_edition_list not accessible
    // SKIPPED: process_id_update requires Session parameter
    // SKIPPED: process_id_permission_save requires Session parameter
    // SKIPPED: process_edition_delete requires Session parameter
    // SKIPPED: script_by_name_exact not accessible
    // SKIPPED: dict_list not accessible
    // SKIPPED: dict_create requires Session parameter
    // SKIPPED: dict_save requires Session parameter
    // SKIPPED: dict_delete requires Session parameter
    // SKIPPED: form_list not accessible
    // SKIPPED: form_create requires Session parameter
    // SKIPPED: form_save requires Session parameter
    // SKIPPED: form_delete requires Session parameter
    // SKIPPED: xform_list not accessible
    // SKIPPED: xform_create requires Session parameter
    // SKIPPED: xform_save requires Session parameter
    // SKIPPED: xform_delete requires Session parameter
    // SKIPPED: create requires Session parameter
    // SKIPPED: update requires Session parameter
    // SKIPPED: delete requires Session parameter
}
