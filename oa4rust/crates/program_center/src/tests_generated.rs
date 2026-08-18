#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_applications() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/applications")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applications route should be registered");
    }

    #[tokio::test]
    async fn test_current_style() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/appstyle/current/style")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "current_style route should be registered");
    }

    #[tokio::test]
    async fn test_modules_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program/datastructure/modules/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "modules_all route should be registered");
    }

    // SKIPPED: collect_list not accessible
    // SKIPPED: collect_add not accessible
    // SKIPPED: collect_remove not accessible
    // SKIPPED: config_get not accessible
    // SKIPPED: agent_flag not accessible
    // SKIPPED: agent_flag_disable not accessible
    // SKIPPED: agent_flag_enable not accessible
    // SKIPPED: agent_flag_execute not accessible
    // SKIPPED: agent_flag_file not accessible
    // SKIPPED: andfx_pull_sync not accessible
    // SKIPPED: appstyle_current_style not accessible
    // SKIPPED: appstyle_current_update not accessible
    // SKIPPED: appstyle_image_application_top not accessible
    // SKIPPED: appstyle_image_application_top_erase not accessible
    // SKIPPED: appstyle_image_launch_logo not accessible
    // SKIPPED: appstyle_image_launch_logo_erase not accessible
    // SKIPPED: appstyle_image_login_avatar not accessible
    // SKIPPED: appstyle_image_login_avatar_erase not accessible
    // SKIPPED: appstyle_image_menu_logo_blur not accessible
    // SKIPPED: appstyle_image_menu_logo_blur_erase not accessible
    // SKIPPED: appstyle_image_menu_logo_focus not accessible
    // SKIPPED: appstyle_image_menu_logo_focus_erase not accessible
    // SKIPPED: appstyle_image_process_default not accessible
    // SKIPPED: appstyle_image_process_default_erase not accessible
    // SKIPPED: appstyle_image_setup_about_logo not accessible
    // SKIPPED: appstyle_image_setup_about_logo_erase not accessible
    // SKIPPED: appstyle_index_portal not accessible
    // SKIPPED: bar_create_mass_from_count not accessible
    // SKIPPED: bar_select1_field_field_value_value_count_count not accessible
    // SKIPPED: bar_select2_count_count not accessible
    // SKIPPED: bar_select3_field_field_value_value_count_count not accessible
    // SKIPPED: bar_select4_field_field_value_value_count_count not accessible
    // SKIPPED: captcha_list not accessible
    // SKIPPED: captcha_v2_create_width_width_height_height not accessible
    // SKIPPED: captcha_id_validate_answer_answer not accessible
    // SKIPPED: center_applications not accessible
    // SKIPPED: center_regist_applications not accessible
    // SKIPPED: center_version not accessible
    // SKIPPED: code_create_mobile_mobile not accessible
    // SKIPPED: code_list not accessible
    // SKIPPED: code_list_paging_page_size_size not accessible
    // SKIPPED: code_validate_mobile_mobile_answer_answer not accessible
    // SKIPPED: code_validate_mobile_mobile_answer_answer_cascade not accessible
    // SKIPPED: collect_code_mobile_mobile not accessible
    // SKIPPED: collect_connect not accessible
    // SKIPPED: collect_controllebbs not accessible
    // SKIPPED: collect_controllermobile_name_name_mobile_mobile not accessible
    // SKIPPED: collect_disconnect not accessible
    // SKIPPED: collect_login not accessible
    // SKIPPED: collect_mobile_check_connect not accessible
    // SKIPPED: collect_name_name_exist not accessible
    // SKIPPED: collect_name_name_mobile_mobile_code_code not accessible
    // SKIPPED: collect_person not accessible
    // SKIPPED: collect_resetpassword not accessible
    // SKIPPED: collect_sync_area not accessible
    // SKIPPED: collect_updateUnit not accessible
    // SKIPPED: collect_urlMapping not accessible
    // SKIPPED: collect_validate not accessible
    // SKIPPED: collect_validate_codeanswer not accessible
    // SKIPPED: collect_validate_direct not accessible
    // SKIPPED: collect_validate_password not accessible
    // SKIPPED: command_execute not accessible
    // SKIPPED: command_list_node not accessible
    // SKIPPED: config_open_get_disable_export_enable not accessible
    // SKIPPED: config_centerserver not accessible
    // SKIPPED: config_change_password not accessible
    // SKIPPED: config_collect not accessible
    // SKIPPED: config_license not accessible
    // SKIPPED: config_list not accessible
    // SKIPPED: config_list_application not accessible
    // SKIPPED: config_list_dump_data not accessible
    // SKIPPED: config_list_dump_data_current_node not accessible
    // SKIPPED: config_list_entity not accessible
    // SKIPPED: config_open not accessible
    // SKIPPED: config_open_run_time_config not accessible
    // SKIPPED: config_person not accessible
    // SKIPPED: config_portal not accessible
    // SKIPPED: config_proxy not accessible
    // SKIPPED: config_save not accessible
    // SKIPPED: config_ternary_management not accessible
    // SKIPPED: config_token not accessible
    // SKIPPED: datastructure_fileds_all not accessible
    // SKIPPED: datastructure_modules_all not accessible
    // SKIPPED: datastructure_tables_all not accessible
    // SKIPPED: deploy_list_paging_page_size_size not accessible
    // SKIPPED: deploy_server_o2 not accessible
    // SKIPPED: deploy_server_resource not accessible
    // SKIPPED: deploy_web_resource_as_new_asNew not accessible
    // SKIPPED: deploy_id not accessible
    // SKIPPED: designer_search not accessible
    // SKIPPED: dict_list not accessible
    // SKIPPED: dict_list_paging_page_size_size not accessible
    // SKIPPED: dict_dictFlag_data not accessible
    // SKIPPED: dict_dictFlag_path_data not accessible
    // SKIPPED: dict_dictFlag_path_data_mockdeletetoget not accessible
    // SKIPPED: dict_dictFlag_path_data_mockputtopost not accessible
    // SKIPPED: dict_id not accessible
    // SKIPPED: dingding_get_callback_aes not accessible
    // SKIPPED: dingding_pull_sync not accessible
    // SKIPPED: dingding_request_pull_sync not accessible
    // SKIPPED: dingding_sync_organization_callback not accessible
    // SKIPPED: dingding_sync_organization_register_callback_enable not accessible
    // SKIPPED: distribute_assemble_source_source not accessible
    // SKIPPED: distribute_webserver_assemble_source_source not accessible
    // SKIPPED: foo_create_mass_from_count not accessible
    // SKIPPED: input_compare not accessible
    // SKIPPED: input_cover not accessible
    // SKIPPED: input_create not accessible
    // SKIPPED: input_prepare_cover not accessible
    // SKIPPED: input_prepare_create not accessible
    // SKIPPED: invoke_list_category not accessible
    // SKIPPED: invoke_list_with_category_category not accessible
    // SKIPPED: invoke_token not accessible
    // SKIPPED: invoke_flag not accessible
    // SKIPPED: invoke_flag_client_client_token_token_execute not accessible
    // SKIPPED: invoke_flag_execute not accessible
    // SKIPPED: invoke_flag_execute_get not accessible
    // SKIPPED: invoke_flag_file not accessible
    // SKIPPED: jest_center_list not accessible
    // SKIPPED: jest_clear_cache_source not accessible
    // SKIPPED: jest_list not accessible
    // SKIPPED: jest_version not accessible
    // SKIPPED: market_cloud_unit_is_vip not accessible
    // SKIPPED: market_install_offline not accessible
    // SKIPPED: market_list_category not accessible
    // SKIPPED: market_list_install_log_paging_page_size_size not accessible
    // SKIPPED: market_list_paging_page_size_size not accessible
    // SKIPPED: market_list_paging_page_size_size_category_category not accessible
    // SKIPPED: market_list_top_three not accessible
    // SKIPPED: market_flag not accessible
    // SKIPPED: market_flag_cover_pic not accessible
    // SKIPPED: market_flag_install_log not accessible
    // SKIPPED: market_flag_install_or_update not accessible
    // SKIPPED: market_flag_installed_version not accessible
    // SKIPPED: market_flag_uninstall not accessible
    // SKIPPED: market_id_download not accessible
    // SKIPPED: module_compare_upload not accessible
    // SKIPPED: module_list not accessible
    // SKIPPED: module_list_category not accessible
    // SKIPPED: module_output not accessible
    // SKIPPED: module_output_list_structure not accessible
    // SKIPPED: module_output_structure not accessible
    // SKIPPED: module_output_flag_file not accessible
    // SKIPPED: module_remove_structure_id not accessible
    // SKIPPED: module_write_flag not accessible
    // SKIPPED: module_id_compare not accessible
    // SKIPPED: mpweixin_check not accessible
    // SKIPPED: mpweixin_media_add_forever not accessible
    // SKIPPED: mpweixin_menu_add not accessible
    // SKIPPED: mpweixin_menu_create_to_weixin not accessible
    // SKIPPED: mpweixin_menu_delete_id not accessible
    // SKIPPED: mpweixin_menu_list_weixin not accessible
    // SKIPPED: mpweixin_menu_subscribe not accessible
    // SKIPPED: mpweixin_menu_update_id not accessible
    // SKIPPED: mpweixin_message_template_send not accessible
    // SKIPPED: output_list not accessible
    // SKIPPED: output_appInfoFlag_select not accessible
    // SKIPPED: output_flag_select_file not accessible
    // SKIPPED: prompterrorlog_count_exceptionclass not accessible
    // SKIPPED: prompterrorlog_count_loggername not accessible
    // SKIPPED: prompterrorlog_list_id_next_count not accessible
    // SKIPPED: prompterrorlog_list_id_next_count_date_date not accessible
    // SKIPPED: prompterrorlog_list_id_next_count_exceptionclass_exceptionClass not accessible
    // SKIPPED: prompterrorlog_list_id_next_count_loggername_loggerName not accessible
    // SKIPPED: prompterrorlog_list_id_prev_count not accessible
    // SKIPPED: prompterrorlog_list_id_prev_count_date_date not accessible
    // SKIPPED: prompterrorlog_list_id_prev_count_exceptionclass_exceptionClass not accessible
    // SKIPPED: prompterrorlog_list_id_prev_count_loggername_loggerName not accessible
    // SKIPPED: prompterrorlog_id not accessible
    // SKIPPED: qiyeweixin_get_callback_aes not accessible
    // SKIPPED: qiyeweixin_pull_sync not accessible
    // SKIPPED: qiyeweixin_request_pull_sync not accessible
    // SKIPPED: qiyeweixin_send_getprivateinfo_message not accessible
    #[tokio::test]
    async fn test_application_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_create route should be registered");
    }

    #[tokio::test]
    async fn test_application_save() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/application/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_save route should be registered");
    }

    #[tokio::test]
    async fn test_agent_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "agent_create route should be registered");
    }

    #[tokio::test]
    async fn test_agent_save() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/program_center/agent/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "agent_save route should be registered");
    }

    // SKIPPED: schedule_list_schedule not accessible
    // SKIPPED: schedule_list_schedulelocal not accessible
    // SKIPPED: schedule_list_schedulelog_application_application not accessible
    // SKIPPED: schedule_report not accessible
    // SKIPPED: schedule_schedule_fire not accessible
    // SKIPPED: script_list not accessible
    // SKIPPED: script_list_paging_page_size_size not accessible
    // SKIPPED: script_name_name not accessible
    // SKIPPED: script_name_name_imported not accessible
    // SKIPPED: script_flag not accessible
    // SKIPPED: script_id not accessible
    // SKIPPED: tokenthreshold_update not accessible
    // SKIPPED: unexpectederrorlog_list_id_next_count not accessible
    // SKIPPED: unexpectederrorlog_list_id_next_count_date_date not accessible
    // SKIPPED: unexpectederrorlog_list_id_prev_count not accessible
    // SKIPPED: unexpectederrorlog_list_id_prev_count_date_date not accessible
    // SKIPPED: unexpectederrorlog_id not accessible
    // SKIPPED: validation_meta not accessible
    // SKIPPED: validation_scripting_benchmark not accessible
    // SKIPPED: validation_timeout_timeout not accessible
    // SKIPPED: zhengwudingding_pull_sync not accessible
    // SKIPPED: zhengwudingding_regist_callback not accessible
    // SKIPPED: zhengwudingding_sync_organization_callback not accessible
}