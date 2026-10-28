-- 113_add_x_attendance_config_id_index_rollback.sql
-- 回滚 113 的考勤配置 id 查找索引（仅用于手工回滚；不会被迁移运行器自动应用）。
DROP INDEX IF EXISTS idx_x_attendance_config_id;
