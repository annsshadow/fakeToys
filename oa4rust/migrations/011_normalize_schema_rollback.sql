-- Rollback for migration 011: Restore uppercase table names
-- WARNING: This is a destructive operation. Run only if migration 011 failed.

BEGIN;

-- ── File tables ──────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'file_folder') THEN
    ALTER TABLE "file_folder" RENAME TO "FILE_FOLDER";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'file_file') THEN
    ALTER TABLE "file_file" RENAME TO "FILE_FILE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'file_permission') THEN
    ALTER TABLE "file_permission" RENAME TO "FILE_PERMISSION";
  END IF;
END $$;

-- ── AI tables ────────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_ai_completion') THEN
    ALTER TABLE "x_ai_completion" RENAME TO "X_AI_COMPLETION";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_ai_clue') THEN
    ALTER TABLE "x_ai_clue" RENAME TO "X_AI_CLUE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_ai_file') THEN
    ALTER TABLE "x_ai_file" RENAME TO "X_AI_FILE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_ai_model') THEN
    ALTER TABLE "x_ai_model" RENAME TO "X_AI_MODEL";
  END IF;
END $$;

-- ── CMS tables ───────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cms_article') THEN
    ALTER TABLE "x_cms_article" RENAME TO "X_CMS_ARTICLE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cms_category') THEN
    ALTER TABLE "x_cms_category" RENAME TO "X_CMS_CATEGORY";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cms_document') THEN
    ALTER TABLE "x_cms_document" RENAME TO "X_CMS_DOCUMENT";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cms_templateform') THEN
    ALTER TABLE "x_cms_templateform" RENAME TO "X_CMS_TEMPLATEFORM";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cms_view') THEN
    ALTER TABLE "x_cms_view" RENAME TO "X_CMS_VIEW";
  END IF;
END $$;

-- ── BBS tables ───────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_bbs') THEN
    ALTER TABLE "x_bbs" RENAME TO "X_BBS";
  END IF;
END $$;

-- ── Component tables ─────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cpt_component') THEN
    ALTER TABLE "x_cpt_component" RENAME TO "X_CPT_COMPONENT";
  END IF;
END $$;

-- ── Calendar tables ──────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cal_calendar') THEN
    ALTER TABLE "x_cal_calendar" RENAME TO "X_CAL_CALENDAR";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cal_event') THEN
    ALTER TABLE "x_cal_event" RENAME TO "X_CAL_EVENT";
  END IF;
END $$;

-- ── Message tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_msg_message') THEN
    ALTER TABLE "x_msg_message" RENAME TO "X_MSG_MESSAGE";
  END IF;
END $$;

-- ── General tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_gen_ara_district') THEN
    ALTER TABLE "x_gen_ara_district" RENAME TO "X_GEN_ARA_DISTRICT";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_gen_dict') THEN
    ALTER TABLE "x_gen_dict" RENAME TO "X_GEN_DICT";
  END IF;
END $$;

-- ── Express tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_express_company') THEN
    ALTER TABLE "x_express_company" RENAME TO "X_EXPRESS_COMPANY";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_express_info') THEN
    ALTER TABLE "x_express_info" RENAME TO "X_EXPRESS_INFO";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_express_subscribe') THEN
    ALTER TABLE "x_express_subscribe" RENAME TO "X_EXPRESS_SUBSCRIBE";
  END IF;
END $$;

-- ── Console tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_console_cache') THEN
    ALTER TABLE "x_console_cache" RENAME TO "X_CONSOLE_CACHE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_console_command_log') THEN
    ALTER TABLE "x_console_command_log" RENAME TO "X_CONSOLE_COMMAND_LOG";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_console_log') THEN
    ALTER TABLE "x_console_log" RENAME TO "X_CONSOLE_LOG";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_console_message') THEN
    ALTER TABLE "x_console_message" RENAME TO "X_CONSOLE_MESSAGE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_console_metric') THEN
    ALTER TABLE "x_console_metric" RENAME TO "X_CONSOLE_METRIC";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_console_status') THEN
    ALTER TABLE "x_console_status" RENAME TO "X_CONSOLE_STATUS";
  END IF;
END $$;

-- ── Correlation tables ───────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_corr_c_correlation') THEN
    ALTER TABLE "x_corr_c_correlation" RENAME TO "X_CORR_C_CORRELATION";
  END IF;
END $$;

-- ── Program center tables ────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cte_agent') THEN
    ALTER TABLE "x_cte_agent" RENAME TO "X_CTE_AGENT";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cte_invoke') THEN
    ALTER TABLE "x_cte_invoke" RENAME TO "X_CTE_INVOKE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_cte_structure') THEN
    ALTER TABLE "x_cte_structure" RENAME TO "X_CTE_STRUCTURE";
  END IF;
END $$;

-- ── Process platform tables ──────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_data_record') THEN
    ALTER TABLE "x_pp_c_data_record" RENAME TO "X_PP_C_DATA_RECORD";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_documentversion') THEN
    ALTER TABLE "x_pp_c_documentversion" RENAME TO "X_PP_C_DOCUMENTVERSION";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_doc_sign') THEN
    ALTER TABLE "x_pp_c_doc_sign" RENAME TO "X_PP_C_DOC_SIGN";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_draft') THEN
    ALTER TABLE "x_pp_c_draft" RENAME TO "X_PP_C_DRAFT";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_handover') THEN
    ALTER TABLE "x_pp_c_handover" RENAME TO "X_PP_C_HANDOVER";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_job') THEN
    ALTER TABLE "x_pp_c_job" RENAME TO "X_PP_C_JOB";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_keylock') THEN
    ALTER TABLE "x_pp_c_keylock" RENAME TO "X_PP_C_KEYLOCK";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_read') THEN
    ALTER TABLE "x_pp_c_read" RENAME TO "X_PP_C_READ";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_readcompleted') THEN
    ALTER TABLE "x_pp_c_readcompleted" RENAME TO "X_PP_C_READCOMPLETED";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_record') THEN
    ALTER TABLE "x_pp_c_record" RENAME TO "X_PP_C_RECORD";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_review') THEN
    ALTER TABLE "x_pp_c_review" RENAME TO "X_PP_C_REVIEW";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_serialnumber') THEN
    ALTER TABLE "x_pp_c_serialnumber" RENAME TO "X_PP_C_SERIALNUMBER";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_task') THEN
    ALTER TABLE "x_pp_c_task" RENAME TO "X_PP_C_TASK";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_taskcompleted') THEN
    ALTER TABLE "x_pp_c_taskcompleted" RENAME TO "X_PP_C_TASKCOMPLETED";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_task_process_mode') THEN
    ALTER TABLE "x_pp_c_task_process_mode" RENAME TO "X_PP_C_TASK_PROCESS_MODE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_work') THEN
    ALTER TABLE "x_pp_c_work" RENAME TO "X_PP_C_WORK";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_workcompleted') THEN
    ALTER TABLE "x_pp_c_workcompleted" RENAME TO "X_PP_C_WORKCOMPLETED";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_c_worklog') THEN
    ALTER TABLE "x_pp_c_worklog" RENAME TO "X_PP_C_WORKLOG";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_e_application') THEN
    ALTER TABLE "x_pp_e_application" RENAME TO "X_PP_E_APPLICATION";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_e_applicationdict') THEN
    ALTER TABLE "x_pp_e_applicationdict" RENAME TO "X_PP_E_APPLICATIONDICT";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_e_file') THEN
    ALTER TABLE "x_pp_e_file" RENAME TO "X_PP_E_FILE";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_e_form') THEN
    ALTER TABLE "x_pp_e_form" RENAME TO "X_PP_E_FORM";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_e_process') THEN
    ALTER TABLE "x_pp_e_process" RENAME TO "X_PP_E_PROCESS";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_pp_e_route') THEN
    ALTER TABLE "x_pp_e_route" RENAME TO "X_PP_E_ROUTE";
  END IF;
END $$;

-- ── Process tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_process_application') THEN
    ALTER TABLE "x_process_application" RENAME TO "X_PROCESS_APPLICATION";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_process_task') THEN
    ALTER TABLE "x_process_task" RENAME TO "X_PROCESS_TASK";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_process_ticket') THEN
    ALTER TABLE "x_process_ticket" RENAME TO "X_PROCESS_TICKET";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_process_work') THEN
    ALTER TABLE "x_process_work" RENAME TO "X_PROCESS_WORK";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_process_work_completed') THEN
    ALTER TABLE "x_process_work_completed" RENAME TO "X_PROCESS_WORK_COMPLETED";
  END IF;
END $$;

-- ── Query tables ─────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_query_import') THEN
    ALTER TABLE "x_query_import" RENAME TO "X_QUERY_IMPORT";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_query_item') THEN
    ALTER TABLE "x_query_item" RENAME TO "X_QUERY_ITEM";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_query_view') THEN
    ALTER TABLE "x_query_view" RENAME TO "X_QUERY_VIEW";
  END IF;
END $$;

COMMIT;
