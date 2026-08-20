#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    async fn ensure_test_surface(pool: &deadpool_postgres::Pool) {
        let client = pool.get().await.unwrap();
        let existing: i64 = client
            .query_one("SELECT COUNT(*)::bigint FROM x_process_surface WHERE id = $1", &[&"test-id"])
            .await
            .unwrap()
            .get(0);
        if existing == 0 {
            let _ = client.execute(
                "INSERT INTO x_process_surface (id, name, category, content, version, creator, create_time, update_time)
                 VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
                 ON CONFLICT (id) DO NOTHING",
                &[&"test-id", &"Test Surface", &"processplatform", &r#"{"html":"<div>test</div>"}"#, &"1.0", &"test"],
            ).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_get_surface() {
        let pool = shared::testing::test_pool();
        ensure_test_surface(&pool).await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/get/test-id")
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
                    .uri("/jaxrs/processplatform/assemble/surface/create")
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
                    .uri("/jaxrs/processplatform/assemble/surface/list/test-id")
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
    async fn test_preview_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/preview/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "preview_surface route should be registered");
    }

    #[tokio::test]
    async fn test_publish_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/publish/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "publish_surface route should be registered");
    }

    #[tokio::test]
    async fn test_delete_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/delete/test-id")
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
    async fn test_save_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_surface route should be registered");
    }

    // SKIPPED: anonymous_read_count_credential not accessible
    // SKIPPED: anonymous_task_count_credential not accessible
    // SKIPPED: application_list not accessible
    // SKIPPED: application_list_complex not accessible
    // SKIPPED: application_list_complex_manage_person not accessible
    // SKIPPED: application_list_key_key not accessible
    // SKIPPED: application_list_range not accessible
    // SKIPPED: application_list_terminal_terminal not accessible
    // SKIPPED: application_flag not accessible
    // SKIPPED: application_flag_icon not accessible
    // SKIPPED: application_flag_is_manager not accessible
    // SKIPPED: application_flag_onlyRemoveNotCompleted not accessible
    // SKIPPED: applicationdict_list_application_applicationFlag not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget not accessible
    // SKIPPED: applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost not accessible
    // SKIPPED: control_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: correlation_job_job not accessible
    // SKIPPED: correlation_job_job_delete not accessible
    // SKIPPED: correlation_list_job_job not accessible
    // SKIPPED: correlation_list_job_job_site_site not accessible
    // SKIPPED: correlation_update_job_job not accessible
    // SKIPPED: data_fetch_job_job not accessible
    // SKIPPED: data_job_job not accessible
    // SKIPPED: data_job_job_array_data not accessible
    // SKIPPED: data_job_job_mockputtopost not accessible
    // SKIPPED: data_job_job_path0 not accessible
    // SKIPPED: data_job_job_path0_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1 not accessible
    // SKIPPED: data_job_job_path0_path1_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1_path2 not accessible
    // SKIPPED: data_job_job_path0_path1_path2_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3 not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4 not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_path5 not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_path5_path6 not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_path5_path6_path7 not accessible
    // SKIPPED: data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost not accessible
    // SKIPPED: data_work_id not accessible
    // SKIPPED: data_work_id_mockdeletetoget not accessible
    // SKIPPED: data_work_id_mockputtopost not accessible
    // SKIPPED: data_work_id_path0 not accessible
    // SKIPPED: data_work_id_path0_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1 not accessible
    // SKIPPED: data_work_id_path0_path1_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1_path2 not accessible
    // SKIPPED: data_work_id_path0_path1_path2_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_path2_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3 not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4 not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5 not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_path6 not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_path6_path7 not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget not accessible
    // SKIPPED: data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id not accessible
    // SKIPPED: data_workcompleted_id_from_data not accessible
    // SKIPPED: data_workcompleted_id_from_item not accessible
    // SKIPPED: data_workcompleted_id_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0 not accessible
    // SKIPPED: data_workcompleted_id_path0_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_path5 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7 not accessible
    // SKIPPED: data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost not accessible
    // SKIPPED: datarecord_get_job_job_path_path not accessible
    // SKIPPED: datarecord_list_job_job not accessible
    // SKIPPED: documentversion_list_job_job not accessible
    // SKIPPED: documentversion_list_job_job_category_category not accessible
    // SKIPPED: documentversion_list_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: documentversion_list_workorworkcompleted_workOrWorkCompleted_category_category not accessible
    // SKIPPED: documentversion_work_work not accessible
    // SKIPPED: documentversion_id not accessible
    // SKIPPED: draft_list_my_paging_page_size_size not accessible
    // SKIPPED: draft_list_id_next_count not accessible
    // SKIPPED: draft_list_id_prev_count not accessible
    // SKIPPED: draft_mockputtopost not accessible
    // SKIPPED: draft_process_processFlag not accessible
    // SKIPPED: draft_id not accessible
    // SKIPPED: draft_id_mockdeletetoget not accessible
    // SKIPPED: draft_id_start not accessible
    // SKIPPED: file_list_application_applicationFlag not accessible
    // SKIPPED: file_flag_application_applicationFlag_content not accessible
    // SKIPPED: file_flag_application_applicationFlag_download not accessible
    // SKIPPED: form_v2_lookup_taskcompleted_taskcompleted not accessible
    // SKIPPED: form_v2_lookup_taskcompleted_taskcompleted_mobile not accessible
    // SKIPPED: form_v2_lookup_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: form_v2_lookup_workorworkcompleted_workOrWorkCompleted_mobile not accessible
    // SKIPPED: form_v2_id not accessible
    // SKIPPED: form_v2_id_mobile not accessible
    // SKIPPED: form_flag not accessible
    // SKIPPED: form_flag_application_applicationFlag not accessible
    // SKIPPED: form_flag_application_applicationFlag_mobile not accessible
    // SKIPPED: form_flag_mobile not accessible
    // SKIPPED: handover_list_paging_page_size_size not accessible
    // SKIPPED: handover_id not accessible
    // SKIPPED: handover_id_cancel not accessible
    // SKIPPED: handover_id_process not accessible
    // SKIPPED: job_latest_work_workcompleted_serial_serial not accessible
    // SKIPPED: job_v2_job_projection not accessible
    // SKIPPED: job_job_allow_visit_person_person not accessible
    // SKIPPED: job_job_find_work_workcompleted not accessible
    // SKIPPED: keylock_lock not accessible
    // SKIPPED: keylock_lock_mockputtopost not accessible
    // SKIPPED: mode_clear_person_person_manager not accessible
    // SKIPPED: mode_list not accessible
    // SKIPPED: mode_save not accessible
    // SKIPPED: mode_id_delete not accessible
    // SKIPPED: process_activity_activity_activityType_activityType not accessible
    // SKIPPED: process_list_application_applicationFlag not accessible
    // SKIPPED: process_list_application_applicationFlag_filter not accessible
    // SKIPPED: process_list_available_identity_process_flag not accessible
    // SKIPPED: process_list_controllable_application_applicationFlag not accessible
    // SKIPPED: process_list_ids not accessible
    // SKIPPED: process_flag not accessible
    // SKIPPED: process_flag_allowrerouteto not accessible
    // SKIPPED: process_flag_application_applicationFlag not accessible
    // SKIPPED: process_flag_complex not accessible
    // SKIPPED: process_flag_onlyRemoveNotCompleted not accessible
    // SKIPPED: read_count_filter not accessible
    // SKIPPED: read_count_credential not accessible
    // SKIPPED: read_filter_attribute not accessible
    // SKIPPED: read_filter_attribute_filter not accessible
    // SKIPPED: read_list_count_application not accessible
    // SKIPPED: read_list_count_application_applicationFlag_process not accessible
    // SKIPPED: read_list_date_date_manage not accessible
    // SKIPPED: read_list_filter_page_size_size_manage not accessible
    // SKIPPED: read_list_job_job not accessible
    // SKIPPED: read_list_my_filter_page_size_size not accessible
    // SKIPPED: read_list_my_paging_page_size_size not accessible
    // SKIPPED: read_list_person_person_manage not accessible
    // SKIPPED: read_list_work_work not accessible
    // SKIPPED: read_list_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: read_list_id_next_count not accessible
    // SKIPPED: read_list_id_next_count_application_applicationFlag not accessible
    // SKIPPED: read_list_id_next_count_filter not accessible
    // SKIPPED: read_list_id_next_count_process_processFlag not accessible
    // SKIPPED: read_list_id_prev_count not accessible
    // SKIPPED: read_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: read_list_id_prev_count_filter not accessible
    // SKIPPED: read_list_id_prev_count_process_processFlag not accessible
    // SKIPPED: read_v2_count not accessible
    // SKIPPED: read_v2_list not accessible
    // SKIPPED: read_v2_list_create_paging_page_size_size not accessible
    // SKIPPED: read_v2_list_create_id_next_count not accessible
    // SKIPPED: read_v2_list_create_id_prev_count not accessible
    // SKIPPED: read_v2_list_paging_page_size_size not accessible
    // SKIPPED: read_v2_list_id_next_count not accessible
    // SKIPPED: read_v2_list_id_prev_count not accessible
    // SKIPPED: read_work_workId not accessible
    // SKIPPED: read_workcompleted_workCompletedId not accessible
    // SKIPPED: read_id not accessible
    // SKIPPED: read_id_manage not accessible
    // SKIPPED: read_id_manage_mockdeletetoget not accessible
    // SKIPPED: read_id_mockputtopost not accessible
    // SKIPPED: read_id_opinion_manage not accessible
    // SKIPPED: read_id_opinion_manage_mockputtopost not accessible
    // SKIPPED: read_id_processing not accessible
    // SKIPPED: read_id_processing_manage not accessible
    // SKIPPED: read_id_processing_manage_mockputtopost not accessible
    // SKIPPED: read_id_reference not accessible
    // SKIPPED: read_id_reset_manage not accessible
    // SKIPPED: read_id_reset_manage_mockputtopost not accessible
    // SKIPPED: readcompleted_count_credential not accessible
    // SKIPPED: readcompleted_filter_attribute not accessible
    // SKIPPED: readcompleted_filter_attribute_filter not accessible
    // SKIPPED: readcompleted_list_count_application not accessible
    // SKIPPED: readcompleted_list_count_application_applicationFlag_process not accessible
    // SKIPPED: readcompleted_list_date_date_manage not accessible
    // SKIPPED: readcompleted_list_filter_page_size_size_manage not accessible
    // SKIPPED: readcompleted_list_job_job not accessible
    // SKIPPED: readcompleted_list_my_filter_page_size_size not accessible
    // SKIPPED: readcompleted_list_my_paging_page_size_size not accessible
    // SKIPPED: readcompleted_list_work_work not accessible
    // SKIPPED: readcompleted_list_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: readcompleted_list_id_next_count not accessible
    // SKIPPED: readcompleted_list_id_next_count_application_applicationFlag not accessible
    // SKIPPED: readcompleted_list_id_next_count_filter not accessible
    // SKIPPED: readcompleted_list_id_next_count_process_processFlag not accessible
    // SKIPPED: readcompleted_list_id_prev_count not accessible
    // SKIPPED: readcompleted_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: readcompleted_list_id_prev_count_filter not accessible
    // SKIPPED: readcompleted_list_id_prev_count_process_processFlag not accessible
    // SKIPPED: readcompleted_v2_count not accessible
    // SKIPPED: readcompleted_v2_list not accessible
    // SKIPPED: readcompleted_v2_list_create_paging_page_size_size not accessible
    // SKIPPED: readcompleted_v2_list_create_id_next_count not accessible
    // SKIPPED: readcompleted_v2_list_create_id_prev_count not accessible
    // SKIPPED: readcompleted_v2_list_paging_page_size_size not accessible
    // SKIPPED: readcompleted_v2_list_id_next_count not accessible
    // SKIPPED: readcompleted_v2_list_id_prev_count not accessible
    // SKIPPED: readcompleted_id not accessible
    // SKIPPED: readcompleted_id_manage not accessible
    // SKIPPED: readcompleted_id_manage_mockdeletetoget not accessible
    // SKIPPED: readcompleted_id_opinion_manage not accessible
    // SKIPPED: readcompleted_id_reference not accessible
    // SKIPPED: readrecord_list_job_job not accessible
    // SKIPPED: readrecord_list_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: record_job_job_manage not accessible
    // SKIPPED: record_list_job_job not accessible
    // SKIPPED: record_list_job_job_paging_page_size_size not accessible
    // SKIPPED: record_list_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: record_list_workorworkcompleted_workOrWorkCompleted_paging_page_size_size not accessible
    // SKIPPED: record_id_manage not accessible
    // SKIPPED: record_id_manage_mockdeletetoget not accessible
    // SKIPPED: record_id_manage_mockputtopost not accessible
    // SKIPPED: review_count_application not accessible
    // SKIPPED: review_count_person_credential not accessible
    // SKIPPED: review_create_work not accessible
    // SKIPPED: review_create_workcompleted not accessible
    // SKIPPED: review_filter_attribute not accessible
    // SKIPPED: review_filter_create_entry not accessible
    // SKIPPED: review_filter_entry not accessible
    // SKIPPED: review_list_job_job not accessible
    // SKIPPED: review_v2_count not accessible
    // SKIPPED: review_v2_list not accessible
    // SKIPPED: review_v2_list_create_paging_page_size_size not accessible
    // SKIPPED: review_v2_list_create_id_next_count not accessible
    // SKIPPED: review_v2_list_create_id_prev_count not accessible
    // SKIPPED: review_v2_list_paging_page_size_size not accessible
    // SKIPPED: review_v2_list_paging_page_size_size_manage not accessible
    // SKIPPED: review_v2_list_id_next_count not accessible
    // SKIPPED: review_v2_list_id_prev_count not accessible
    // SKIPPED: review_v2_search not accessible
    // SKIPPED: review_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: review_id not accessible
    // SKIPPED: review_id_application_applicationFlag_manage not accessible
    // SKIPPED: review_id_application_applicationFlag_manage_mockdeletetoget not accessible
    // SKIPPED: route_list not accessible
    // SKIPPED: route_list_mockputtopost not accessible
    // SKIPPED: route_id not accessible
    // SKIPPED: route_id_selectconfig not accessible
    // SKIPPED: script_flag_application_applicationFlag not accessible
    // SKIPPED: script_flag_application_applicationFlag_imported not accessible
    // SKIPPED: serialnumber_generate_process_processId_name_name_serial not accessible
    // SKIPPED: serialnumber_list_application_applicationFlag not accessible
    // SKIPPED: serialnumber_list_paging_page_size_size not accessible
    // SKIPPED: serialnumber_id not accessible
    // SKIPPED: serialnumber_id_mockdeletetoget not accessible
    // SKIPPED: serialnumber_id_mockputtopost not accessible
    // SKIPPED: service_work_id_touch not accessible
    // SKIPPED: service_work_id_touch_mockputtopost not accessible
    // SKIPPED: sign_download_scrawlId not accessible
    // SKIPPED: sign_list_job_job not accessible
    // SKIPPED: sign_save_task_taskId not accessible
    // SKIPPED: sign_task_taskId not accessible
    // SKIPPED: sign_task_taskId_mockdeletetoget not accessible
    // SKIPPED: sign_id not accessible
    // SKIPPED: sign_id_mockdeletetoget not accessible
    // SKIPPED: task_count_filter not accessible
    // SKIPPED: task_count_credential not accessible
    // SKIPPED: task_filter_attribute not accessible
    // SKIPPED: task_filter_attribute_filter not accessible
    // SKIPPED: task_list_count_application not accessible
    // SKIPPED: task_list_count_application_applicationFlag_process not accessible
    // SKIPPED: task_list_date_date_hour_hour_exclude_draft_isExcludeDraft_manage not accessible
    // SKIPPED: task_list_filter_page_size_size_manage not accessible
    // SKIPPED: task_list_job_job not accessible
    // SKIPPED: task_list_my_filter_page_size_size not accessible
    // SKIPPED: task_list_my_paging_page_size_size not accessible
    // SKIPPED: task_list_person_person_exclude_draft_isExcludeDraft_manage not accessible
    // SKIPPED: task_list_work_work not accessible
    // SKIPPED: task_list_id_next_count not accessible
    // SKIPPED: task_list_id_next_count_application_applicationFlag not accessible
    // SKIPPED: task_list_id_next_count_filter not accessible
    // SKIPPED: task_list_id_next_count_filter_manage not accessible
    // SKIPPED: task_list_id_next_count_manage not accessible
    // SKIPPED: task_list_id_next_count_process_processFlag not accessible
    // SKIPPED: task_list_id_prev_count not accessible
    // SKIPPED: task_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: task_list_id_prev_count_filter not accessible
    // SKIPPED: task_list_id_prev_count_filter_manage not accessible
    // SKIPPED: task_list_id_prev_count_manage not accessible
    // SKIPPED: task_list_id_prev_count_process_processFlag not accessible
    // SKIPPED: task_v2_count not accessible
    // SKIPPED: task_v2_list not accessible
    // SKIPPED: task_v2_list_create_paging_page_size_size not accessible
    // SKIPPED: task_v2_list_create_id_next_count not accessible
    // SKIPPED: task_v2_list_create_id_prev_count not accessible
    // SKIPPED: task_v2_list_paging_page_size_size not accessible
    // SKIPPED: task_v2_list_id_next_count not accessible
    // SKIPPED: task_v2_list_id_prev_count not accessible
    // SKIPPED: task_v2_id_pause not accessible
    // SKIPPED: task_v2_id_reset not accessible
    // SKIPPED: task_v2_id_reset_mockputtopost not accessible
    // SKIPPED: task_v2_id_resume not accessible
    // SKIPPED: task_v2_id_trigger_processing not accessible
    // SKIPPED: task_v3_id_add not accessible
    // SKIPPED: task_v3_id_pin not accessible
    // SKIPPED: task_id not accessible
    // SKIPPED: task_id_manage not accessible
    // SKIPPED: task_id_manage_mockdeletetoget not accessible
    // SKIPPED: task_id_mockputtopost not accessible
    // SKIPPED: task_id_opinion_manage not accessible
    // SKIPPED: task_id_opinion_manage_mockputtopost not accessible
    // SKIPPED: task_id_press_manage not accessible
    // SKIPPED: task_id_processing not accessible
    // SKIPPED: task_id_processing_manage not accessible
    // SKIPPED: task_id_processing_manage_mockputtopost not accessible
    // SKIPPED: task_id_processing_neural not accessible
    // SKIPPED: task_id_reference not accessible
    // SKIPPED: task_id_reset_manage not accessible
    // SKIPPED: task_id_reset_manage_mockputtopost not accessible
    // SKIPPED: task_id_will not accessible
    // SKIPPED: taskcompleted_count_credential not accessible
    // SKIPPED: taskcompleted_filter_attribute not accessible
    // SKIPPED: taskcompleted_filter_attribute_filter not accessible
    // SKIPPED: taskcompleted_list_count_application not accessible
    // SKIPPED: taskcompleted_list_count_application_applicationFlag_process not accessible
    // SKIPPED: taskcompleted_list_date_date_hour_hour_manage not accessible
    // SKIPPED: taskcompleted_list_filter_page_size_size_manage not accessible
    // SKIPPED: taskcompleted_list_job_job not accessible
    // SKIPPED: taskcompleted_list_my_filter_page_size_size not accessible
    // SKIPPED: taskcompleted_list_my_paging_page_size_size not accessible
    // SKIPPED: taskcompleted_list_prev_manual_flag not accessible
    // SKIPPED: taskcompleted_list_work_work not accessible
    // SKIPPED: taskcompleted_list_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: taskcompleted_list_id_next_count not accessible
    // SKIPPED: taskcompleted_list_id_next_count_application_applicationFlag not accessible
    // SKIPPED: taskcompleted_list_id_next_count_filter not accessible
    // SKIPPED: taskcompleted_list_id_next_count_process_processFlag not accessible
    // SKIPPED: taskcompleted_list_id_prev_count not accessible
    // SKIPPED: taskcompleted_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: taskcompleted_list_id_prev_count_filter not accessible
    // SKIPPED: taskcompleted_list_id_prev_count_process_processFlag not accessible
    // SKIPPED: taskcompleted_press_work_work not accessible
    // SKIPPED: taskcompleted_v2_count not accessible
    // SKIPPED: taskcompleted_v2_list not accessible
    // SKIPPED: taskcompleted_v2_list_create_paging_page_size_size not accessible
    // SKIPPED: taskcompleted_v2_list_create_id_next_count not accessible
    // SKIPPED: taskcompleted_v2_list_create_id_prev_count not accessible
    // SKIPPED: taskcompleted_v2_list_paging_page_size_size not accessible
    // SKIPPED: taskcompleted_v2_list_id_next_count not accessible
    // SKIPPED: taskcompleted_v2_list_id_prev_count not accessible
    // SKIPPED: taskcompleted_id not accessible
    // SKIPPED: taskcompleted_id_manage not accessible
    // SKIPPED: taskcompleted_id_manage_mockdeletetoget not accessible
    // SKIPPED: taskcompleted_id_opinion_manage not accessible
    // SKIPPED: taskcompleted_id_opinion_manage_mockputtopost not accessible
    // SKIPPED: taskcompleted_id_reference not accessible
    // SKIPPED: taskcompleted_id_reference_control not accessible
    // SKIPPED: touch_expire not accessible
    // SKIPPED: touch_passexpired not accessible
    // SKIPPED: touch_touchdetained not accessible
    // SKIPPED: work_application_applicationFlag_process_processFlag not accessible
    // SKIPPED: work_count_credential not accessible
    // SKIPPED: work_count_credential_application_appId not accessible
    // SKIPPED: work_filter_attribute_application_applicationFlag not accessible
    // SKIPPED: work_filter_attribute_application_applicationFlag_manage not accessible
    // SKIPPED: work_list_count_application not accessible
    // SKIPPED: work_list_count_application_applicationFlag_process not accessible
    // SKIPPED: work_list_count_application_applicationFlag_process_manage not accessible
    // SKIPPED: work_list_filter_page_size_size_manage not accessible
    // SKIPPED: work_list_my_paging_page_size_size not accessible
    // SKIPPED: work_list_paging_page_size_size_application_applicationFlag_filter_manage not accessible
    // SKIPPED: work_list_id_next_count_application_applicationFlag not accessible
    // SKIPPED: work_list_id_next_count_application_applicationFlag_filter not accessible
    // SKIPPED: work_list_id_next_count_application_applicationFlag_filter_manage not accessible
    // SKIPPED: work_list_id_next_count_application_applicationFlag_manage not accessible
    // SKIPPED: work_list_id_next_count_creator_current not accessible
    // SKIPPED: work_list_id_next_count_creator_current_filter not accessible
    // SKIPPED: work_list_id_next_count_process_processFlag not accessible
    // SKIPPED: work_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: work_list_id_prev_count_application_applicationFlag_filter not accessible
    // SKIPPED: work_list_id_prev_count_application_applicationFlag_filter_manage not accessible
    // SKIPPED: work_list_id_prev_count_application_applicationFlag_manage not accessible
    // SKIPPED: work_list_id_prev_count_creator_current not accessible
    // SKIPPED: work_list_id_prev_count_creator_current_filter not accessible
    // SKIPPED: work_list_id_prev_count_process_processFlag not accessible
    // SKIPPED: work_process_processFlag not accessible
    // SKIPPED: work_process_processFlag_force not accessible
    // SKIPPED: work_v2_list not accessible
    // SKIPPED: work_v2_list_paging_page_size_size not accessible
    // SKIPPED: work_v2_list_id_activity_goback not accessible
    // SKIPPED: work_v2_list_id_next_count not accessible
    // SKIPPED: work_v2_list_id_prev_count not accessible
    // SKIPPED: work_v2_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: work_v2_id_add_split not accessible
    // SKIPPED: work_v2_id_add_split_mockputtopost not accessible
    // SKIPPED: work_v2_id_reroute not accessible
    // SKIPPED: work_v2_id_reroute_mockputtopost not accessible
    // SKIPPED: work_v2_id_retract not accessible
    // SKIPPED: work_v2_id_retract_mockputtopost not accessible
    // SKIPPED: work_v2_id_rollback not accessible
    // SKIPPED: work_v2_id_rollback_mockputtopost not accessible
    // SKIPPED: work_v2_id_terminate not accessible
    // SKIPPED: work_v2_id_terminate_manage not accessible
    // SKIPPED: work_v2_id_trigger_processing not accessible
    // SKIPPED: work_v3_retract not accessible
    // SKIPPED: work_v3_retract_stage_job_job not accessible
    // SKIPPED: work_v3_workorworkcompleted_workOrWorkCompleted_permission not accessible
    // SKIPPED: work_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: work_id not accessible
    // SKIPPED: work_id_assignment_manage not accessible
    // SKIPPED: work_id_close_check not accessible
    // SKIPPED: work_id_manage not accessible
    // SKIPPED: work_id_mockdeletetoget not accessible
    // SKIPPED: work_id_processing not accessible
    // SKIPPED: work_id_processing_mockputtopost not accessible
    // SKIPPED: work_id_projection not accessible
    // SKIPPED: work_id_refer not accessible
    // SKIPPED: work_id_relative_manage not accessible
    // SKIPPED: work_id_relative_manage_mockdeletetoget not accessible
    // SKIPPED: work_id_single_manage not accessible
    // SKIPPED: work_id_single_manage_mockdeletetoget not accessible
    // SKIPPED: workcompleted_filter_attribute_application_applicationFlag not accessible
    // SKIPPED: workcompleted_filter_attribute_application_applicationFlag_manage not accessible
    // SKIPPED: workcompleted_filter_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: workcompleted_list_count_application not accessible
    // SKIPPED: workcompleted_list_count_application_applicationFlag_process not accessible
    // SKIPPED: workcompleted_list_count_application_applicationFlag_process_manage not accessible
    // SKIPPED: workcompleted_list_filter_page_size_size_manage not accessible
    // SKIPPED: workcompleted_list_paging_page_size_size_application_applicationFlag_filter_manage not accessible
    // SKIPPED: workcompleted_list_id_next_count_application_applicationFlag not accessible
    // SKIPPED: workcompleted_list_id_next_count_application_applicationFlag_filter not accessible
    // SKIPPED: workcompleted_list_id_next_count_application_applicationFlag_filter_manage not accessible
    // SKIPPED: workcompleted_list_id_next_count_application_applicationFlag_manage not accessible
    // SKIPPED: workcompleted_list_id_prev_count_application_applicationFlag not accessible
    // SKIPPED: workcompleted_list_id_prev_count_application_applicationFlag_filter not accessible
    // SKIPPED: workcompleted_list_id_prev_count_application_applicationFlag_manage not accessible
    // SKIPPED: workcompleted_process_processFlag not accessible
    // SKIPPED: workcompleted_shift_time not accessible
    // SKIPPED: workcompleted_flag_rollback not accessible
    // SKIPPED: workcompleted_flag_rollback_mockputtopost not accessible
    // SKIPPED: workcompleted_id not accessible
    // SKIPPED: workcompleted_id_assignment_manage not accessible
    // SKIPPED: workcompleted_id_delete_manage not accessible
    // SKIPPED: workcompleted_id_delete_manage_mockdeletetoget not accessible
    // SKIPPED: workcompleted_id_manage not accessible
    // SKIPPED: worklog_list_add_split_work_workId not accessible
    // SKIPPED: worklog_list_job_job not accessible
    // SKIPPED: worklog_list_rollback_workorworkcompleted_workOrWorkCompleted not accessible
    // SKIPPED: worklog_list_workorworkcompleted_workOrWorkCompleted not accessible
}