-- ============================================================
-- oa4rust 行为对比 · 组织人员域共享种子（Rust 侧）
-- 目标：使 comparator 全量跑时的字面模板 URL（如
--   /jaxrs/organization/assemble/control/unit/list/%7Bflag%7D/sub/direct）
-- 在两侧命中同一资源，消除“业务状态不对称”类假差异。
-- 标识符字面值来源：tests/behavior_compare.rs 全量驱动不做任何
-- 占位符替换，ENDPOINTS 中 {flag}/{xxx} 原样进入 URL（reqwest
-- percent-encode 传输、服务端 decode 回 '{flag}' 等字符串）。
-- 幂等策略：ON CONFLICT 原地更新；重放安全。
-- 执行：docker exec -i bc-postgres psql -U o2server -d oa4rust < seed_org.sql
-- ============================================================

-- ── 单元（org_unit：整表无任何约束 → 用 INSERT..WHERE NOT EXISTS + UPDATE 实现幂等；
--     id=name 双保险覆盖按 id 与按 name 两种查找）──
INSERT INTO org_unit (id, name, superior, level, cnt, sequence, order_number,
                      creator, creator_person, deleted_at)
SELECT * FROM (VALUES
    ('{flag}'::text,    '{flag}'::text,    NULL::text, '1'::text, '0'::text, '0'::text, 0::bigint, 'behavior-seed'::text, 'behavior-seed'::text, NULL::timestamp),
    ('{unitFlag}',      '{unitFlag}',      NULL,       '1',       '0',       '0',       0,         'behavior-seed',       'behavior-seed',       NULL)
) AS v(id,name,superior,level,cnt,sequence,order_number,creator,creator_person,deleted_at)
WHERE NOT EXISTS (SELECT 1 FROM org_unit t WHERE t.id IN ('{flag}','{unitFlag}'));
UPDATE org_unit SET deleted_at = NULL, creator = COALESCE(creator,'behavior-seed')
WHERE id IN ('{flag}','{unitFlag}');

-- ── 人员（auth_person：PK id + UNIQUE unique_id，恒取 id=unique_id=字面值；
--     password_hash 为不可登录占位（bcrypt 格式），行为对比只读不登录）──
INSERT INTO auth_person (id, unique_id, name, mobile, password_hash, locked, failed_attempts, deleted_at)
VALUES ('{personFlag}','{personFlag}','{personFlag}','13900000001',
        '$2b$12$seedseedseedseedseedseedseedseedseedseedseedseedse', false, 0, NULL),
       ('{flag}',      '{flag}',      '{flag}',      '13900000002',
        '$2b$12$seedseedseedseedseedseedseedseedseedseedseedseedse', false, 0, NULL)
ON CONFLICT (unique_id) DO UPDATE
SET name = EXCLUDED.name, mobile = EXCLUDED.mobile, deleted_at = NULL;

-- ── 群组（auth_group：PK id；id=name 双保险）──
INSERT INTO auth_group (id, name, description, disable, deleted_at)
VALUES ('{flag}',     '{flag}',     'behavior-compare seed', false, NULL),
       ('{groupFlag}','{groupFlag}','behavior-compare seed', false, NULL)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, deleted_at = NULL;

-- ── 角色（auth_role：PK id；id=name 双保险）──
INSERT INTO auth_role (id, name, description, disable, deleted_at)
VALUES ('{roleFlag}','{roleFlag}','behavior-compare seed', false, NULL)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, deleted_at = NULL;

-- ── 身份（auth_identity：PK id；挂到 person {personFlag} 下，
--     支撑 unitduty/list/identity/{identityFlag} 类查询）──
INSERT INTO auth_identity (id, name, identity_type, description, deleted_at)
VALUES ('{identityFlag}','{identityFlag}','主职','behavior-compare seed', NULL)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, deleted_at = NULL;

INSERT INTO auth_person_identity (person_id, identity_id, deleted_at)
VALUES ('{personFlag}', '{identityFlag}', NULL)
ON CONFLICT (person_id, identity_id) DO UPDATE SET deleted_at = NULL;

-- ── 身份影子行（x_org_identity：Rust control 域 identity/list/* 查询此表；
--     unit_id/creator/person_id 直接字面匹配 → 与 Java 侧 identity/list/unit|person
--     返回的 array[1] 对称；Java 侧该 identity 挂 person {personFlag} + unit {unitFlag}）──
INSERT INTO x_org_identity (id, name, unit_id, identity_id, creator, person_id, type, major, deleted_at)
VALUES ('{identityFlag}','{identityFlag}','{unitFlag}','{identityFlag}','{personFlag}','{personFlag}','主职', false, NULL)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, unit_id = EXCLUDED.unit_id,
    creator = EXCLUDED.creator, person_id = EXCLUDED.person_id, deleted_at = NULL;

-- ── 人员影子行（x_org_person：Rust express 域 person/auth/info、person/mobile、
--     person/nick/name 按 (id OR name) 匹配此表；缺失时 Rust 返回 data:Null，
--     与 Java Object 信封产生 type differs → 播种后两侧均为对象）──
INSERT INTO x_org_person (id, name, mobile, status, deleted_at)
VALUES ('{flag}','{flag}','13900000002','0',NULL)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, mobile = EXCLUDED.mobile, deleted_at = NULL;

-- ── 职务（x_org_duty：可选，支撑 unitduty/list/unit/{unitFlag}
--     返回非空场景；保持空列表与 Java 侧一致，故默认不插。
--     如需非空对称样本，取消注释并同步在 Java 侧建同名 duty）──
-- INSERT INTO x_org_duty (id, name, unit_id, identity_id, creator, deleted_at)
-- VALUES ('duty-seed-001','{unitFlag}主管','{unitFlag}','{identityFlag}','behavior-seed',NULL)
-- ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, deleted_at = NULL;

-- ── 单元属性（x_org_unit_attribute：可选，支撑 unitattribute/list/unit/{flag}；
--     默认两侧均为空列表即可趋同，故不插。）

-- ── 验证 ──
SELECT 'org_unit' AS tbl, count(*) FROM org_unit WHERE id IN ('{flag}','{unitFlag}')
UNION ALL SELECT 'auth_person', count(*) FROM auth_person WHERE unique_id IN ('{flag}','{personFlag}')
UNION ALL SELECT 'auth_group', count(*) FROM auth_group WHERE id IN ('{flag}','{groupFlag}')
UNION ALL SELECT 'auth_role', count(*) FROM auth_role WHERE id IN ('{roleFlag}')
UNION ALL SELECT 'auth_identity', count(*) FROM auth_identity WHERE id IN ('{identityFlag}')
UNION ALL SELECT 'x_org_person', count(*) FROM x_org_person WHERE id IN ('{flag}')
UNION ALL SELECT 'x_org_identity', count(*) FROM x_org_identity WHERE id IN ('{identityFlag}');

-- ======== content domain (CMS/BBS/Meeting) ========

-- behavior_compare shared seed fixtures - content domain (CMS/BBS/Meeting)
-- Identifier = literal path-template token sent by comparator ("{id}" etc),
-- so both sides resolve the same resource for the same literal URL.
-- Idempotent. Java-side companion recipe: seed_fixtures_java.http.md

-- CMS ------------------------------------------------------------------
INSERT INTO x_cms_appinfo (id, app_type, alias, enabled, creator)
VALUES ('{id}', 'INFO', '{id}', true, 'xadmin-seed-0001')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_cms_categoryinfo (id, app_id, name, status, sort_order, creator)
VALUES ('{id}', '{id}', '{id}', 'enabled', 0, 'xadmin-seed-0001')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_cms_document (id, xid, title, content, creator_person, deleted_at)
VALUES ('{id}', '{id}', '{id}', 'seed', 'xadmin-seed-0001', NULL)
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_cms_fileinfo (id, doc_id, original_name, content_type, size, upload_person)
VALUES ('{id}', '{id}', '{id}.txt', 'text/plain', 4, 'xadmin-seed-0001')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

-- BBS --------------------------------------------------------------------
INSERT INTO x_bbs_forum (id, name, creator)
VALUES ('{forumId}', '{forumId}', 'xadmin-seed-0001')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_bbs_section (id, forum_id, name)
VALUES ('{sectionId}', '{forumId}', '{sectionId}')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_bbs_topic (id, forum_id, section_id, section_name, title, content,
                         author_id, subject_type, creator)
VALUES ('{id}', '{forumId}', '{sectionId}', '{sectionId}', '{id}', 'seed',
        'xadmin-seed-0001', 'DISCUSS', 'xadmin-seed-0001')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

-- Meeting ------------------------------------------------------------------
INSERT INTO x_meeting_building (id, name, address)
VALUES ('{id}', '{id}', 'seed')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_meeting_room (id, name, building_id, capacity)
VALUES ('{id}', '{id}', '{id}', 10)
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

INSERT INTO x_meeting (id, title, content, room_id, creator, status,
                       applied, invited, start_time, end_time)
VALUES ('{id}', '{id}', 'seed', '{id}', 'xadmin-seed-0001', 'draft',
        true, true, NOW(), NOW() + INTERVAL '1 hour')
ON CONFLICT (id) DO UPDATE SET deleted_at = NULL;

