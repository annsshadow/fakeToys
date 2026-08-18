-- 038: add columns that list/query endpoints select or filter on but the
-- migrations never created. Root cause of the residual HTTP 500s captured from
-- the PostgreSQL server log during the parity probe: "column X does not exist".
-- These columns are referenced in lowercase by the Rust handlers (unquoted SQL
-- identifiers fold to lowercase), so they are created lowercase.
-- Uses ADD COLUMN IF NOT EXISTS for idempotency (PostgreSQL 9.6+).

-- bbs_subject_info: subject_list selects title.
ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "title" TEXT;

-- cal_calendar: calendar_list selects target.
ALTER TABLE "cal_calendar" ADD COLUMN IF NOT EXISTS "target" TEXT;

-- cal_event: event_list selects calendar_id.
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "calendar_id" TEXT;

-- corr_c_correlation: correlation queries reference from_type.
ALTER TABLE "corr_c_correlation" ADD COLUMN IF NOT EXISTS "from_type" TEXT;

-- x_ai_mcp_config: mcp config list selects name.
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "name" TEXT;

-- x_ai_model: model list selects create_time.
ALTER TABLE "x_ai_model" ADD COLUMN IF NOT EXISTS "create_time" TEXT;

-- x_attendance_detail: attendance detail list references file_id.
ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "file_id" TEXT;

-- x_correlation: correlation queries reference person_id.
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "person_id" TEXT;

-- x_file: file list selects name.
ALTER TABLE "x_file" ADD COLUMN IF NOT EXISTS "name" TEXT;

-- x_meeting: meeting list selects applied / status.
ALTER TABLE "x_meeting" ADD COLUMN IF NOT EXISTS "applied" TEXT;
ALTER TABLE "x_meeting" ADD COLUMN IF NOT EXISTS "status" TEXT;

-- x_message_consume: consume list references consumed.
ALTER TABLE "x_message_consume" ADD COLUMN IF NOT EXISTS "consumed" TEXT;

-- x_msg_message: message handler filters/persists xconsumed (boolean flag).
ALTER TABLE "x_msg_message" ADD COLUMN IF NOT EXISTS "xconsumed" BOOLEAN NOT NULL DEFAULT false;

-- x_org_role: role list references role_id.
ALTER TABLE "x_org_role" ADD COLUMN IF NOT EXISTS "role_id" TEXT;

-- x_portal_design: portal designer reads content.
ALTER TABLE "x_portal_design" ADD COLUMN IF NOT EXISTS "content" TEXT;

-- x_portal_surface: portal surface list selects category.
ALTER TABLE "x_portal_surface" ADD COLUMN IF NOT EXISTS "category" TEXT;

-- x_process_surface: process surface list selects name.
ALTER TABLE "x_process_surface" ADD COLUMN IF NOT EXISTS "name" TEXT;

-- x_query_design: query designer reads query_definition.
ALTER TABLE "x_query_design" ADD COLUMN IF NOT EXISTS "query_definition" TEXT;

-- x_query_surface: query surface list selects name.
ALTER TABLE "x_query_surface" ADD COLUMN IF NOT EXISTS "name" TEXT;
