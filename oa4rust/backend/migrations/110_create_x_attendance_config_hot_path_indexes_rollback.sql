-- 110_create_x_attendance_config_hot_path_indexes_rollback.sql
-- 回滚 110 的考勤配置热路径索引（仅用于手工回滚；不会被迁移运行器自动应用）。
DROP INDEX IF EXISTS idx_x_attendance_config_create_time;
DROP INDEX IF EXISTS idx_x_attendance_config_category_creator_name;
DROP INDEX IF EXISTS idx_x_attendance_config_category_name;
