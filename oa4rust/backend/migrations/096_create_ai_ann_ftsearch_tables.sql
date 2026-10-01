-- 096_create_ai_announcement_ftsearch_tables.sql
-- 补齐 /api/ai/assemble/control/ann/list 与 /api/ftsearch/list 斜杠路径家族所需的表。
-- ai_ann：AI 模块公告/模型公告；ftsearch：全文检索索引文档。

CREATE TABLE IF NOT EXISTS "x_ai_ann" (
    "id" TEXT PRIMARY KEY,
    "title" TEXT,
    "content" TEXT,
    "category" TEXT,
    "status" TEXT DEFAULT 'active',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_ftsearch_document" (
    "id" TEXT PRIMARY KEY,
    "title" TEXT,
    "body" TEXT,
    "source" TEXT,
    "score" DOUBLE PRECISION DEFAULT 0,
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);
