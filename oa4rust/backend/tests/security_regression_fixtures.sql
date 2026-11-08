-- 安全回归门禁夹具（第八轮；第九轮改受害者身份）
-- 依赖：security_regression_seed.sql 已先执行（受害行 owner=gate-victim）
--
-- 受害者用 gate-victim 而不是 person-admin：迁移 007 把 person-admin 绑定了
-- role-admin，若拿它做「B 操作 A」的反向身份，B 的 2xx 是管理员合法放行，
-- 会把反向守卫变成假阳性。

-- 身份：攻击者与受害者（两者都不是管理员）
INSERT INTO auth_person (id, unique_id, name, password_hash, locked, unit, mobile, email) VALUES
  ('audit-user',   'audit-user',   'Gate Attacker', 'gate-noop', false, 'unit-gate-a', 'rev-GATE-V-SETTING', 'gate-a@gate'),
  ('gate-victim',  'gate-victim',  'GATE-V-NAME',   'gate-noop', false, 'unit-gate-v', 'GATE-V-SETTING',     'GATE-V-EMAIL'),
  -- 假门/软门夹具仍用 person-admin（那里验证的是「管理员本人」语义）
  ('person-admin', 'person-admin', 'Gate Admin',    'gate-noop', false, 'unit-gate-v', NULL, NULL)
ON CONFLICT (id) DO NOTHING;

UPDATE auth_person SET mobile = 'rev-GATE-V-SETTING', email = 'gate-a@gate' WHERE id = 'audit-user';
UPDATE auth_person SET name = 'GATE-V-NAME', mobile = 'GATE-V-SETTING', email = 'GATE-V-EMAIL' WHERE id = 'gate-victim';

-- 假门族：cms 文档（draft，非发布状态，所以非所有者不应 readable=true）
DELETE FROM x_cms_data_document WHERE id = 'gate-doc-1';
INSERT INTO x_cms_data_document
  (id, app_id, title, content, author_id, status, creator, create_time, deleted_at)
VALUES
  ('gate-doc-1', 'gate-app', 'GATE draft doc', 'GATE content',
   'person-admin', 'draft', 'person-admin', NOW(), NULL);

-- fail-open 族：x_work 的 creator 为空（helper 与内联副本都会 fail-open）
DELETE FROM x_work WHERE id = 'gate-work-null';
INSERT INTO x_work
  (id, title, process, application, work_status, creator, create_time, status, person)
VALUES
  ('gate-work-null', 'GATE null creator', 'gate-process', 'gate-app',
   'pending', NULL, NOW(), 'pending', NULL);

-- 软门族：empower 授权行（仅用于 manager_update 的「不得被改」复核）
DELETE FROM x_empower WHERE id = 'gate-emp-1';
INSERT INTO x_empower
  (id, from_person, to_person, role_id, enabled, created_at, updated_at)
VALUES
  ('gate-emp-1', 'person-admin', 'person-admin', 'role-gate', true, NOW(), NOW());

-- ── 第十一轮：深分页夹具 ────────────────────────────────────────────
-- 全部用 pg-* 前缀；三表在 seed 里没有既有行，不会与其它用例互扰。
--
-- detail：攻击者 6 行（日期并列 2+2、1 行 NULL、1 行普通）；
--         pg-a-00 是「NULL 排序键里 id 最小」的 prev 游标（受害者所有，仅借用键）；
--         受害者 3 行带 GATE-V-PAGE，用于「无归属过滤时必须变红」。
DELETE FROM x_attendance_detail WHERE id LIKE 'pg-a-%' OR id LIKE 'pg-v-%';
INSERT INTO x_attendance_detail (id, person_id, date, status) VALUES
  ('pg-a-00', 'gate-victim', NULL,        'GATE-V-PAGE'),
  ('pg-a-01', 'audit-user', '2026-02-01', 'GATE-A-PAGE'),
  ('pg-a-02', 'audit-user', '2026-02-01', 'GATE-A-PAGE'),
  ('pg-a-03', 'audit-user', '2026-02-02', 'GATE-A-PAGE'),
  ('pg-a-04', 'audit-user', '2026-02-02', 'GATE-A-PAGE'),
  ('pg-a-05', 'audit-user', NULL,         'GATE-A-PAGE'),
  ('pg-a-06', 'audit-user', '2026-02-03', 'GATE-A-PAGE'),
  ('pg-v-01', 'gate-victim', '2026-02-01', 'GATE-V-PAGE'),
  ('pg-v-02', 'gate-victim', '2026-02-02', 'GATE-V-PAGE'),
  ('pg-v-03', 'gate-victim', '2026-02-03', 'GATE-V-PAGE');

-- selfholiday：6 行，holiday_date 三行并列 + 两行 NULL。
DELETE FROM x_attendance_selfholiday WHERE id LIKE 'pg-s-%';
INSERT INTO x_attendance_selfholiday (id, person_id, holiday_date, reason) VALUES
  ('pg-s-01', 'audit-user',  '2026-05-01', 'GATE-PAGE-S'),
  ('pg-s-02', 'gate-victim', '2026-05-01', 'GATE-PAGE-S'),
  ('pg-s-03', 'audit-user',  '2026-05-01', 'GATE-PAGE-S'),
  ('pg-s-04', 'audit-user',  '2026-04-01', 'GATE-PAGE-S'),
  ('pg-s-05', 'audit-user',  NULL,         'GATE-PAGE-S'),
  ('pg-s-06', 'gate-victim', NULL,         'GATE-PAGE-S');

-- statisticshow personMonth：month 用 "1"/"01"/"1" 验证数字语义并列，
-- 另加 2026/2、2025/12、NULL 行。
DELETE FROM x_attendance_statisticshow WHERE id LIKE 'pg-m-%';
INSERT INTO x_attendance_statisticshow (id, person_id, year, month, status) VALUES
  ('pg-m-01', 'audit-user',  '2026', '1',  'GATE-PAGE-M'),
  ('pg-m-02', 'audit-user',  '2026', '01', 'GATE-PAGE-M'),
  ('pg-m-03', 'audit-user',  '2026', '1',  'GATE-PAGE-M'),
  ('pg-m-04', 'audit-user',  '2026', '2',  'GATE-PAGE-M'),
  ('pg-m-05', 'gate-victim', '2025', '12', 'GATE-PAGE-M'),
  ('pg-m-06', 'audit-user',  NULL,   NULL, 'GATE-PAGE-M');

-- statisticshow topUnitDay：6 行 unit_id=NULL（work_date 三行并列 + NULL），
-- 另造 2 行 unit_id 非空，必须被端点的 unit_id IS NULL 过滤掉。
DELETE FROM x_attendance_statisticshow WHERE id LIKE 'pg-d-%' OR id LIKE 'pg-du-%';
INSERT INTO x_attendance_statisticshow (id, person_id, unit_id, work_date, status) VALUES
  ('pg-d-01',  'audit-user', NULL,     '2026-06-01', 'GATE-PAGE-D'),
  ('pg-d-02',  'audit-user', NULL,     '2026-06-01', 'GATE-PAGE-D'),
  ('pg-d-03',  'audit-user', NULL,     '2026-06-01', 'GATE-PAGE-D'),
  ('pg-d-04',  'audit-user', NULL,     '2026-06-02', 'GATE-PAGE-D'),
  ('pg-d-05',  'audit-user', NULL,     NULL,         'GATE-PAGE-D'),
  ('pg-d-06',  'audit-user', NULL,     '2026-06-03', 'GATE-PAGE-D'),
  ('pg-du-01', 'audit-user', 'unit-x', '2026-06-01', 'GATE-PAGE-DU'),
  ('pg-du-02', 'audit-user', 'unit-x', '2026-06-02', 'GATE-PAGE-DU');
DELETE FROM x_org_person WHERE id IN ('gate-person-v', 'gate-person-a', 'gate-person-del');
INSERT INTO x_org_person (id, name, mobile, email, unit_id, creator, create_time, deleted_at) VALUES
  ('gate-person-v', 'GATE-PERSON-V', 'GATE-V-MOBILE',     'v@gate', 'unit-gate-v', 'gate-victim', NOW(), NULL),
  ('gate-person-a', 'GATE-PERSON-A', 'rev-GATE-V-MOBILE', 'a@gate', 'unit-gate-a', 'audit-user',  NOW(), NULL),
  -- 已软删人员：仅供 person/list/delete 分页用例验证「仅管理员可读」。
  ('gate-person-del', 'GATE-PERSON-DEL', 'GATE-DEL-MOBILE', 'del@gate', 'unit-gate-v', 'gate-victim', NOW(), NOW());
DELETE FROM x_org_group_member WHERE group_id = 'grp-gate-b';
DELETE FROM x_org_role WHERE id IN ('role-gate-v', 'role-gate-a');
INSERT INTO x_org_role (id, name) VALUES
  ('role-gate-v', 'GATE-V-ROLE'),
  ('role-gate-a', 'rev-GATE-V-ROLE');
INSERT INTO x_org_group_member (group_id, person_id, role_id) VALUES
  ('grp-gate-b', 'gate-victim', 'role-gate-v'),
  ('grp-gate-b', 'audit-user',  'role-gate-a'),
  ('grp-gate-b', 'gate-person-v', 'role-gate-v'),
  ('grp-gate-b', 'gate-person-a', 'role-gate-a');
DELETE FROM x_org_group WHERE id = 'grp-gate-b';
INSERT INTO x_org_group (id, name, unit_id, type, creator, create_time, deleted_at) VALUES
  ('grp-gate-b', 'GATE-GROUP', 'unit-gate-b', 'group', 'audit-user', NOW(), NULL);
-- B8 公开目录查询：unit_check 需要「人员-组织」身份行与目标组织都存在才可观测。
DELETE FROM x_org_identity WHERE person_id = 'gate-person-v';
DELETE FROM x_org_unit WHERE id = 'unit-gate-v';
INSERT INTO x_org_unit (id, name, parent_id, level, sort, creator, create_time, deleted_at) VALUES
  ('unit-gate-v', 'GATE-UNIT-V', NULL, 0, 0, 'gate-victim', NOW(), NULL);
INSERT INTO x_org_identity (id, name, unit_id, person_id, creator, create_time, deleted_at) VALUES
  ('ident-gate-v', 'GATE-IDENT-V', 'unit-gate-v', 'gate-person-v', 'gate-victim', NOW(), NULL);
DELETE FROM auth_person_role WHERE role_id IN ('role-gate-v', 'role-gate-a');
INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES
  ('gate-person-v', 'role-gate-v', 'unit-gate-b'),
  ('gate-person-a', 'role-gate-a', 'unit-gate-b');
DELETE FROM x_message WHERE id IN ('gate-msg-v', 'gate-msg-a');
INSERT INTO x_message (id, conversation_id, content, sender, type, create_time) VALUES
  ('gate-msg-v', 'conv-gate', 'GATE-V-MSG',     'gate-victim', 'text', '2026-11-06 10:00:00'),
  ('gate-msg-a', 'conv-gate', 'rev-GATE-V-MSG', 'audit-user',  'text', '2026-11-06 09:00:00');

