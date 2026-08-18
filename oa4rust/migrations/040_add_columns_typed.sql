-- 040: missing columns surfaced by the clean parity-probe PostgreSQL log after
-- 039. Types are chosen to match how the Rust handlers bind/read them:
--   * x_file.size            -> BIGINT   (bound/reads as i64)
--   * x_ai_mcp_config.enabled-> BOOLEAN  (bound as bool, read as bool)
--   * x_ai_mcp_config.temperature -> DOUBLE PRECISION (bound as f64, read as f64)
--   * everything else        -> TEXT     (read as String)
-- ADD COLUMN IF NOT EXISTS for idempotency; runs inside the migrate transaction.

ALTER TABLE "bbs_subject_info"  ADD COLUMN IF NOT EXISTS "section_id" TEXT;
ALTER TABLE "cal_calendar"      ADD COLUMN IF NOT EXISTS "description" TEXT;
ALTER TABLE "cal_event"         ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_ai_mcp_config"   ADD COLUMN IF NOT EXISTS "enabled" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_ai_mcp_config"   ADD COLUMN IF NOT EXISTS "temperature" DOUBLE PRECISION;
ALTER TABLE "x_correlation"     ADD COLUMN IF NOT EXISTS "target_id" TEXT;
ALTER TABLE "x_file"            ADD COLUMN IF NOT EXISTS "size" BIGINT;
ALTER TABLE "x_portal_surface"  ADD COLUMN IF NOT EXISTS "template" TEXT;
ALTER TABLE "x_process_surface" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_process_surface" ADD COLUMN IF NOT EXISTS "version" TEXT;
ALTER TABLE "x_query_surface"   ADD COLUMN IF NOT EXISTS "content" TEXT;
