-- 039: two classes of fixes derived from the parity probe PostgreSQL log.
--
--  (A) Genuinely-missing columns that list/query endpoints select or filter on
--      but the migrations never created (contaminated log diff minus the
--      columns already added by 038). All created as TEXT to match O2OA.
--
--  (B) text = boolean operator errors: 038 added x_meeting.applied and
--      x_message_consume.consumed as TEXT, but the handlers compare them to
--      boolean literals (applied = true, consumed = false). They must be
--      BOOLEAN. x_meeting.invited is also compared to true/false and was never
--      created, so it is added as BOOLEAN directly.
--
-- Uses ADD COLUMN IF NOT EXISTS / DROP COLUMN IF EXISTS and guarded type
-- changes for idempotency (PostgreSQL 9.6+). Runs inside the migrate
-- transaction.

-- (A) missing TEXT columns -----------------------------------------------
ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "author_id" TEXT;
ALTER TABLE "cal_calendar"      ADD COLUMN IF NOT EXISTS "color"     TEXT;
ALTER TABLE "cal_event"         ADD COLUMN IF NOT EXISTS "title"     TEXT;
ALTER TABLE "x_ai_mcp_config"   ADD COLUMN IF NOT EXISTS "default_model" TEXT;
ALTER TABLE "x_ai_mcp_config"   ADD COLUMN IF NOT EXISTS "url"       TEXT;
ALTER TABLE "x_file"            ADD COLUMN IF NOT EXISTS "path"      TEXT;
ALTER TABLE "x_portal_surface"  ADD COLUMN IF NOT EXISTS "html"      TEXT;
ALTER TABLE "x_process_surface" ADD COLUMN IF NOT EXISTS "category"  TEXT;
ALTER TABLE "x_query_surface"   ADD COLUMN IF NOT EXISTS "category"  TEXT;

-- (B) boolean columns -----------------------------------------------------
-- x_meeting.applied: currently TEXT -> BOOLEAN
ALTER TABLE "x_meeting" ALTER COLUMN "applied" TYPE BOOLEAN
  USING (CASE WHEN "applied" IS NOT NULL AND "applied" = 'true' THEN true ELSE false END);
ALTER TABLE "x_meeting" ALTER COLUMN "applied" SET DEFAULT false;
ALTER TABLE "x_meeting" ALTER COLUMN "applied" SET NOT NULL;

-- x_meeting.invited: never created -> BOOLEAN
ALTER TABLE "x_meeting" ADD COLUMN IF NOT EXISTS "invited" BOOLEAN NOT NULL DEFAULT false;

-- x_message_consume.consumed: currently TEXT -> BOOLEAN
ALTER TABLE "x_message_consume" ALTER COLUMN "consumed" TYPE BOOLEAN
  USING (CASE WHEN "consumed" IS NOT NULL AND "consumed" = 'true' THEN true ELSE false END);
ALTER TABLE "x_message_consume" ALTER COLUMN "consumed" SET DEFAULT false;
ALTER TABLE "x_message_consume" ALTER COLUMN "consumed" SET NOT NULL;
