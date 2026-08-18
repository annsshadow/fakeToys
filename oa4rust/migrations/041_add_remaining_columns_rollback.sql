-- Rollback for 041.

ALTER TABLE "cal_event"    DROP COLUMN IF EXISTS "location";
ALTER TABLE "cal_calendar" DROP COLUMN IF EXISTS "source";
ALTER TABLE "x_file"       DROP COLUMN IF EXISTS "folder_id";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "type";
ALTER TABLE "x_ai_mcp_config" DROP COLUMN IF EXISTS "max_tokens";
ALTER TABLE "x_ai_mcp_config" DROP COLUMN IF EXISTS "is_extended";
ALTER TABLE "x_ai_mcp_config" DROP COLUMN IF EXISTS "is_base";
ALTER TABLE "bbs_subject_info" DROP COLUMN IF EXISTS "disable";
ALTER TABLE "bbs_subject_info" DROP COLUMN IF EXISTS "is_top";
ALTER TABLE "bbs_subject_info" DROP COLUMN IF EXISTS "view_count";
ALTER TABLE "bbs_subject_info" DROP COLUMN IF EXISTS "reply_count";
