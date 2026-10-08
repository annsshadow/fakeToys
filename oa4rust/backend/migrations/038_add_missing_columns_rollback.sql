-- Rollback for 038: drop the columns added in 038.

ALTER TABLE "bbs_subject_info" DROP COLUMN IF EXISTS "title";
ALTER TABLE "cal_calendar" DROP COLUMN IF EXISTS "target";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "calendar_id";
ALTER TABLE "corr_c_correlation" DROP COLUMN IF EXISTS "from_type";
ALTER TABLE "x_ai_mcp_config" DROP COLUMN IF EXISTS "name";
ALTER TABLE "x_ai_model" DROP COLUMN IF EXISTS "create_time";
ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "file_id";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "person_id";
ALTER TABLE "x_file" DROP COLUMN IF EXISTS "name";
ALTER TABLE "x_meeting" DROP COLUMN IF EXISTS "applied";
ALTER TABLE "x_meeting" DROP COLUMN IF EXISTS "status";
ALTER TABLE "x_message_consume" DROP COLUMN IF EXISTS "consumed";
ALTER TABLE "x_msg_message" DROP COLUMN IF EXISTS "xconsumed";
ALTER TABLE "x_org_role" DROP COLUMN IF EXISTS "role_id";
ALTER TABLE "x_portal_design" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_portal_surface" DROP COLUMN IF EXISTS "category";
ALTER TABLE "x_process_surface" DROP COLUMN IF EXISTS "name";
ALTER TABLE "x_query_design" DROP COLUMN IF EXISTS "query_definition";
ALTER TABLE "x_query_surface" DROP COLUMN IF EXISTS "name";
