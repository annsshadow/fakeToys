-- 第五轮审计：processplatform 审批链最小造数（Rust scratch 库 oa_audit_20261105）
-- 用途：把 report-4 的「审批链前置状态是否可跳过 / status 字段能否直改」转成可判。
-- 可重复执行（先删后插）。归属：admin（creator=person-admin，即 auth.person.id）；攻击者 audit-user。
-- 注意：审计口径下 x_work.creator 存的是 person id（person-admin），而 session.person_unique 是 auth.unique_id（admin），
--       二者不一致；这本身是归属门在 admin 之外的正常用户身上会失效的关键观测点。

DELETE FROM x_work WHERE id IN ('work-admin-1','work-audit-1');
DELETE FROM x_task WHERE id IN ('task-admin-1','task-admin-2');

INSERT INTO x_work (id, title, process, application, work_status, creator, create_time)
VALUES
  ('work-admin-1', 'Admin work 1',  'p1', 'a1', 'pending', 'person-admin', '2026-11-06 00:00:00'),
  ('work-audit-1', 'Audit work 1',  'p1', 'a1', 'pending', 'audit-user',   '2026-11-06 00:00:00');

INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time)
VALUES
  ('task-admin-1', 'Admin task 1', 'work-admin-1', 'act-1', 'tok-1', 'person-admin', 'pending', '2026-11-06 00:00:00'),
  ('task-admin-2', 'Admin task 2', 'work-admin-1', 'act-1', 'tok-1', 'person-admin', 'pending', '2026-11-06 00:00:00');
