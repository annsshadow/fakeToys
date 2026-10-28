-- 113_add_x_attendance_config_id_index.sql
-- 110 为 x_attendance_config 补齐了 v2 配置族的读路径索引，但写路径仍按 id 定位：
--   SELECT id FROM x_attendance_config WHERE id = $1
--   UPDATE x_attendance_config SET ... WHERE id = $1
-- 该表没有主键/唯一约束（109 只补了另外三张考勤表），id 等值查询仍是全表扫描。
--
-- 这里只加**非唯一**索引：解决查找性能，不改数据完整性约束。
-- 是否升级为主键/唯一约束需要先确认线上 id 无重复、无 NULL，属于单独的数据治理决策；
-- 在完成该决策前，非唯一索引对重复行同样有效，且不会让迁移因历史数据失败。
CREATE INDEX IF NOT EXISTS idx_x_attendance_config_id
    ON x_attendance_config (id);
