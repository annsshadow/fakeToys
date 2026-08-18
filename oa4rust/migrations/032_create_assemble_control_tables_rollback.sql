-- Rollback for 032_create_assemble_control_tables.sql

DROP TABLE IF EXISTS "application";

DROP TABLE IF EXISTS "bbs_forum_info";

DROP TABLE IF EXISTS "bbs_section_info";

DROP TABLE IF EXISTS "bbs_subject_info";

DROP TABLE IF EXISTS "cal_calendar";

DROP TABLE IF EXISTS "cal_control_config";

DROP TABLE IF EXISTS "cal_event";

DROP TABLE IF EXISTS "corr_c_correlation";

DROP TABLE IF EXISTS "cpt_component";

DROP TABLE IF EXISTS "district";

DROP TABLE IF EXISTS "gen_dict";

DROP TABLE IF EXISTS "mind_base_info";

DROP TABLE IF EXISTS "mind_folder_info";

DROP TABLE IF EXISTS "mind_version_info";

DROP TABLE IF EXISTS "oa_process";

DROP TABLE IF EXISTS "oa_workcompleted";

DROP TABLE IF EXISTS "org_unit";

DROP TABLE IF EXISTS "pg_class";

DROP TABLE IF EXISTS "pp_c_data_record";

DROP TABLE IF EXISTS "pp_c_doc_sign";

DROP TABLE IF EXISTS "pp_c_documentversion";

DROP TABLE IF EXISTS "pp_c_draft";

DROP TABLE IF EXISTS "pp_c_handover";

DROP TABLE IF EXISTS "pp_c_job";

DROP TABLE IF EXISTS "pp_c_keylock";

DROP TABLE IF EXISTS "pp_c_read";

DROP TABLE IF EXISTS "pp_c_readcompleted";

DROP TABLE IF EXISTS "pp_c_record";

DROP TABLE IF EXISTS "pp_c_review";

DROP TABLE IF EXISTS "pp_c_serialnumber";

DROP TABLE IF EXISTS "pp_c_task";

DROP TABLE IF EXISTS "pp_c_task_process_mode";

DROP TABLE IF EXISTS "pp_c_taskcompleted";

DROP TABLE IF EXISTS "pp_c_work";

DROP TABLE IF EXISTS "pp_c_workcompleted";

DROP TABLE IF EXISTS "pp_c_worklog";

DROP TABLE IF EXISTS "pp_e_application";

DROP TABLE IF EXISTS "pp_e_applicationcategory";

DROP TABLE IF EXISTS "pp_e_applicationdict";

DROP TABLE IF EXISTS "pp_e_file";

DROP TABLE IF EXISTS "pp_e_form";

DROP TABLE IF EXISTS "pp_e_formversion";

DROP TABLE IF EXISTS "pp_e_item_access";

DROP TABLE IF EXISTS "pp_e_mapping";

DROP TABLE IF EXISTS "pp_e_mergeitemplan";

DROP TABLE IF EXISTS "pp_e_output";

DROP TABLE IF EXISTS "pp_e_process";

DROP TABLE IF EXISTS "pp_e_process_activity";

DROP TABLE IF EXISTS "pp_e_process_element";

DROP TABLE IF EXISTS "pp_e_processversion";

DROP TABLE IF EXISTS "pp_e_route";

DROP TABLE IF EXISTS "pp_e_script";

DROP TABLE IF EXISTS "pp_e_scriptversion";

DROP TABLE IF EXISTS "pp_e_templateform";

DROP TABLE IF EXISTS "pp_e_workcompleted";

DROP TABLE IF EXISTS "process_application";

DROP TABLE IF EXISTS "query_import";

DROP TABLE IF EXISTS "query_item";

DROP TABLE IF EXISTS "query_view";

DROP TABLE IF EXISTS "route";

DROP TABLE IF EXISTS "set";

DROP TABLE IF EXISTS "sub";

DROP TABLE IF EXISTS "sup";

DROP TABLE IF EXISTS "x_ai_file";

DROP TABLE IF EXISTS "x_ai_index";

DROP TABLE IF EXISTS "x_ai_mcp_config";

DROP TABLE IF EXISTS "x_ai_model_config";

DROP TABLE IF EXISTS "x_applications";

DROP TABLE IF EXISTS "x_attendance_admin";

DROP TABLE IF EXISTS "x_attendance_appeal_info";

DROP TABLE IF EXISTS "x_attendance_assemble_control_rule";

DROP TABLE IF EXISTS "x_attendance_config";

DROP TABLE IF EXISTS "x_attendance_detail";

DROP TABLE IF EXISTS "x_attendance_employee_config";

DROP TABLE IF EXISTS "x_attendance_import_file_info";

DROP TABLE IF EXISTS "x_attendance_schedule_setting";

DROP TABLE IF EXISTS "x_attendance_selfholiday";

DROP TABLE IF EXISTS "x_attendance_setting";

DROP TABLE IF EXISTS "x_attendance_statistic";

DROP TABLE IF EXISTS "x_attendance_statistic_require_log";

DROP TABLE IF EXISTS "x_attendance_statistical_cycle";

DROP TABLE IF EXISTS "x_attendance_statisticshow";

DROP TABLE IF EXISTS "x_attendance_workday_config";

DROP TABLE IF EXISTS "x_attendance_workplace";

DROP TABLE IF EXISTS "x_bbs_shutup";

DROP TABLE IF EXISTS "x_cms_assemble_control_config";

DROP TABLE IF EXISTS "x_cms_assemble_control_section";

DROP TABLE IF EXISTS "x_cms_content";

DROP TABLE IF EXISTS "x_cms_control_config";

DROP TABLE IF EXISTS "x_cms_control_section";

DROP TABLE IF EXISTS "x_cms_templateform";

DROP TABLE IF EXISTS "x_component";

DROP TABLE IF EXISTS "x_correlation";

DROP TABLE IF EXISTS "x_file";

DROP TABLE IF EXISTS "x_file_assemble_control_category";

DROP TABLE IF EXISTS "x_file_assemble_control_config";

DROP TABLE IF EXISTS "x_file_assemble_control_storage_pool";

DROP TABLE IF EXISTS "x_general_assemble_area";

DROP TABLE IF EXISTS "x_general_assemble_control_config";

DROP TABLE IF EXISTS "x_general_assemble_control_permission";

DROP TABLE IF EXISTS "x_general_assemble_ecnet_config";

DROP TABLE IF EXISTS "x_general_assemble_excel";

DROP TABLE IF EXISTS "x_general_assemble_excel_sheet";

DROP TABLE IF EXISTS "x_general_assemble_invoice";

DROP TABLE IF EXISTS "x_general_assemble_office";

DROP TABLE IF EXISTS "x_general_assemble_qrcode";

DROP TABLE IF EXISTS "x_general_assemble_security_clearance";

DROP TABLE IF EXISTS "x_general_assemble_upgrade";

DROP TABLE IF EXISTS "x_general_assemble_worktime";

DROP TABLE IF EXISTS "x_general_attend_scope";

DROP TABLE IF EXISTS "x_jpush";

DROP TABLE IF EXISTS "x_meeting_assemble_control";

DROP TABLE IF EXISTS "x_meeting_checkin";

DROP TABLE IF EXISTS "x_meeting_config";

DROP TABLE IF EXISTS "x_message";

DROP TABLE IF EXISTS "x_message_collection";

DROP TABLE IF EXISTS "x_message_consume";

DROP TABLE IF EXISTS "x_message_conversation";

DROP TABLE IF EXISTS "x_message_conversation_member";

DROP TABLE IF EXISTS "x_message_file";

DROP TABLE IF EXISTS "x_message_instant";

DROP TABLE IF EXISTS "x_message_mass";

DROP TABLE IF EXISTS "x_mind";

DROP TABLE IF EXISTS "x_mind_assemble_control_config";

DROP TABLE IF EXISTS "x_org_config";

DROP TABLE IF EXISTS "x_portal_design";

DROP TABLE IF EXISTS "x_portal_dict";

DROP TABLE IF EXISTS "x_portal_file";

DROP TABLE IF EXISTS "x_portal_input";

DROP TABLE IF EXISTS "x_portal_layout";

DROP TABLE IF EXISTS "x_portal_output";

DROP TABLE IF EXISTS "x_portal_page_version";

DROP TABLE IF EXISTS "x_portal_script";

DROP TABLE IF EXISTS "x_portal_script_version";

DROP TABLE IF EXISTS "x_portal_surface";

DROP TABLE IF EXISTS "x_portal_template_page";

DROP TABLE IF EXISTS "x_portal_widget";

DROP TABLE IF EXISTS "x_process_surface";

DROP TABLE IF EXISTS "x_program_agent";

DROP TABLE IF EXISTS "x_program_callback_registration";

DROP TABLE IF EXISTS "x_program_collect";

DROP TABLE IF EXISTS "x_program_config";

DROP TABLE IF EXISTS "x_program_deploy";

DROP TABLE IF EXISTS "x_program_deploy_resource";

DROP TABLE IF EXISTS "x_program_design";

DROP TABLE IF EXISTS "x_program_dict";

DROP TABLE IF EXISTS "x_program_field";

DROP TABLE IF EXISTS "x_program_message_log";

DROP TABLE IF EXISTS "x_program_module";

DROP TABLE IF EXISTS "x_program_mpweixin_menu";

DROP TABLE IF EXISTS "x_program_output";

DROP TABLE IF EXISTS "x_program_prompt_error_log";

DROP TABLE IF EXISTS "x_program_schedule";

DROP TABLE IF EXISTS "x_program_schedule_log";

DROP TABLE IF EXISTS "x_program_script";

DROP TABLE IF EXISTS "x_program_sync_log";

DROP TABLE IF EXISTS "x_program_table";

DROP TABLE IF EXISTS "x_program_unexpected_error_log";

DROP TABLE IF EXISTS "x_query";

DROP TABLE IF EXISTS "x_query_design";

DROP TABLE IF EXISTS "x_query_entity_property";

DROP TABLE IF EXISTS "x_query_import_model";

DROP TABLE IF EXISTS "x_query_import_model_record";

DROP TABLE IF EXISTS "x_query_import_record";

DROP TABLE IF EXISTS "x_query_input";

DROP TABLE IF EXISTS "x_query_neural_calculate";

DROP TABLE IF EXISTS "x_query_neural_model";

DROP TABLE IF EXISTS "x_query_output";

DROP TABLE IF EXISTS "x_query_stat";

DROP TABLE IF EXISTS "x_query_surface";

DROP TABLE IF EXISTS "x_query_table";

DROP TABLE IF EXISTS "x_query_table_data";

DROP TABLE IF EXISTS "x_query_view_excel";

