-- 109_add_attendance_pk.sql
-- 修复考勤三表缺少主键（四轮安全审计 Defect #3）：x_attendance_appeal_info、
-- x_attendance_detail、x_attendance_setting 的 id 列未设主键，既违反数据完整性，
-- 也削弱了 ownership / 幂等类查询的可靠性（如 import_checkin_rows 按 user_id 去重、
-- require_owner 按 owner_id 校验时，缺少唯一约束会让重复 / 孤儿行更易产生）。
--
-- 每张表独立处理：
--   1) 把 NULL id 补成随机 uuid（非破坏性，仅覆盖缺失行）；
--   2) 删去同 id 的重复行（保留 ctid 最小的一条，即最早写入的）；
--   3) SET NOT NULL 并加主键。
-- 若线上已存在非 NULL 且唯一的 id（正常情况），UPDATE / DELETE 均为空操作，迁移幂等安全。
-- 若确有历史重复 id，DELETE 仅保留一条，符合「id 应为唯一标识」的既有约定。

-- x_attendance_appeal_info
UPDATE x_attendance_appeal_info SET id = gen_random_uuid()::text WHERE id IS NULL;
DELETE FROM x_attendance_appeal_info a
USING x_attendance_appeal_info b
WHERE a.ctid < b.ctid AND a.id = b.id;
ALTER TABLE x_attendance_appeal_info ALTER COLUMN id SET NOT NULL;
ALTER TABLE x_attendance_appeal_info ADD PRIMARY KEY (id);

-- x_attendance_detail
UPDATE x_attendance_detail SET id = gen_random_uuid()::text WHERE id IS NULL;
DELETE FROM x_attendance_detail a
USING x_attendance_detail b
WHERE a.ctid < b.ctid AND a.id = b.id;
ALTER TABLE x_attendance_detail ALTER COLUMN id SET NOT NULL;
ALTER TABLE x_attendance_detail ADD PRIMARY KEY (id);

-- x_attendance_setting
UPDATE x_attendance_setting SET id = gen_random_uuid()::text WHERE id IS NULL;
DELETE FROM x_attendance_setting a
USING x_attendance_setting b
WHERE a.ctid < b.ctid AND a.id = b.id;
ALTER TABLE x_attendance_setting ALTER COLUMN id SET NOT NULL;
ALTER TABLE x_attendance_setting ADD PRIMARY KEY (id);
