-- 108_create_x_attendance_detail_indexes.sql
-- 运行时性能优化：考勤明细 x_attendance_detail 此前零二级索引，多条最热查询
-- 只能全表顺序扫描。本轮补齐 person+date 主导谓词族索引（103 方法学：每条索引
-- 对应 handler 中至少一条真实 SQL 谓词组合）。
--
-- attendance_assemble_control 热路径（按 person_id 过滤 + date 等值/前缀/范围/排序）：
--   · 统计面：   WHERE person_id = ? AND date LIKE ?   GROUP BY status   (v2_my_statistic)
--   · 列表面：   WHERE person_id = ? ORDER BY date [DESC] [LIMIT]       (考勤明细/游标列表)
--   · 单日面：   WHERE person_id = ? AND date = ? [LIMIT 1]            (当日打卡/缺勤)
--   · 区间面：   WHERE person_id = ? AND date >= ? AND date <= ?       (月度打卡区间)
-- 复合 (person_id, date) 一条即覆盖上述全部：person_id 走等值前缀，date 复用同一
-- 索引做等值/范围/排序，免二次排序与全表扫。
CREATE INDEX IF NOT EXISTS idx_x_attendance_detail_person_date
    ON x_attendance_detail (person_id, date);
