-- 安全回归门禁夹具（第八轮）
-- 依赖：security_regression_seed.sql 已先执行（82 张受害表的 r6v-<table> 行，owner=person-admin）

-- 身份：攻击者（非所有者、非管理员）与受害者
INSERT INTO auth_person (id, unique_id, name, password_hash, locked, unit) VALUES
  ('audit-user',   'audit-user',   'Gate Attacker', 'gate-noop', false, 'unit-gate-a'),
  ('person-admin', 'person-admin', 'Gate Victim',   'gate-noop', false, 'unit-gate-v')
ON CONFLICT (id) DO NOTHING;

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
