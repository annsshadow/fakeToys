#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_bam_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/bam/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_bam_config route should be registered");
    }

    #[tokio::test]
    async fn test_create_bam() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/bam/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_bam route should be registered");
    }

    #[tokio::test]
    async fn test_list_bams() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/bam/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_bams route should be registered");
    }

    #[tokio::test]
    async fn test_delete_bam() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/bam/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_bam route should be registered");
    }

    #[tokio::test]
    async fn test_get_bam_status() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/bam/status/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_bam_status route should be registered");
    }

    // SKIPPED: period_list_completed_task_application not accessible
    // SKIPPED: period_list_completed_task_unit not accessible
    // SKIPPED: period_list_completed_work_application not accessible
    // SKIPPED: period_list_completed_work_unit not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_by_unit not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_process_processId_unit_unit_person_person_by_activity not accessible
    // SKIPPED: period_list_count_completed_task_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_completed_task_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_completed_work_application_applicationId_process_processId_by_unit not accessible
    // SKIPPED: period_list_count_completed_work_application_applicationId_process_processId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_completed_work_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_completed_work_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_by_unit not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_process_processId_unit_unit_person_person_by_activity not accessible
    // SKIPPED: period_list_count_expired_task_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_expired_task_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_expired_work_application_applicationId_process_processId_by_unit not accessible
    // SKIPPED: period_list_count_expired_work_application_applicationId_process_processId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_expired_work_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_expired_work_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_process_processId_activity_activityId_by_unit not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_process_processId_unit_unit_person_person_by_activity not accessible
    // SKIPPED: period_list_count_start_task_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_start_task_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_count_start_work_application_applicationId_process_processId_by_unit not accessible
    // SKIPPED: period_list_count_start_work_application_applicationId_process_processId_unit_unit_person_person not accessible
    // SKIPPED: period_list_count_start_work_application_applicationId_unit_unit_person_person_by_process not accessible
    // SKIPPED: period_list_count_start_work_unit_unit_person_person_by_application not accessible
    // SKIPPED: period_list_expired_task_application not accessible
    // SKIPPED: period_list_expired_task_unit not accessible
    // SKIPPED: period_list_expired_work_application not accessible
    // SKIPPED: period_list_expired_work_unit not accessible
    // SKIPPED: period_list_start_task_application not accessible
    // SKIPPED: period_list_start_task_unit not accessible
    // SKIPPED: period_list_start_work_application not accessible
    // SKIPPED: period_list_start_work_unit not accessible
    // SKIPPED: state_applicationtstubs_trigger not accessible
    // SKIPPED: state_category not accessible
    // SKIPPED: state_category_trigger not accessible
    // SKIPPED: state_organization not accessible
    // SKIPPED: state_running not accessible
    // SKIPPED: state_summary not accessible
}