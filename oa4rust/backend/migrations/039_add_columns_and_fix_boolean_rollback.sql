-- Rollback for 039.
--  (A) drop the TEXT columns added in 039.
--  (B) revert applied/consumed back to TEXT (their 038 state) and drop invited.

-- (B) revert boolean columns to TEXT
ALTER TABLE "x_meeting" ALTER COLUMN "applied" TYPE TEXT
  USING (CASE WHEN "applied" THEN 'true' ELSE 'false' END);
ALTER TABLE "x_meeting" ALTER COLUMN "applied" DROP NOT NULL;
ALTER TABLE "x_meeting" ALTER COLUMN "applied" DROP DEFAULT;

ALTER TABLE "x_message_consume" ALTER COLUMN "consumed" TYPE TEXT
  USING (CASE WHEN "consumed" THEN 'true' ELSE 'false' END);
ALTER TABLE "x_message_consume" ALTER COLUMN "consumed" DROP NOT NULL;
ALTER TABLE "x_message_consume" ALTER COLUMN "consumed" DROP DEFAULT;

ALTER TABLE "x_meeting" DROP COLUMN IF EXISTS "invited";

-- (A) drop the TEXT columns added in 039
ALTER TABLE "x_query_surface"   DROP COLUMN IF EXISTS "category";
ALTER TABLE "x_process_surface" DROP COLUMN IF EXISTS "category";
ALTER TABLE "x_portal_surface"  DROP COLUMN IF EXISTS "html";
ALTER TABLE "x_file"            DROP COLUMN IF EXISTS "path";
ALTER TABLE "x_ai_mcp_config"   DROP COLUMN IF EXISTS "url";
ALTER TABLE "x_ai_mcp_config"   DROP COLUMN IF EXISTS "default_model";
ALTER TABLE "cal_event"         DROP COLUMN IF EXISTS "title";
ALTER TABLE "cal_calendar"      DROP COLUMN IF EXISTS "color";
ALTER TABLE "bbs_subject_info"  DROP COLUMN IF EXISTS "author_id";
