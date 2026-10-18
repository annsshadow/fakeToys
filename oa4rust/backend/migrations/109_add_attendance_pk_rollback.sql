-- 109_add_attendance_pk_rollback.sql
-- 回滚 109 的考勤三表主键（仅用于手工回滚；不会被迁移运行器自动应用）。
-- 注意：回滚会移除主键约束并允许 id 为空，但不会还原被 109 去重 / 补 NULL 的数据。

ALTER TABLE x_attendance_appeal_info DROP CONSTRAINT IF EXISTS x_attendance_appeal_info_pkey;
ALTER TABLE x_attendance_appeal_info ALTER COLUMN id DROP NOT NULL;

ALTER TABLE x_attendance_detail DROP CONSTRAINT IF EXISTS x_attendance_detail_pkey;
ALTER TABLE x_attendance_detail ALTER COLUMN id DROP NOT NULL;

ALTER TABLE x_attendance_setting DROP CONSTRAINT IF EXISTS x_attendance_setting_pkey;
ALTER TABLE x_attendance_setting ALTER COLUMN id DROP NOT NULL;
