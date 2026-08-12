-- Migration 011: Normalize schema (UPPERCASE table names → snake_case)
-- This migration renames all uppercase table names to lowercase snake_case.
-- It is idempotent: safe to run multiple times.
--
-- WARNING: This is a breaking change. All application code referencing these
-- tables must be updated to use the new lowercase names.
--
-- Rollback: See 011_normalize_schema_rollback.sql

BEGIN;

-- ── File tables ──────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_FOLDER') THEN
    ALTER TABLE "FILE_FOLDER" RENAME TO "file_folder";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_FILE') THEN
    ALTER TABLE "FILE_FILE" RENAME TO "file_file";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'FILE_PERMISSION') THEN
    ALTER TABLE "FILE_PERMISSION" RENAME TO "file_permission";
  END IF;
END $$;

-- ── AI tables ────────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_AI_COMPLETION') THEN
    ALTER TABLE "X_AI_COMPLETION" RENAME TO "x_ai_completion";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_AI_CLUE') THEN
    ALTER TABLE "X_AI_CLUE" RENAME TO "x_ai_clue";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_AI_FILE') THEN
    ALTER TABLE "X_AI_FILE" RENAME TO "x_ai_file";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_AI_MODEL') THEN
    ALTER TABLE "X_AI_MODEL" RENAME TO "x_ai_model";
  END IF;
END $$;

-- ── CMS tables ───────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CMS_ARTICLE') THEN
    ALTER TABLE "X_CMS_ARTICLE" RENAME TO "x_cms_article";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CMS_CATEGORY') THEN
    ALTER TABLE "X_CMS_CATEGORY" RENAME TO "x_cms_category";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CMS_DOCUMENT') THEN
    ALTER TABLE "X_CMS_DOCUMENT" RENAME TO "x_cms_document";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CMS_TEMPLATEFORM') THEN
    ALTER TABLE "X_CMS_TEMPLATEFORM" RENAME TO "x_cms_templateform";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CMS_VIEW') THEN
    ALTER TABLE "X_CMS_VIEW" RENAME TO "x_cms_view";
  END IF;
END $$;

-- ── BBS tables ───────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_BBS') THEN
    ALTER TABLE "X_BBS" RENAME TO "x_bbs";
  END IF;
END $$;

-- ── Component tables ─────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CPT_COMPONENT') THEN
    ALTER TABLE "X_CPT_COMPONENT" RENAME TO "x_cpt_component";
  END IF;
END $$;

-- ── Calendar tables ──────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CAL_CALENDAR') THEN
    ALTER TABLE "X_CAL_CALENDAR" RENAME TO "x_cal_calendar";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CAL_EVENT') THEN
    ALTER TABLE "X_CAL_EVENT" RENAME TO "x_cal_event";
  END IF;
END $$;

-- ── Message tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_MSG_MESSAGE') THEN
    ALTER TABLE "X_MSG_MESSAGE" RENAME TO "x_msg_message";
  END IF;
END $$;

-- ── General tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_GEN_ARA_DISTRICT') THEN
    ALTER TABLE "X_GEN_ARA_DISTRICT" RENAME TO "x_gen_ara_district";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_GEN_DICT') THEN
    ALTER TABLE "X_GEN_DICT" RENAME TO "x_gen_dict";
  END IF;
END $$;

-- ── Express tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_EXPRESS_COMPANY') THEN
    ALTER TABLE "X_EXPRESS_COMPANY" RENAME TO "x_express_company";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_EXPRESS_INFO') THEN
    ALTER TABLE "X_EXPRESS_INFO" RENAME TO "x_express_info";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_EXPRESS_SUBSCRIBE') THEN
    ALTER TABLE "X_EXPRESS_SUBSCRIBE" RENAME TO "x_express_subscribe";
  END IF;
END $$;

-- ── Console tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CONSOLE_CACHE') THEN
    ALTER TABLE "X_CONSOLE_CACHE" RENAME TO "x_console_cache";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CONSOLE_COMMAND_LOG') THEN
    ALTER TABLE "X_CONSOLE_COMMAND_LOG" RENAME TO "x_console_command_log";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CONSOLE_LOG') THEN
    ALTER TABLE "X_CONSOLE_LOG" RENAME TO "x_console_log";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CONSOLE_MESSAGE') THEN
    ALTER TABLE "X_CONSOLE_MESSAGE" RENAME TO "x_console_message";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CONSOLE_METRIC') THEN
    ALTER TABLE "X_CONSOLE_METRIC" RENAME TO "x_console_metric";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CONSOLE_STATUS') THEN
    ALTER TABLE "X_CONSOLE_STATUS" RENAME TO "x_console_status";
  END IF;
END $$;

-- ── Correlation tables ───────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CORR_C_CORRELATION') THEN
    ALTER TABLE "X_CORR_C_CORRELATION" RENAME TO "x_corr_c_correlation";
  END IF;
END $$;

-- ── Program center tables ────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CTE_AGENT') THEN
    ALTER TABLE "X_CTE_AGENT" RENAME TO "x_cte_agent";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CTE_INVOKE') THEN
    ALTER TABLE "X_CTE_INVOKE" RENAME TO "x_cte_invoke";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_CTE_STRUCTURE') THEN
    ALTER TABLE "X_CTE_STRUCTURE" RENAME TO "x_cte_structure";
  END IF;
END $$;

-- ── Process platform tables ──────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_DATA_RECORD') THEN
    ALTER TABLE "X_PP_C_DATA_RECORD" RENAME TO "x_pp_c_data_record";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_DOCUMENTVERSION') THEN
    ALTER TABLE "X_PP_C_DOCUMENTVERSION" RENAME TO "x_pp_c_documentversion";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_DOC_SIGN') THEN
    ALTER TABLE "X_PP_C_DOC_SIGN" RENAME TO "x_pp_c_doc_sign";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_DRAFT') THEN
    ALTER TABLE "X_PP_C_DRAFT" RENAME TO "x_pp_c_draft";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_HANDOVER') THEN
    ALTER TABLE "X_PP_C_HANDOVER" RENAME TO "x_pp_c_handover";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_JOB') THEN
    ALTER TABLE "X_PP_C_JOB" RENAME TO "x_pp_c_job";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_KEYLOCK') THEN
    ALTER TABLE "X_PP_C_KEYLOCK" RENAME TO "x_pp_c_keylock";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_READ') THEN
    ALTER TABLE "X_PP_C_READ" RENAME TO "x_pp_c_read";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_READCOMPLETED') THEN
    ALTER TABLE "X_PP_C_READCOMPLETED" RENAME TO "x_pp_c_readcompleted";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_RECORD') THEN
    ALTER TABLE "X_PP_C_RECORD" RENAME TO "x_pp_c_record";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_REVIEW') THEN
    ALTER TABLE "X_PP_C_REVIEW" RENAME TO "x_pp_c_review";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_SERIALNUMBER') THEN
    ALTER TABLE "X_PP_C_SERIALNUMBER" RENAME TO "x_pp_c_serialnumber";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_TASK') THEN
    ALTER TABLE "X_PP_C_TASK" RENAME TO "x_pp_c_task";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_TASKCOMPLETED') THEN
    ALTER TABLE "X_PP_C_TASKCOMPLETED" RENAME TO "x_pp_c_taskcompleted";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_TASK_PROCESS_MODE') THEN
    ALTER TABLE "X_PP_C_TASK_PROCESS_MODE" RENAME TO "x_pp_c_task_process_mode";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_WORK') THEN
    ALTER TABLE "X_PP_C_WORK" RENAME TO "x_pp_c_work";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_WORKCOMPLETED') THEN
    ALTER TABLE "X_PP_C_WORKCOMPLETED" RENAME TO "x_pp_c_workcompleted";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_C_WORKLOG') THEN
    ALTER TABLE "X_PP_C_WORKLOG" RENAME TO "x_pp_c_worklog";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_E_APPLICATION') THEN
    ALTER TABLE "X_PP_E_APPLICATION" RENAME TO "x_pp_e_application";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_E_APPLICATIONDICT') THEN
    ALTER TABLE "X_PP_E_APPLICATIONDICT" RENAME TO "x_pp_e_applicationdict";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_E_FILE') THEN
    ALTER TABLE "X_PP_E_FILE" RENAME TO "x_pp_e_file";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_E_FORM') THEN
    ALTER TABLE "X_PP_E_FORM" RENAME TO "x_pp_e_form";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_E_PROCESS') THEN
    ALTER TABLE "X_PP_E_PROCESS" RENAME TO "x_pp_e_process";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PP_E_ROUTE') THEN
    ALTER TABLE "X_PP_E_ROUTE" RENAME TO "x_pp_e_route";
  END IF;
END $$;

-- ── Process tables ───────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PROCESS_APPLICATION') THEN
    ALTER TABLE "X_PROCESS_APPLICATION" RENAME TO "x_process_application";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PROCESS_TASK') THEN
    ALTER TABLE "X_PROCESS_TASK" RENAME TO "x_process_task";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PROCESS_TICKET') THEN
    ALTER TABLE "X_PROCESS_TICKET" RENAME TO "x_process_ticket";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PROCESS_WORK') THEN
    ALTER TABLE "X_PROCESS_WORK" RENAME TO "x_process_work";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_PROCESS_WORK_COMPLETED') THEN
    ALTER TABLE "X_PROCESS_WORK_COMPLETED" RENAME TO "x_process_work_completed";
  END IF;
END $$;

-- ── Query tables ─────────────────────────────────────────────────────────────
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_QUERY_IMPORT') THEN
    ALTER TABLE "X_QUERY_IMPORT" RENAME TO "x_query_import";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_QUERY_ITEM') THEN
    ALTER TABLE "X_QUERY_ITEM" RENAME TO "x_query_item";
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'X_QUERY_VIEW') THEN
    ALTER TABLE "X_QUERY_VIEW" RENAME TO "x_query_view";
  END IF;
END $$;

-- ── Update indexes ───────────────────────────────────────────────────────────
-- Indexes are already lowercase; skip no-op renames.
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'IDX_FILE_FOLDER_SUPERIOR') THEN
    ALTER INDEX "IDX_FILE_FOLDER_SUPERIOR" RENAME TO "idx_file_folder_superior";
  END IF;
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'IDX_FILE_FILE_PERSON') THEN
    ALTER INDEX "IDX_FILE_FILE_PERSON" RENAME TO "idx_file_file_person";
  END IF;
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'IDX_FILE_FILE_REFERENCE') THEN
    ALTER INDEX "IDX_FILE_FILE_REFERENCE" RENAME TO "idx_file_file_reference";
  END IF;
END $$;

COMMIT;
