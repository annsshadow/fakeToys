-- Rollback of 061_attendance_v2_tables.sql

DROP INDEX IF EXISTS "idx_x_attendance_v2_leave_batch_flag";
DROP TABLE IF EXISTS "x_attendance_v2_leave";
DROP TABLE IF EXISTS "x_attendance_v2_shift";
DROP TABLE IF EXISTS "x_attendance_v2_group";
