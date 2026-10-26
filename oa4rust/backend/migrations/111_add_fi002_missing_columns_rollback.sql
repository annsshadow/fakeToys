-- 111_add_fi002_missing_columns_rollback.sql
-- 回滚 111：只删本迁移新建的列。注意这些列本就被 handler 引用，
-- 删掉后对应 handler 会回到「undefined column」500 —— 只在确认要退回旧状态时执行。

DO $$
BEGIN
IF TO_REGCLASS('public.auth_person_identity') IS NOT NULL THEN
    ALTER TABLE "auth_person_identity" DROP COLUMN IF EXISTS "person_id";
END IF;
IF TO_REGCLASS('public.pp_c_draft') IS NOT NULL THEN
    ALTER TABLE "pp_c_draft" DROP COLUMN IF EXISTS "xstatus";
END IF;
IF TO_REGCLASS('public.pp_c_keylock') IS NOT NULL THEN
    ALTER TABLE "pp_c_keylock" DROP COLUMN IF EXISTS "xidentity";
    ALTER TABLE "pp_c_keylock" DROP COLUMN IF EXISTS "xtitle";
END IF;
IF TO_REGCLASS('public.pp_e_file') IS NOT NULL THEN
    ALTER TABLE "pp_e_file" DROP COLUMN IF EXISTS "xcontent";
END IF;
IF TO_REGCLASS('public.pp_e_script') IS NOT NULL THEN
    ALTER TABLE "pp_e_script" DROP COLUMN IF EXISTS "xcode";
END IF;
IF TO_REGCLASS('public.x_attendance_appeal_info') IS NOT NULL THEN
    ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "appeal_date";
    ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "reason";
END IF;
IF TO_REGCLASS('public.x_attendance_statistic_dd_unit_day') IS NOT NULL THEN
    ALTER TABLE "x_attendance_statistic_dd_unit_day" DROP COLUMN IF EXISTS "o2_user";
END IF;
IF TO_REGCLASS('public.x_attendance_statistic_dd_unit_month') IS NOT NULL THEN
    ALTER TABLE "x_attendance_statistic_dd_unit_month" DROP COLUMN IF EXISTS "o2_user";
END IF;
IF TO_REGCLASS('public.x_bbs_section') IS NOT NULL THEN
    ALTER TABLE "x_bbs_section" DROP COLUMN IF EXISTS "creator";
END IF;
IF TO_REGCLASS('public.x_bbs_subject_attachment') IS NOT NULL THEN
    ALTER TABLE "x_bbs_subject_attachment" DROP COLUMN IF EXISTS "extension";
END IF;
IF TO_REGCLASS('public.x_cms_index') IS NOT NULL THEN
    ALTER TABLE "x_cms_index" DROP COLUMN IF EXISTS "status";
END IF;
IF TO_REGCLASS('public.x_cms_viewfieldconfig') IS NOT NULL THEN
    ALTER TABLE "x_cms_viewfieldconfig" DROP COLUMN IF EXISTS "creator";
END IF;
IF TO_REGCLASS('public.x_console_message') IS NOT NULL THEN
    ALTER TABLE "x_console_message" DROP COLUMN IF EXISTS "xtoken";
END IF;
IF TO_REGCLASS('public.x_express_subscribe') IS NOT NULL THEN
    ALTER TABLE "x_express_subscribe" DROP COLUMN IF EXISTS "xcallback";
    ALTER TABLE "x_express_subscribe" DROP COLUMN IF EXISTS "xcompany";
    ALTER TABLE "x_express_subscribe" DROP COLUMN IF EXISTS "xtrackingNumber";
END IF;
IF TO_REGCLASS('public.x_general_assemble_excel') IS NOT NULL THEN
    ALTER TABLE "x_general_assemble_excel" DROP COLUMN IF EXISTS "flag";
    ALTER TABLE "x_general_assemble_excel" DROP COLUMN IF EXISTS "name";
END IF;
IF TO_REGCLASS('public.x_general_assemble_office') IS NOT NULL THEN
    ALTER TABLE "x_general_assemble_office" DROP COLUMN IF EXISTS "html_content";
END IF;
IF TO_REGCLASS('public.x_meeting_checkin') IS NOT NULL THEN
    ALTER TABLE "x_meeting_checkin" DROP COLUMN IF EXISTS "meeting_id";
    ALTER TABLE "x_meeting_checkin" DROP COLUMN IF EXISTS "person";
    ALTER TABLE "x_meeting_checkin" DROP COLUMN IF EXISTS "checkin_time";
END IF;
IF TO_REGCLASS('public.x_portal') IS NOT NULL THEN
    ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "config";
    ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "pages";
END IF;
IF TO_REGCLASS('public.x_portal_layout') IS NOT NULL THEN
    ALTER TABLE "x_portal_layout" DROP COLUMN IF EXISTS "category";
    ALTER TABLE "x_portal_layout" DROP COLUMN IF EXISTS "content";
    ALTER TABLE "x_portal_layout" DROP COLUMN IF EXISTS "name";
END IF;
IF TO_REGCLASS('public.x_portal_page_version') IS NOT NULL THEN
    ALTER TABLE "x_portal_page_version" DROP COLUMN IF EXISTS "content";
END IF;
IF TO_REGCLASS('public.x_process_organization') IS NOT NULL THEN
    ALTER TABLE "x_process_organization" DROP COLUMN IF EXISTS "name";
END IF;
IF TO_REGCLASS('public.x_program_callback_registration') IS NOT NULL THEN
    ALTER TABLE "x_program_callback_registration" DROP COLUMN IF EXISTS "source";
END IF;
IF TO_REGCLASS('public.x_program_deploy') IS NOT NULL THEN
    ALTER TABLE "x_program_deploy" DROP COLUMN IF EXISTS "status";
END IF;
IF TO_REGCLASS('public.x_program_message_log') IS NOT NULL THEN
    ALTER TABLE "x_program_message_log" DROP COLUMN IF EXISTS "message_type";
    ALTER TABLE "x_program_message_log" DROP COLUMN IF EXISTS "user_id";
END IF;
IF TO_REGCLASS('public.x_program_module') IS NOT NULL THEN
    ALTER TABLE "x_program_module" DROP COLUMN IF EXISTS "vip";
END IF;
IF TO_REGCLASS('public.x_program_output') IS NOT NULL THEN
    ALTER TABLE "x_program_output" DROP COLUMN IF EXISTS "select_file";
END IF;
IF TO_REGCLASS('public.x_program_script') IS NOT NULL THEN
    ALTER TABLE "x_program_script" DROP COLUMN IF EXISTS "imported_content";
END IF;
IF TO_REGCLASS('public.x_program_sync_log') IS NOT NULL THEN
    ALTER TABLE "x_program_sync_log" DROP COLUMN IF EXISTS "action";
    ALTER TABLE "x_program_sync_log" DROP COLUMN IF EXISTS "event_type";
    ALTER TABLE "x_program_sync_log" DROP COLUMN IF EXISTS "org_id";
END IF;
END $$;
