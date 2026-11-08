-- 第六轮审计造数（自有 scratch 库 oa_audit_20261105）
-- 用途：空归属 fail-open helper（u4_require_actor）实测 + 两个普通用户双向交叉读
-- 可复跑：先 DELETE 后 INSERT，固定 id 幂等

-- ── A. 空 creator / 空 person 的流程行（fail-open 实测） ──
DELETE FROM x_timer_job WHERE id = 'timer-null-1';
DELETE FROM x_task WHERE id IN ('task-null-pending','task-null-active');
DELETE FROM x_work WHERE id IN ('work-null-pending','work-null-processing');

INSERT INTO x_work (id, title, process, application, work_status, creator, create_time)
VALUES ('work-null-pending','Null creator pending','p1','a1','pending',NULL,'2026-11-06 00:00:00');

INSERT INTO x_work (id, title, process, application, work_status, creator, create_time)
VALUES ('work-null-processing','Null creator processing','p1','a1','processing',NULL,'2026-11-06 00:00:00');

INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time)
VALUES ('task-null-pending','Null person pending','work-null-pending','act-1','tok-1',NULL,'pending','2026-11-06 00:00:00');

INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time)
VALUES ('task-null-active','Null person active','work-null-processing','act-1','tok-1',NULL,'active','2026-11-06 00:00:00');

INSERT INTO x_timer_job (id, work_id, task_id, fire_at, cron, created_at, kind)
VALUES ('timer-null-1','work-null-pending','task-null-pending','2026-11-06 01:00:00',NULL,NOW(),'delay');

-- ── B. 两个普通用户的考勤明细（双向交叉读） ──
DELETE FROM x_attendance_detail WHERE id IN ('r6-ad-a','r6-ad-b');

INSERT INTO x_attendance_detail (id, person_id, date, status, received, create_time)
VALUES ('r6-ad-a','audit-u1','2026-11-06','NORMAL',false,'2026-11-06 00:00:00'),
       ('r6-ad-b','audit-u2','2026-11-06','LATE',false,'2026-11-06 00:00:00');

-- ── C. 第二普通用户（仅 scratch 库；凭据克隆自 audit-user） ──
INSERT INTO auth_person (id, unique_id, name, password_hash, salt, locked, failed_attempts, created_at, updated_at)
SELECT 'audit-u2','audit-user2','Audit User 2',password_hash,salt,locked,0,NOW(),NOW()
FROM auth_person WHERE unique_id = 'audit-user'
ON CONFLICT (id) DO NOTHING;
