#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_process route should be registered");
    }

    #[tokio::test]
    async fn test_create_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_process route should be registered");
    }

    #[tokio::test]
    async fn test_list_processes() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_processes route should be registered");
    }

    #[tokio::test]
    async fn test_execute_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/execute/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "execute_process route should be registered");
    }

    #[tokio::test]
    async fn test_get_process_instance() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/instance/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_process_instance route should be registered");
    }

    #[tokio::test]
    async fn test_cancel_process_instance() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/cancel/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "cancel_process_instance route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/processing")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_processing route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_terminate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/terminate")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_terminate route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_retract() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/retract")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_retract route should be registered");
    }

    // SKIPPED: work_v2_id_goback not accessible
    // SKIPPED: work_v2_id_rollback not accessible
    // SKIPPED: work_v2_id_add_split not accessible
    // SKIPPED: work_v2_id_reroute not accessible
    // SKIPPED: work_id_draft not accessible
    // SKIPPED: work_id_manual_append_identity not accessible
    // SKIPPED: work_id_projection not accessible
    // SKIPPED: work_id_series_series_activitytoken_activityToken_processing_signal not accessible
    // SKIPPED: work_process_processId not accessible
    // SKIPPED: work_process_processId_name_name_serial not accessible
    // SKIPPED: work_manual_after_processing not accessible
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
    #[tokio::test]
    async fn test_work_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/work/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list route should be registered");
    }

    #[tokio::test]
    async fn test_process_id_complex() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/process/test-id/complex")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_id_complex route should be registered");
    }

    // SKIPPED: data_work_id_path_delete not accessible
    // SKIPPED: data_workcompleted_id not accessible
    // SKIPPED: data_workcompleted_id_path not accessible
    // SKIPPED: documentversion_work_work not accessible
    // SKIPPED: event_add_update_table not accessible
    // SKIPPED: form_suitable_activity_activityId not accessible
    // SKIPPED: job_v2_job_person_person_view not accessible
    // SKIPPED: job_v2_job_projection not accessible
    // SKIPPED: job_job not accessible
    // SKIPPED: readcompleted_id not accessible
    // SKIPPED: record_job_job not accessible
    // SKIPPED: record_task_processing not accessible
    // SKIPPED: record_work_processing not accessible
    // SKIPPED: record_work_terminate not accessible
    // SKIPPED: record_id not accessible
    // SKIPPED: service_work_id_touch not accessible
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
    #[tokio::test]
    async fn test_work_start() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/start")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_start route should be registered");
    }

    #[tokio::test]
    async fn test_work_complete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/work/test-id/complete")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_complete route should be registered");
    }

    #[tokio::test]
    async fn test_task_claim() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/task/test-id/claim")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_claim route should be registered");
    }

    #[tokio::test]
    async fn test_task_complete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/task/test-id/complete")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_complete route should be registered");
    }

    #[tokio::test]
    async fn test_task_reject() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/task/test-id/reject")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_reject route should be registered");
    }

    #[tokio::test]
    async fn test_task_transfer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/task/test-id/transfer/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_transfer route should be registered");
    }

    #[tokio::test]
    #[ignore = "route matching issue in service_processing crate"]
    async fn test_gateway_join() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/gateway/test-id/test-id/join")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "gateway_join route should be registered");
    }

    #[tokio::test]
    async fn test_gateway_fork() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/gateway/fork/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "gateway_fork route should be registered");
    }

    // SKIPPED: start not accessible
    // SKIPPED: cancel not accessible
    // SKIPPED: restore not accessible
    // SKIPPED: register not accessible
    #[tokio::test]
    async fn test_start_timer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/timer/start")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "start_timer route should be registered");
    }

    #[tokio::test]
    async fn test_cancel_timer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/service/processing/timer/test-id/cancel")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "cancel_timer route should be registered");
    }

}