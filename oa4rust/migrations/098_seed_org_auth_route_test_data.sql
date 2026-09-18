-- Seed data for organization_assemble_authentication route-presence tests.
--
-- 背景：crates/organization_assemble_authentication/src/tests.rs 的两个用例
--   test_person_id_icon_route_exists   → GET /api/organization/assemble/authentication/person/test-person-id/icon
--   test_identity_id_route_exists      → GET /api/organization/assemble/authentication/identity/test-identity-id
-- 以 assert_ne!(status, 404) 断言"路由已注册"。但两个 handler 的写法是
--   .query_one(...).map_err(|_| AppError::NotFound)
-- 即"行不存在"同样返回 404，与"路由未注册"无法区分。因此在行缺失的环境
-- （如 CI 的干净迁移历史）这两个用例必然失败。
--
-- 这里按本仓库既有惯例（参照 053_seed_hotpic_test_data.sql）补上所需行，
-- 使断言回到它本意要验证的"路由存在"语义上。
--
-- 注意：两个 handler 均以**非 Option** 方式读取若干可空列：
--   person_id_icon → let icon_url: String = row.get("icon_url");
--   identity_id    → let unit_id: String  = row.get("unit_id");
-- 因此这些列必须提供非 NULL 值，否则会以
--   "error deserializing column N" 的形式 panic。
-- （这也暴露了 handler 侧的真实缺陷：可空列应按 Option<String> 读取；
--   本次仅按最小改动补齐 seed，不去改业务代码。）

INSERT INTO auth_person (id, unique_id, name, password_hash, icon_url, locked, deleted_at)
VALUES
    ('test-person-id', 'test-person-id', 'Test Person', '{bcrypt}$2b$12$dummy.hash.for.testing',
     'http://example.com/icon.png', FALSE, NULL)
ON CONFLICT (id) DO UPDATE SET icon_url = EXCLUDED.icon_url;

INSERT INTO x_org_identity (id, name, unit_id, person_id, type, major, deleted_at)
VALUES
    ('test-identity-id', 'Test Identity', 'test-unit-id', 'test-person-id', 'person', FALSE, NULL)
ON CONFLICT (id) DO UPDATE SET unit_id = EXCLUDED.unit_id;
