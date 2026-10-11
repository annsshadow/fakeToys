-- 108_create_x_attendance_detail_indexes_rollback.sql
-- 回滚 108 的考勤明细 person+date 热路径索引。
DROP INDEX IF EXISTS idx_x_attendance_detail_person_date;
