-- Rollback for 040: drop the columns added in 040.

ALTER TABLE "x_query_surface"   DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_process_surface" DROP COLUMN IF EXISTS "version";
ALTER TABLE "x_process_surface" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_portal_surface"  DROP COLUMN IF EXISTS "template";
ALTER TABLE "x_file"            DROP COLUMN IF EXISTS "size";
ALTER TABLE "x_correlation"     DROP COLUMN IF EXISTS "target_id";
ALTER TABLE "x_ai_mcp_config"   DROP COLUMN IF EXISTS "temperature";
ALTER TABLE "x_ai_mcp_config"   DROP COLUMN IF EXISTS "enabled";
ALTER TABLE "cal_event"         DROP COLUMN IF EXISTS "content";
ALTER TABLE "cal_calendar"      DROP COLUMN IF EXISTS "description";
ALTER TABLE "bbs_subject_info"  DROP COLUMN IF EXISTS "section_id";
