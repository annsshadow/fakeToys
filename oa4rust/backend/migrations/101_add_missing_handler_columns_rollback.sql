-- 101_add_missing_handler_columns_rollback.sql
DO $$
BEGIN
IF TO_REGCLASS('public.x_cms_document') IS NOT NULL THEN
    ALTER TABLE "x_cms_document" DROP COLUMN IF EXISTS "xappid";
    ALTER TABLE "x_cms_document" DROP COLUMN IF EXISTS "xdocstatus";
END IF;
IF TO_REGCLASS('public.x_org_config') IS NOT NULL THEN
    ALTER TABLE "x_org_config" DROP COLUMN IF EXISTS "config_key";
END IF;
IF TO_REGCLASS('public.x_cms_categoryinfo') IS NOT NULL THEN
    ALTER TABLE "x_cms_categoryinfo" DROP COLUMN IF EXISTS "alias";
END IF;
IF TO_REGCLASS('public.x_attendance_detail') IS NOT NULL THEN
    ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "cycle_year";
    ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "cycle_month";
    ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "received";
    ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "checked";
    ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "archived";
END IF;
IF TO_REGCLASS('public.x_attendance_appeal_info') IS NOT NULL THEN
    ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "workflow_status";
    ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "archived";
END IF;
IF TO_REGCLASS('public.x_attendance_selfholiday') IS NOT NULL THEN
    ALTER TABLE "x_attendance_selfholiday" DROP COLUMN IF EXISTS "doc_id";
END IF;
IF TO_REGCLASS('public.x_message') IS NOT NULL THEN
    ALTER TABLE "x_message" DROP COLUMN IF EXISTS "consumed";
    ALTER TABLE "x_message" DROP COLUMN IF EXISTS "cleared";
END IF;
IF TO_REGCLASS('public.x_message_conversation') IS NOT NULL THEN
    ALTER TABLE "x_message_conversation" DROP COLUMN IF EXISTS "title";
    ALTER TABLE "x_message_conversation" DROP COLUMN IF EXISTS "note";
    ALTER TABLE "x_message_conversation" DROP COLUMN IF EXISTS "top";
    ALTER TABLE "x_message_conversation" DROP COLUMN IF EXISTS "top_time";
    ALTER TABLE "x_message_conversation" DROP COLUMN IF EXISTS "business_id";
END IF;
IF TO_REGCLASS('public.x_org_person') IS NOT NULL THEN
    ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "card_code";
    ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "qr_code";
END IF;
IF TO_REGCLASS('public.x_org_identity') IS NOT NULL THEN
    ALTER TABLE "x_org_identity" DROP COLUMN IF EXISTS "order_number";
END IF;
IF TO_REGCLASS('public.x_org_duty') IS NOT NULL THEN
    ALTER TABLE "x_org_duty" DROP COLUMN IF EXISTS "sort";
END IF;
IF TO_REGCLASS('public.x_org_import_result') IS NOT NULL THEN
    ALTER TABLE "x_org_import_result" DROP COLUMN IF EXISTS "import_id";
END IF;
IF TO_REGCLASS('public.x_portal') IS NOT NULL THEN
    ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "update_time";
END IF;
IF TO_REGCLASS('public.x_portal_widget') IS NOT NULL THEN
    ALTER TABLE "x_portal_widget" DROP COLUMN IF EXISTS "flag";
END IF;
IF TO_REGCLASS('public.x_console_cache') IS NOT NULL THEN
    ALTER TABLE "x_console_cache" DROP COLUMN IF EXISTS "xtype";
END IF;
IF TO_REGCLASS('public.x_console_log') IS NOT NULL THEN
    ALTER TABLE "x_console_log" DROP COLUMN IF EXISTS "xtype";
END IF;
IF TO_REGCLASS('public.x_program_deploy_server') IS NOT NULL THEN
    ALTER TABLE "x_program_deploy_server" DROP COLUMN IF EXISTS "server_type";
END IF;
IF TO_REGCLASS('public.x_program_output') IS NOT NULL THEN
    ALTER TABLE "x_program_output" DROP COLUMN IF EXISTS "app_info_flag";
END IF;
IF TO_REGCLASS('public.x_program_sync_log') IS NOT NULL THEN
    ALTER TABLE "x_program_sync_log" DROP COLUMN IF EXISTS "source";
END IF;
IF TO_REGCLASS('public.x_general_assemble_excel') IS NOT NULL THEN
    ALTER TABLE "x_general_assemble_excel" DROP COLUMN IF EXISTS "excel_name";
END IF;
IF TO_REGCLASS('public.x_general_assemble_office') IS NOT NULL THEN
    ALTER TABLE "x_general_assemble_office" DROP COLUMN IF EXISTS "word_flag";
END IF;
IF TO_REGCLASS('public.pp_c_data_record') IS NOT NULL THEN
    ALTER TABLE "pp_c_data_record" DROP COLUMN IF EXISTS "xwork";
END IF;
IF TO_REGCLASS('public.pp_c_doc_sign') IS NOT NULL THEN
    ALTER TABLE "pp_c_doc_sign" DROP COLUMN IF EXISTS "xjob";
END IF;
IF TO_REGCLASS('public.pp_c_documentversion') IS NOT NULL THEN
    ALTER TABLE "pp_c_documentversion" DROP COLUMN IF EXISTS "xjob";
    ALTER TABLE "pp_c_documentversion" DROP COLUMN IF EXISTS "xcategory";
END IF;
IF TO_REGCLASS('public.pp_c_job') IS NOT NULL THEN
    ALTER TABLE "pp_c_job" DROP COLUMN IF EXISTS "xjob";
    ALTER TABLE "pp_c_job" DROP COLUMN IF EXISTS "xsite";
END IF;
IF TO_REGCLASS('public.pp_c_keylock') IS NOT NULL THEN
    ALTER TABLE "pp_c_keylock" DROP COLUMN IF EXISTS "xid";
    ALTER TABLE "pp_c_keylock" DROP COLUMN IF EXISTS "xwork";
END IF;
IF TO_REGCLASS('public.pp_c_record') IS NOT NULL THEN
    ALTER TABLE "pp_c_record" DROP COLUMN IF EXISTS "xwork";
END IF;
IF TO_REGCLASS('public.pp_c_taskcompleted') IS NOT NULL THEN
    ALTER TABLE "pp_c_taskcompleted" DROP COLUMN IF EXISTS "xwork";
END IF;
IF TO_REGCLASS('public.pp_e_output') IS NOT NULL THEN
    ALTER TABLE "pp_e_output" DROP COLUMN IF EXISTS "xapplication";
END IF;
IF TO_REGCLASS('public.pp_e_process_activity') IS NOT NULL THEN
    ALTER TABLE "pp_e_process_activity" DROP COLUMN IF EXISTS "xactivitytype";
END IF;
IF TO_REGCLASS('public.pp_e_process_element') IS NOT NULL THEN
    ALTER TABLE "pp_e_process_element" DROP COLUMN IF EXISTS "xprocessid";
END IF;
IF TO_REGCLASS('public.x_pp_c_task') IS NOT NULL THEN
    ALTER TABLE "x_pp_c_task" DROP COLUMN IF EXISTS "xbamconfig";
END IF;
IF TO_REGCLASS('public.file_file') IS NOT NULL THEN
    ALTER TABLE "file_file" DROP COLUMN IF EXISTS "md5";
END IF;
IF TO_REGCLASS('public.auth_person') IS NOT NULL THEN
    ALTER TABLE "auth_person" DROP COLUMN IF EXISTS "settings";
END IF;
END $$;
