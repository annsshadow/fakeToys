// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/get/test-id")
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
            assert!(!bytes.is_empty(), "get_process route should be registered");
        }
    }

    // SKIPPED: create_process requires Session parameter
    #[tokio::test]
    async fn test_list_processes() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/list/test-id")
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
                "list_processes route should be registered"
            );
        }
    }

    // SKIPPED: execute_process requires Session parameter
    #[tokio::test]
    async fn test_get_process_instance() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/instance/test-id")
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
                "get_process_instance route should be registered"
            );
        }
    }

    // SKIPPED: cancel_process_instance requires Session parameter
    // SKIPPED: work_id_processing requires Session parameter
    // SKIPPED: work_v2_id_terminate requires Session parameter
    // SKIPPED: work_v2_id_retract requires Session parameter
    #[tokio::test]
    async fn test_work_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/work/list")
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
            assert!(!bytes.is_empty(), "work_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_process_id_complex() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/process/test-id/complex")
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
                "process_id_complex route should be registered"
            );
        }
    }

    // SKIPPED: work_start requires Session parameter
    // SKIPPED: work_complete requires Session parameter
    // SKIPPED: task_claim requires Session parameter
    // SKIPPED: task_complete requires Session parameter
    // SKIPPED: task_reject requires Session parameter
    // SKIPPED: task_transfer requires Session parameter
    // SKIPPED: gateway_join requires Session parameter
    // SKIPPED: gateway_fork requires Session parameter
    // SKIPPED: start_timer requires Session parameter
    // SKIPPED: cancel_timer requires Session parameter
    // SKIPPED: work_v2_id_goback requires Session parameter
    // SKIPPED: work_v2_id_goback_p2 requires Session parameter
    // SKIPPED: work_v2_id_rollback requires Session parameter
    // SKIPPED: work_v2_id_rollback_p2 requires Session parameter
    // SKIPPED: work_v2_id_add_split requires Session parameter
    // SKIPPED: work_v2_id_add_split_p2 requires Session parameter
    // SKIPPED: work_v2_id_reroute requires Session parameter
    // SKIPPED: work_v2_id_reroute_p2 requires Session parameter
    // SKIPPED: work_id_draft not accessible
    // SKIPPED: work_id_manual_append_identity not accessible
    // SKIPPED: work_id_manual_append_identity_p2 not accessible
    // SKIPPED: work_id_projection not accessible
    // SKIPPED: work_id_projection_p2 not accessible
    // SKIPPED: work_id_series_series_activitytoken_activityToken_processing_signal not accessible
    // SKIPPED: work_process_processId not accessible
    // SKIPPED: work_process_processId_name_name_serial not accessible
    // SKIPPED: work_manual_after_processing requires Session parameter
    // SKIPPED: task_list not accessible
    // SKIPPED: task_id_processing requires Session parameter
    // SKIPPED: task_id_urge requires Session parameter
    // SKIPPED: task_id_replace requires Session parameter
    // SKIPPED: task_id_press requires Session parameter
    // SKIPPED: task_id_expire requires Session parameter
    // SKIPPED: task_id_pass_expired requires Session parameter
    // SKIPPED: task_id_will not accessible
    // SKIPPED: task_v2_id not accessible
    // SKIPPED: task_v2_id_pause requires Session parameter
    // SKIPPED: task_v2_id_reset requires Session parameter
    // SKIPPED: task_v2_id_resume requires Session parameter
    // SKIPPED: task_v3_id_add requires Session parameter
    // SKIPPED: task_id not accessible
    // SKIPPED: taskcompleted_next_task_identity not accessible
    // SKIPPED: taskcompleted_id not accessible
    // SKIPPED: taskcompleted_id_press_work_work requires Session parameter
    // SKIPPED: snap_upload requires Session parameter
    // SKIPPED: snap_work_workId_type_abandoned requires Session parameter
    // SKIPPED: snap_work_workId_type_snap not accessible
    // SKIPPED: snap_work_workId_type_suspend requires Session parameter
    // SKIPPED: snap_workcompleted_workCompletedId_type_abandonedworkcompleted requires Session parameter
    // SKIPPED: snap_workcompleted_workCompletedId_type_snapworkcompleted not accessible
    // SKIPPED: snap_id not accessible
    // SKIPPED: snap_id_restore requires Session parameter
    // SKIPPED: touch_cleanevent not accessible
    // SKIPPED: touch_deletedraft not accessible
    // SKIPPED: touch_handoverjob not accessible
    // SKIPPED: touch_loglongdetained not accessible
    // SKIPPED: touch_merge not accessible
    // SKIPPED: touch_mergeitem not accessible
    // SKIPPED: touch_touchdelay not accessible
    // SKIPPED: touch_urge not accessible
    // SKIPPED: review_create_work not accessible
    // SKIPPED: review_create_workcompleted not accessible
    // SKIPPED: review_init_review requires Session parameter
    // SKIPPED: review_id not accessible
    // SKIPPED: data_job_job not accessible
    // SKIPPED: data_job_job_path not accessible
    // SKIPPED: data_work_id not accessible
    // SKIPPED: data_work_id_delete requires Session parameter
    // SKIPPED: data_work_id_path not accessible
    // SKIPPED: data_work_id_path_delete requires Session parameter
    // SKIPPED: data_workcompleted_id not accessible
    // SKIPPED: data_workcompleted_id_path not accessible
    // SKIPPED: documentversion_work_work not accessible
    // SKIPPED: event_add_update_table requires Session parameter
    // SKIPPED: form_suitable_activity_activityId not accessible
    // SKIPPED: job_v2_job_person_person_view not accessible
    // SKIPPED: job_v2_job_projection not accessible
    // SKIPPED: job_v2_job_projection_p2 not accessible
    // SKIPPED: job_job not accessible
    // SKIPPED: readcompleted_id not accessible
    // SKIPPED: record_job_job not accessible
    // SKIPPED: record_task_processing not accessible
    // SKIPPED: record_work_processing not accessible
    // SKIPPED: record_work_terminate not accessible
    // SKIPPED: record_id not accessible
    // SKIPPED: service_work_id_touch requires Session parameter
    // SKIPPED: service_work_id_touch_p2 requires Session parameter
    // SKIPPED: attachment_copy_work_workId requires Session parameter
    // SKIPPED: attachment_copy_workcompleted_workCompletedId requires Session parameter
    // SKIPPED: attachment_edit_id_text not accessible
    // SKIPPED: attachment_id not accessible
    // SKIPPED: attachment_id_work_workId not accessible
    // SKIPPED: attachment_id_workcompleted_workCompletedId not accessible
    // SKIPPED: applicationdict_id not accessible
    // SKIPPED: applicationdict_id_path0_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_path2_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_path2_path3_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_path2_path3_path4_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_path2_path3_path4_path5_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_path2_path3_path4_path5_path6_data not accessible
    // SKIPPED: applicationdict_id_path0_path1_path2_path3_path4_path5_path6_path7_data not accessible
    // SKIPPED: workcompleted_process_processFlag not accessible
    // SKIPPED: workcompleted_shift_time not accessible
    // SKIPPED: workcompleted_flag_merge requires Session parameter
    // SKIPPED: workcompleted_flag_rollback requires Session parameter
    // SKIPPED: work_v3_retract requires Session parameter
    // SKIPPED: start not accessible
    // SKIPPED: cancel not accessible
    // SKIPPED: restore not accessible
    // SKIPPED: register not accessible
    // SKIPPED: dict_edit requires Session parameter
    // SKIPPED: data_job_put requires Session parameter
    // SKIPPED: data_job_put_path requires Session parameter
    // SKIPPED: data_work_create requires Session parameter
    // SKIPPED: data_work_update requires Session parameter
    // SKIPPED: data_work_create_path requires Session parameter
    // SKIPPED: data_work_update_path requires Session parameter
    // SKIPPED: data_work_delete requires Session parameter
    // SKIPPED: data_work_delete_path requires Session parameter
    // SKIPPED: data_wc_update requires Session parameter
    // SKIPPED: data_wc_update_path requires Session parameter
    // SKIPPED: att_delete_id requires Session parameter
    // SKIPPED: att_delete_with_work requires Session parameter
    // SKIPPED: att_delete_with_workcompleted requires Session parameter
    // SKIPPED: att_edit_id requires Session parameter
    // SKIPPED: att_edit_text requires Session parameter
    // SKIPPED: att_copy_to_work requires Session parameter
    // SKIPPED: job_delete requires Session parameter
    // SKIPPED: job_v2_view not accessible
    // SKIPPED: read_create_with_work requires Session parameter
    // SKIPPED: read_create_with_workcompleted requires Session parameter
    // SKIPPED: read_delete requires Session parameter
    // SKIPPED: read_processing requires Session parameter
    // SKIPPED: read_replace requires Session parameter
    // SKIPPED: read_reset requires Session parameter
    // SKIPPED: readcompleted_delete requires Session parameter
    // SKIPPED: record_create_with_job requires Session parameter
    // SKIPPED: record_edit requires Session parameter
    // SKIPPED: record_delete requires Session parameter
    // SKIPPED: review_init_for_view requires Session parameter
    // SKIPPED: review_delete requires Session parameter
    // SKIPPED: snap_delete requires Session parameter
    // SKIPPED: snap_type_abandoned requires Session parameter
    // SKIPPED: snap_type_snap requires Session parameter
    // SKIPPED: snap_type_suspend requires Session parameter
    // SKIPPED: snap_wc_type_abandoned requires Session parameter
    // SKIPPED: snap_wc_type_snap requires Session parameter
    // SKIPPED: task_delete requires Session parameter
    // SKIPPED: taskcompleted_delete requires Session parameter
    // SKIPPED: taskcompleted_press requires Session parameter
    // SKIPPED: taskcompleted_update_next_identity requires Session parameter
    // SKIPPED: touch_clean_event requires Session parameter
    // SKIPPED: touch_delete_draft requires Session parameter
    // SKIPPED: touch_handover_job requires Session parameter
    // SKIPPED: touch_log_long_detained requires Session parameter
    // SKIPPED: touch_delay requires Session parameter
    // SKIPPED: touch_merge_item requires Session parameter
    // SKIPPED: documentversion_create requires Session parameter
    // SKIPPED: work_create requires Session parameter
    // SKIPPED: work_create_for_process requires Session parameter
    // SKIPPED: work_create_with_serial requires Session parameter
    // SKIPPED: work_get not accessible
    // SKIPPED: work_edit requires Session parameter
    // SKIPPED: work_delete requires Session parameter
    // SKIPPED: work_draft_delete requires Session parameter
    // SKIPPED: work_manual_after requires Session parameter
    // SKIPPED: work_v3_retract_body requires Session parameter
    // SKIPPED: work_processing_signal requires Session parameter
    // SKIPPED: work_add_identity_matrix requires Session parameter
    // SKIPPED: wc_merge_flag requires Session parameter
    // SKIPPED: wc_process_flag requires Session parameter
    // SKIPPED: wc_shift_time requires Session parameter
    // SKIPPED: wc_rollback_flag requires Session parameter
}
