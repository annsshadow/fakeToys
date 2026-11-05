-- 117_add_x_attendance_record_created_by_rollback.sql
ALTER TABLE "x_attendance_record"
    DROP COLUMN IF EXISTS "created_by";
