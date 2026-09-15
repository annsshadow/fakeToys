-- 094_create_system_config_table.sql
-- 补齐 /jaxrs/config/* 斜杠路径家族所需的通用系统配置表。
-- 此前 /jaxrs/config/create 在桌面契约守卫 KNOWN_BACKEND_GAPS 中声明为"后端未实现"，
-- 现建通用配置表 + 实装 create handler，使配置入口可操作。

CREATE TABLE IF NOT EXISTS "x_system_config" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT,
    "value" TEXT,
    "category" TEXT,
    "description" TEXT,
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);
