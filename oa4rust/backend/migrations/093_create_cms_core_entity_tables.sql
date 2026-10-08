-- 093_create_cms_core_entity_tables.sql
-- 补齐 cms/core/entity/* 斜杠路径家族（column / column_manager / index / module / note）
-- 所需的真实表。此前这些端点在桌面契约守卫 KNOWN_BACKEND_GAPS 中声明为"后端未实现"，
-- 现按 O2OA CMS 核心实体语义建表 + 实装 list handler，使对应视图可操作而非降级横幅。

CREATE TABLE IF NOT EXISTS "x_cms_column" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT,
    "parent_id" TEXT,
    "sort_order" BIGINT,
    "description" TEXT,
    "status" TEXT DEFAULT 'active',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_cms_column_manager" (
    "id" TEXT PRIMARY KEY,
    "column_id" TEXT,
    "person_id" TEXT,
    "role" TEXT DEFAULT 'manager',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_cms_index" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT,
    "target" TEXT,
    "sort_order" BIGINT,
    "description" TEXT,
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_cms_module" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT,
    "app_id" TEXT,
    "module_type" TEXT,
    "description" TEXT,
    "status" TEXT DEFAULT 'active',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_cms_note" (
    "id" TEXT PRIMARY KEY,
    "title" TEXT,
    "content" TEXT,
    "person_id" TEXT,
    "status" TEXT DEFAULT 'active',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);
