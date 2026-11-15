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

    #[tokio::test]
    async fn test_create_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/create")
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
                "create_process route should be registered"
            );
        }
    }

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

    #[tokio::test]
    async fn test_execute_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/execute/test-id")
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
                "execute_process route should be registered"
            );
        }
    }

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

    #[tokio::test]
    async fn test_cancel_process_instance() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/cancel/test-id")
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
                "cancel_process_instance route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_work_id_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/processing")
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
                "work_id_processing route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_work_v2_id_terminate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/terminate")
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
                "work_v2_id_terminate route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_work_v2_id_retract() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/retract")
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
                "work_v2_id_retract route should be registered"
            );
        }
    }

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

    #[tokio::test]
    async fn test_work_start() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/start")
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
            assert!(!bytes.is_empty(), "work_start route should be registered");
        }
    }

    #[tokio::test]
    async fn test_work_complete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/complete")
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
                "work_complete route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_task_claim() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/claim")
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
            assert!(!bytes.is_empty(), "task_claim route should be registered");
        }
    }

    #[tokio::test]
    async fn test_task_complete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/complete")
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
                "task_complete route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_task_reject() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/reject")
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
            assert!(!bytes.is_empty(), "task_reject route should be registered");
        }
    }

    #[tokio::test]
    async fn test_task_transfer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/transfer/test-id")
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
                "task_transfer route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_gateway_join() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/gateway/test-id/test-id/join")
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
            assert!(!bytes.is_empty(), "gateway_join route should be registered");
        }
    }

    #[tokio::test]
    async fn test_gateway_fork() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/gateway/fork/test-id")
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
            assert!(!bytes.is_empty(), "gateway_fork route should be registered");
        }
    }

    #[tokio::test]
    async fn test_start_timer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/timer/start")
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
            assert!(!bytes.is_empty(), "start_timer route should be registered");
        }
    }

    #[tokio::test]
    async fn test_cancel_timer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/timer/test-id/cancel")
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
            assert!(!bytes.is_empty(), "cancel_timer route should be registered");
        }
    }

    // SKIPPED: work_v2_id_goback not accessible
    // SKIPPED: work_v2_id_goback_p2 not accessible
    // SKIPPED: work_v2_id_rollback not accessible
    // SKIPPED: work_v2_id_rollback_p2 not accessible
    // SKIPPED: work_v2_id_add_split not accessible
    // SKIPPED: work_v2_id_add_split_p2 not accessible
    // SKIPPED: work_v2_id_reroute not accessible
    // SKIPPED: work_v2_id_reroute_p2 not accessible
    // SKIPPED: work_id_draft not accessible
    // SKIPPED: work_id_manual_append_identity not accessible
    // SKIPPED: work_id_manual_append_identity_p2 not accessible
    // SKIPPED: work_id_projection not accessible
    // SKIPPED: work_id_projection_p2 not accessible
    // SKIPPED: work_id_series_series_activitytoken_activityToken_processing_signal not accessible
    // SKIPPED: work_process_processId not accessible
    // SKIPPED: work_process_processId_name_name_serial not accessible
    // SKIPPED: work_manual_after_processing not accessible
    // SKIPPED: task_list not accessible
    // SKIPPED: task_id_processing not accessible
    // SKIPPED: task_id_urge not accessible
    // SKIPPED: task_id_replace not accessible
    // SKIPPED: task_id_press not accessible
    // SKIPPED: task_id_expire not accessible
    // SKIPPED: task_id_pass_expired not accessible
    // SKIPPED: task_id_will not accessible
    // SKIPPED: task_v2_id not accessible
    // SKIPPED: task_v2_id_pause not accessible
    // SKIPPED: task_v2_id_reset not accessible
    // SKIPPED: task_v2_id_resume not accessible
    // SKIPPED: task_v3_id_add not accessible
    // SKIPPED: task_id not accessible
    // SKIPPED: taskcompleted_next_task_identity not accessible
    // SKIPPED: taskcompleted_id not accessible
    // SKIPPED: taskcompleted_id_press_work_work not accessible
    // SKIPPED: snap_upload not accessible
    // SKIPPED: snap_work_workId_type_abandoned not accessible
    // SKIPPED: snap_work_workId_type_snap not accessible
    // SKIPPED: snap_work_workId_type_suspend not accessible
    // SKIPPED: snap_workcompleted_workCompletedId_type_abandonedworkcompleted not accessible
    // SKIPPED: snap_workcompleted_workCompletedId_type_snapworkcompleted not accessible
    // SKIPPED: snap_id not accessible
    // SKIPPED: snap_id_restore not accessible
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
    // SKIPPED: review_init_review not accessible
    // SKIPPED: review_id not accessible
    // SKIPPED: data_job_job not accessible
    // SKIPPED: data_job_job_path not accessible
    // SKIPPED: data_work_id not accessible
    // SKIPPED: data_work_id_delete not accessible
    // SKIPPED: data_work_id_path not accessible
    // SKIPPED: data_work_id_path_delete not accessible
    // SKIPPED: data_workcompleted_id not accessible
    // SKIPPED: data_workcompleted_id_path not accessible
    // SKIPPED: documentversion_work_work not accessible
    // SKIPPED: event_add_update_table not accessible
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
    // SKIPPED: service_work_id_touch not accessible
    // SKIPPED: service_work_id_touch_p2 not accessible
    // SKIPPED: attachment_copy_work_workId not accessible
    // SKIPPED: attachment_copy_workcompleted_workCompletedId not accessible
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
    // SKIPPED: workcompleted_flag_merge not accessible
    // SKIPPED: workcompleted_flag_rollback not accessible
    // SKIPPED: work_v3_retract not accessible
    // SKIPPED: tick not accessible
    // SKIPPED: start not accessible
    // SKIPPED: cancel not accessible
    // SKIPPED: restore not accessible
    // SKIPPED: register not accessible
    // SKIPPED: dict_edit not accessible
    // SKIPPED: data_job_put not accessible
    // SKIPPED: data_job_put_path not accessible
    // SKIPPED: data_work_create not accessible
    // SKIPPED: data_work_update not accessible
    // SKIPPED: data_work_create_path not accessible
    // SKIPPED: data_work_update_path not accessible
    // SKIPPED: data_work_delete not accessible
    // SKIPPED: data_work_delete_path not accessible
    // SKIPPED: data_wc_update not accessible
    // SKIPPED: data_wc_update_path not accessible
    // SKIPPED: att_delete_id not accessible
    // SKIPPED: att_delete_with_work not accessible
    // SKIPPED: att_delete_with_workcompleted not accessible
    // SKIPPED: att_edit_id not accessible
    // SKIPPED: att_edit_text not accessible
    // SKIPPED: att_copy_to_work not accessible
    // SKIPPED: job_delete not accessible
    // SKIPPED: job_v2_view not accessible
    // SKIPPED: read_create_with_work not accessible
    // SKIPPED: read_create_with_workcompleted not accessible
    // SKIPPED: read_delete not accessible
    // SKIPPED: read_processing not accessible
    // SKIPPED: read_replace not accessible
    // SKIPPED: read_reset not accessible
    // SKIPPED: readcompleted_delete not accessible
    // SKIPPED: record_create_with_job not accessible
    // SKIPPED: record_edit not accessible
    // SKIPPED: record_delete not accessible
    // SKIPPED: review_init_for_view not accessible
    // SKIPPED: review_delete not accessible
    // SKIPPED: snap_delete not accessible
    // SKIPPED: snap_type_abandoned not accessible
    // SKIPPED: snap_type_snap not accessible
    // SKIPPED: snap_type_suspend not accessible
    // SKIPPED: snap_wc_type_abandoned not accessible
    // SKIPPED: snap_wc_type_snap not accessible
    // SKIPPED: task_delete not accessible
    // SKIPPED: taskcompleted_delete not accessible
    // SKIPPED: taskcompleted_press not accessible
    // SKIPPED: taskcompleted_update_next_identity not accessible
    // SKIPPED: touch_clean_event not accessible
    // SKIPPED: touch_delete_draft not accessible
    // SKIPPED: touch_handover_job not accessible
    // SKIPPED: run_log_long_detained not accessible
    // SKIPPED: run_touch_delay not accessible
    // SKIPPED: touch_log_long_detained not accessible
    // SKIPPED: touch_delay not accessible
    // SKIPPED: touch_merge_item not accessible
    // SKIPPED: documentversion_create not accessible
    // SKIPPED: work_create not accessible
    // SKIPPED: work_create_for_process not accessible
    // SKIPPED: work_create_with_serial not accessible
    // SKIPPED: work_get not accessible
    // SKIPPED: work_edit not accessible
    // SKIPPED: work_delete not accessible
    // SKIPPED: work_draft_delete not accessible
    // SKIPPED: work_manual_after not accessible
    // SKIPPED: work_v3_retract_body not accessible
    // SKIPPED: work_processing_signal not accessible
    // SKIPPED: work_add_identity_matrix not accessible
    // SKIPPED: wc_merge_flag not accessible
    // SKIPPED: wc_process_flag not accessible
    // SKIPPED: wc_shift_time not accessible
    // SKIPPED: wc_rollback_flag not accessible
}
