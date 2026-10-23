-- 110_create_x_attendance_config_hot_path_indexes.sql
-- 运行时性能优化：考勤配置表 x_attendance_config 此前零二级索引（id 列也无
-- 主键/索引），v2 配置族的读写全部走全表顺序扫描。本轮按 103/108 方法学
-- 补齐索引——每条索引对应 handler 中至少一条真实 SQL 谓词组合。
--
-- attendance_assemble_control 热路径（x_attendance_config，分类存储 v2 配置）：
--   · 全局列表：   SELECT ... ORDER BY create_time DESC            (attendanceconfig_list, L432)
--   · v2 列表：    WHERE category = 'v2' ORDER BY name             (v2_config_get, L5923)
--   · v2 存在性：  WHERE category = 'v2' AND name = $1              (v2_config_post, L5957)
--   · 个人列表：   WHERE category = 'v2_person' AND creator = $1
--                    ORDER BY name                                (v2_config_person_get, L6007)
--   · 个人存在性： WHERE category = 'v2_person' AND name = $1
--                    AND creator = $2                             (v2_config_person_post, L6040)
--
-- (category, name)：category 等值前缀 + name 有序，一条覆盖 v2 家族的
-- 过滤、排序与存在性检查（name 等值时同样命中）。
-- (category, creator, name)：v2_person 家族多一个 creator 等值谓词，
-- creator 插在 name 前，使「creator 等值 + name 排序」免二次排序。
-- (create_time DESC)：全局列表的 ORDER BY create_time DESC 走索引反向扫描，
-- 避免全表排序（对齐 104 list_order 方法学；create_time 为 TEXT，
-- DESC 文本序与查询的 DESC 语义一致）。

-- v2 配置族：过滤 + name 排序 + 存在性检查
CREATE INDEX IF NOT EXISTS idx_x_attendance_config_category_name
    ON x_attendance_config (category, name);

-- v2_person 个人配置族：creator 等值 + name 排序
CREATE INDEX IF NOT EXISTS idx_x_attendance_config_category_creator_name
    ON x_attendance_config (category, creator, name);

-- 全局配置列表：ORDER BY create_time DESC
CREATE INDEX IF NOT EXISTS idx_x_attendance_config_create_time
    ON x_attendance_config (create_time DESC);
