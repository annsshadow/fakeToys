-- 092: add status columns referenced by attendance_assemble_control endpoints
-- (attendancesetting/enable/type, statistic/do, attendanceappealInfo/audit|check|workflow/sync,
--  attendancedetail/analyse/redo). IF NOT EXISTS keeps this a no-op when the column
-- already exists (e.g. DB provisioned from the Java o2server schema).

ALTER TABLE "x_attendance_setting" ADD COLUMN IF NOT EXISTS "enabled" BOOLEAN;

ALTER TABLE "x_attendance_statistic" ADD COLUMN IF NOT EXISTS "person_id" TEXT;
ALTER TABLE "x_attendance_statistic" ADD COLUMN IF NOT EXISTS "year" TEXT;
ALTER TABLE "x_attendance_statistic" ADD COLUMN IF NOT EXISTS "month" TEXT;

ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "audit_status" TEXT;
ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "checked" BOOLEAN;
ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "workflow_synced" BOOLEAN;

ALTER TABLE "x_attendance_detail" ADD COLUMN IF NOT EXISTS "analysed" BOOLEAN;
