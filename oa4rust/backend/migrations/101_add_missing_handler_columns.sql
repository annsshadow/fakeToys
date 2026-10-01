-- 101_add_missing_handler_columns.sql
-- 系统性核对（2026-09-27）：以「代码 SQL 引用的列」对撞「live PG information_schema」，
-- 找出全部 handler 运行时必然 500（undefined column）的缺失列，幂等补齐。
-- 类型按 handler 语义定：计数/排序用 BIGINT/INTEGER，布尔开关用 BOOLEAN DEFAULT false，
-- 其余引用字段用 TEXT；时间戳用 TIMESTAMP。
-- 每条 ALTER 前以 TO_REGCLASS 判表存在，避免个别环境缺表时整批失败。
-- Rollback file: 101_add_missing_handler_columns_rollback.sql

DO $$
BEGIN
-- ── CMS ──────────────────────────────────────────────────────────────────
IF TO_REGCLASS('public.x_cms_categoryinfo') IS NOT NULL THEN
    ALTER TABLE "x_cms_categoryinfo" ADD COLUMN IF NOT EXISTS "alias" TEXT;
END IF;
IF TO_REGCLASS('public.x_cms_document') IS NOT NULL THEN
    -- AI 索引端点（ai/src/index.rs）按应用过滤已发布文档
    ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "xappid" TEXT;
    ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "xdocstatus" TEXT;
END IF;

-- ── 组织同步配置（organization_core_express 读 config_key='sync_enabled'）──
IF TO_REGCLASS('public.x_org_config') IS NOT NULL THEN
    ALTER TABLE "x_org_config" ADD COLUMN IF NOT EXISTS "config_key" TEXT;
END IF;

-- ── 考勤（v2 周期列 + 申诉/请假流转列）────────────────────────────────────
IF TO_REGCLASS('public.x_attendance_detail') IS NOT NULL THEN
    ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "cycle_year" INTEGER;
    ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "cycle_month" INTEGER;
    ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "received" BOOLEAN DEFAULT false;
    ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "checked" BOOLEAN DEFAULT false;
    ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "archived" BOOLEAN DEFAULT false;
END IF;
IF TO_REGCLASS('public.x_attendance_appeal_info') IS NOT NULL THEN
    ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "workflow_status" TEXT;
    ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "archived" BOOLEAN DEFAULT false;
END IF;
IF TO_REGCLASS('public.x_attendance_selfholiday') IS NOT NULL THEN
    ALTER TABLE "x_attendance_selfholiday" ADD COLUMN IF NOT EXISTS "doc_id" TEXT;
END IF;

-- ── 消息/会话（IM 消费回执与置顶等会话操作）───────────────────────────────
IF TO_REGCLASS('public.x_message') IS NOT NULL THEN
    ALTER TABLE "x_message" ADD COLUMN IF NOT EXISTS "consumed" BOOLEAN DEFAULT false;
    ALTER TABLE "x_message" ADD COLUMN IF NOT EXISTS "cleared" BOOLEAN DEFAULT false;
END IF;
IF TO_REGCLASS('public.x_message_conversation') IS NOT NULL THEN
    ALTER TABLE "x_message_conversation" ADD COLUMN IF NOT EXISTS "title" TEXT;
    ALTER TABLE "x_message_conversation" ADD COLUMN IF NOT EXISTS "note" TEXT;
    ALTER TABLE "x_message_conversation" ADD COLUMN IF NOT EXISTS "top" BOOLEAN DEFAULT false;
    ALTER TABLE "x_message_conversation" ADD COLUMN IF NOT EXISTS "top_time" TIMESTAMP;
    ALTER TABLE "x_message_conversation" ADD COLUMN IF NOT EXISTS "business_id" TEXT;
END IF;

-- ── 组织（人员卡片/二维码、身份排序、职务排序、导入结果、同步配置）────────
IF TO_REGCLASS('public.x_org_person') IS NOT NULL THEN
    ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "card_code" TEXT;
    ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "qr_code" TEXT;
END IF;
IF TO_REGCLASS('public.x_org_identity') IS NOT NULL THEN
    ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "order_number" BIGINT DEFAULT 0;
END IF;
IF TO_REGCLASS('public.x_org_duty') IS NOT NULL THEN
    ALTER TABLE "x_org_duty" ADD COLUMN IF NOT EXISTS "sort" BIGINT DEFAULT 0;
END IF;
IF TO_REGCLASS('public.x_org_import_result') IS NOT NULL THEN
    ALTER TABLE "x_org_import_result" ADD COLUMN IF NOT EXISTS "import_id" TEXT;
END IF;

-- ── 门户 ─────────────────────────────────────────────────────────────────
IF TO_REGCLASS('public.x_portal') IS NOT NULL THEN
    ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "update_time" TIMESTAMP;
END IF;
IF TO_REGCLASS('public.x_portal_widget') IS NOT NULL THEN
    ALTER TABLE "x_portal_widget" ADD COLUMN IF NOT EXISTS "flag" TEXT;
END IF;

-- ── 控制台 / 程序中心 / 通用组件 ─────────────────────────────────────────
IF TO_REGCLASS('public.x_console_cache') IS NOT NULL THEN
    ALTER TABLE "x_console_cache" ADD COLUMN IF NOT EXISTS "xtype" TEXT;
END IF;
IF TO_REGCLASS('public.x_console_log') IS NOT NULL THEN
    ALTER TABLE "x_console_log" ADD COLUMN IF NOT EXISTS "xtype" TEXT;
END IF;
IF TO_REGCLASS('public.x_program_deploy_server') IS NOT NULL THEN
    ALTER TABLE "x_program_deploy_server" ADD COLUMN IF NOT EXISTS "server_type" TEXT;
END IF;
IF TO_REGCLASS('public.x_program_output') IS NOT NULL THEN
    ALTER TABLE "x_program_output" ADD COLUMN IF NOT EXISTS "app_info_flag" BOOLEAN DEFAULT false;
END IF;
IF TO_REGCLASS('public.x_program_sync_log') IS NOT NULL THEN
    ALTER TABLE "x_program_sync_log" ADD COLUMN IF NOT EXISTS "source" TEXT;
END IF;
IF TO_REGCLASS('public.x_general_assemble_excel') IS NOT NULL THEN
    ALTER TABLE "x_general_assemble_excel" ADD COLUMN IF NOT EXISTS "excel_name" TEXT;
END IF;
IF TO_REGCLASS('public.x_general_assemble_office') IS NOT NULL THEN
    ALTER TABLE "x_general_assemble_office" ADD COLUMN IF NOT EXISTS "word_flag" BOOLEAN DEFAULT false;
END IF;

-- ── 流程（PP_C/PP_E 各表 handler 按 jobId/workId 等过滤的引用列）──────────
IF TO_REGCLASS('public.pp_c_data_record') IS NOT NULL THEN
    ALTER TABLE "pp_c_data_record" ADD COLUMN IF NOT EXISTS "xwork" TEXT;
END IF;
IF TO_REGCLASS('public.pp_c_doc_sign') IS NOT NULL THEN
    ALTER TABLE "pp_c_doc_sign" ADD COLUMN IF NOT EXISTS "xjob" TEXT;
END IF;
IF TO_REGCLASS('public.pp_c_documentversion') IS NOT NULL THEN
    ALTER TABLE "pp_c_documentversion" ADD COLUMN IF NOT EXISTS "xjob" TEXT;
    ALTER TABLE "pp_c_documentversion" ADD COLUMN IF NOT EXISTS "xcategory" TEXT;
END IF;
IF TO_REGCLASS('public.pp_c_job') IS NOT NULL THEN
    ALTER TABLE "pp_c_job" ADD COLUMN IF NOT EXISTS "xjob" TEXT;
    ALTER TABLE "pp_c_job" ADD COLUMN IF NOT EXISTS "xsite" TEXT;
END IF;
IF TO_REGCLASS('public.pp_c_keylock') IS NOT NULL THEN
    ALTER TABLE "pp_c_keylock" ADD COLUMN IF NOT EXISTS "xid" TEXT;
    ALTER TABLE "pp_c_keylock" ADD COLUMN IF NOT EXISTS "xwork" TEXT;
END IF;
IF TO_REGCLASS('public.pp_c_record') IS NOT NULL THEN
    ALTER TABLE "pp_c_record" ADD COLUMN IF NOT EXISTS "xwork" TEXT;
END IF;
IF TO_REGCLASS('public.pp_c_taskcompleted') IS NOT NULL THEN
    ALTER TABLE "pp_c_taskcompleted" ADD COLUMN IF NOT EXISTS "xwork" TEXT;
END IF;
IF TO_REGCLASS('public.pp_e_output') IS NOT NULL THEN
    ALTER TABLE "pp_e_output" ADD COLUMN IF NOT EXISTS "xapplication" TEXT;
END IF;
IF TO_REGCLASS('public.pp_e_process_activity') IS NOT NULL THEN
    ALTER TABLE "pp_e_process_activity" ADD COLUMN IF NOT EXISTS "xactivitytype" TEXT;
END IF;
IF TO_REGCLASS('public.pp_e_process_element') IS NOT NULL THEN
    ALTER TABLE "pp_e_process_element" ADD COLUMN IF NOT EXISTS "xprocessid" TEXT;
END IF;
IF TO_REGCLASS('public.x_pp_c_task') IS NOT NULL THEN
    ALTER TABLE "x_pp_c_task" ADD COLUMN IF NOT EXISTS "xbamconfig" TEXT;
END IF;

-- ── 文件 / 认证 ──────────────────────────────────────────────────────────
IF TO_REGCLASS('public.file_file') IS NOT NULL THEN
    ALTER TABLE "file_file" ADD COLUMN IF NOT EXISTS "md5" TEXT;
END IF;
IF TO_REGCLASS('public.auth_person') IS NOT NULL THEN
    ALTER TABLE "auth_person" ADD COLUMN IF NOT EXISTS "settings" TEXT;
END IF;
END $$;
