-- 092 rollback: drop the columns added by 092_attendance_endpoint_status_columns.sql.

ALTER TABLE "x_attendance_setting" DROP COLUMN IF EXISTS "enabled";

ALTER TABLE "x_attendance_statistic" DROP COLUMN IF EXISTS "person_id";
ALTER TABLE "x_attendance_statistic" DROP COLUMN IF EXISTS "year";
ALTER TABLE "x_attendance_statistic" DROP COLUMN IF EXISTS "month";

ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "audit_status";
ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "checked";
ALTER TABLE "x_attendance_appeal_info" DROP COLUMN IF EXISTS "workflow_synced";

ALTER TABLE "x_attendance_detail" DROP COLUMN IF EXISTS "analysed";
