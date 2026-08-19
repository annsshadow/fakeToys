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

    // SKIPPED: work_id_processing not accessible
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

    // ──────────────────────────────────────────────────────────────────────────────
    // Integration tests for workflow execution semantics
    // ──────────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_work_start_and_claim() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        let mut client = pool.get().await.unwrap();
        client.execute("DELETE FROM x_task WHERE work = $1", &[&"test-work-start"]).await.unwrap();
        client.execute("DELETE FROM x_work WHERE id = $1", &[&"test-work-start"]).await.unwrap();
        client.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&"test-work-start", &"Test Work", &"default", &"default", &"pending", &"tester"],
        ).await.unwrap();

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            "/jaxrs/work/test-work-start/start",
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "work_start should succeed: {:?}", json);

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            "/jaxrs/task/test-task-id/claim",
            None,
            None,
            None,
        ).await;
        assert_ne!(status, StatusCode::NOT_FOUND, "task_claim route should be registered");
    }

    #[tokio::test]
    async fn test_task_complete_flow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        let mut client = pool.get().await.unwrap();
        client.execute("DELETE FROM x_task WHERE id = $1", &[&"test-task-complete"]).await.unwrap();
        client.execute("DELETE FROM x_work WHERE id = $1", &[&"test-work-complete"]).await.unwrap();
        client.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&"test-work-complete", &"Test Work", &"default", &"default", &"processing", &"tester"],
        ).await.unwrap();
        client.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&"test-task-complete", &"Test Task", &"test-work-complete", &"approve", &"token-1", &"user1", &"active"],
        ).await.unwrap();

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            "/jaxrs/task/test-task-complete/complete",
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "task_complete should succeed: {:?}", json);
    }

    #[tokio::test]
    async fn test_task_reject_flow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        let mut client = pool.get().await.unwrap();
        client.execute("DELETE FROM x_task WHERE id = $1", &[&"test-task-reject"]).await.unwrap();
        client.execute("DELETE FROM x_work WHERE id = $1", &[&"test-work-reject"]).await.unwrap();
        client.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&"test-work-reject", &"Test Work", &"default", &"default", &"processing", &"tester"],
        ).await.unwrap();
        client.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&"test-task-reject", &"Test Task", &"test-work-reject", &"approve", &"token-1", &"user1", &"active"],
        ).await.unwrap();

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            "/jaxrs/task/test-task-reject/reject",
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "task_reject should succeed: {:?}", json);
    }

    #[tokio::test]
    async fn test_gateway_join_flow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        let mut client = pool.get().await.unwrap();
        client.execute("DELETE FROM x_task WHERE work = $1", &[&"test-work-gateway"]).await.unwrap();
        client.execute("DELETE FROM x_work WHERE id = $1", &[&"test-work-gateway"]).await.unwrap();
        client.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&"test-work-gateway", &"Test Work", &"default", &"default", &"processing", &"tester"],
        ).await.unwrap();
        for i in 1..=2 {
            client.execute(
                "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
                &[&format!("gateway-task-{}", i), &"Gateway Task", &"test-work-gateway", &"review", &"gw-token-1", &"user1", &"completed"],
            ).await.unwrap();
        }

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            "/jaxrs/gateway/test-work-gateway/gw-token-1/join",
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "gateway_join should succeed: {:?}", json);
    }

    #[tokio::test]
    async fn test_work_complete_flow() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        let mut client = pool.get().await.unwrap();
        client.execute("DELETE FROM x_task WHERE work = $1", &[&"test-work-complete-flow"]).await.unwrap();
        client.execute("DELETE FROM x_work WHERE id = $1", &[&"test-work-complete-flow"]).await.unwrap();
        client.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&"test-work-complete-flow", &"Test Work", &"default", &"default", &"processing", &"tester"],
        ).await.unwrap();

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            "/jaxrs/work/test-work-complete-flow/complete",
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "work_complete should succeed: {:?}", json);
    }

    #[tokio::test]
    async fn test_full_e2e_workflow_with_countersign_and_gateway() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool.clone());
        let mut client = pool.get().await.unwrap();

        let work_id = "test-work-e2e-gateway-reject";
        client.execute("DELETE FROM x_record WHERE work_id = $1", &[&work_id]).await.unwrap();
        client.execute("DELETE FROM x_task WHERE work = $1", &[&work_id]).await.unwrap();
        client.execute("DELETE FROM x_workcompleted WHERE work_id = $1", &[&work_id]).await.unwrap();
        client.execute("DELETE FROM x_work WHERE id = $1", &[&work_id]).await.unwrap();

        client.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&work_id, &"E2E Test Work", &"default", &"default", &"pending", &"tester"],
        ).await.unwrap();

        let (status, _) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/work/{}/start", work_id),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "work_start should succeed");

        let start_task_id = "e2e-start-task";
        client.execute("DELETE FROM x_task WHERE id = $1", &[&start_task_id]).await.unwrap();
        client.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&start_task_id, &"Start", &work_id, &"start", &"", &"system", &"completed"],
        ).await.unwrap();

        let countersign_token = "cs-token-1";
        for i in 1..=2 {
            let task_id = format!("e2e-cs-task-{}", i);
            client.execute("DELETE FROM x_task WHERE id = $1", &[&task_id]).await.unwrap();
            client.execute(
                "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
                &[&task_id, &"Countersign", &work_id, &"countersign", &countersign_token, &"user1", &"pending"],
            ).await.unwrap();
        }

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/task/{}/claim", "e2e-cs-task-1"),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "claim first countersign task should succeed: {:?}", json);

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/task/{}/reject", "e2e-cs-task-1"),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "reject first countersign task should succeed: {:?}", json);

        client.execute(
            "UPDATE x_task SET task_status = $1 WHERE id = $2",
            &[&"pending", &"e2e-cs-task-1"],
        ).await.unwrap();

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/task/{}/claim", "e2e-cs-task-1"),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "re-claim first countersign task should succeed: {:?}", json);

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/task/{}/complete", "e2e-cs-task-1"),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "complete first countersign task should succeed: {:?}", json);

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/task/{}/complete", "e2e-cs-task-2"),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "complete second countersign task should succeed: {:?}", json);

        let (status, json) = shared::testing::send_request(
            &app,
            axum::http::Method::POST,
            &format!("/jaxrs/gateway/{}/{}/join", work_id, countersign_token),
            None,
            None,
            None,
        ).await;
        assert_eq!(status, StatusCode::OK, "gateway_join should succeed after countersign: {:?}", json);
    }
}