-- 041: remaining missing columns surfaced by the clean parity-probe log after
-- 040. These complete the column sets for the failing raw-SQL handlers (and
-- the is_base/is_extended flags used by x_ai_mcp_config WHERE clauses).
-- Types match handler bind/read:
--   * reply_count / view_count -> INTEGER   (read as i32)
--   * is_top / disable / is_base / is_extended -> BOOLEAN (compared to true/false)
--   * max_tokens               -> BIGINT    (read/bound as i64)
--   * folder_id / source / location / "type" -> TEXT
-- ADD COLUMN IF NOT EXISTS for idempotency; runs inside the migrate transaction.

-- bbs_subject_info
ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "reply_count" INTEGER NOT NULL DEFAULT 0;
ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "view_count"  INTEGER NOT NULL DEFAULT 0;
ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "is_top"       BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "disable"      BOOLEAN NOT NULL DEFAULT false;

-- x_ai_mcp_config
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "is_base"     BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "is_extended" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "max_tokens"  BIGINT  NOT NULL DEFAULT 0;

-- x_correlation: "type" is a reserved word, must stay quoted
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "type" TEXT;

-- x_file
ALTER TABLE "x_file" ADD COLUMN IF NOT EXISTS "folder_id" TEXT;

-- raw calendar tables (distinct from the SeaORM x_cal_calendar / x_cal_event)
ALTER TABLE "cal_calendar" ADD COLUMN IF NOT EXISTS "source" TEXT;
ALTER TABLE "cal_event"    ADD COLUMN IF NOT EXISTS "location" TEXT;
