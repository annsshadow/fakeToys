-- 007_admin_seed.sql
-- U6 安全加固：将 role-admin 角色绑定到 person-admin（初始化管理员），
-- 使授权检查中间件能够识别 admin 角色。unit-root 为 002 seed 中的根单位。
INSERT INTO auth_person_role (person_id, role_id, unit_id)
VALUES ('person-admin', 'role-admin', 'unit-root')
ON CONFLICT (person_id, role_id, unit_id) DO NOTHING;
