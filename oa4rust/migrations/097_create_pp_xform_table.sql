-- 097_create_pp_xform_table.sql
-- 补齐 /api/processplatform/assemble/designer/xform/list 斜杠路径家族（ProcessXformApp）所需的表。
-- x_pp_xform：流程平台 XForm 设计器条目（名称/定义 JSON/状态，与 CMS XForm 同语义）。

CREATE TABLE IF NOT EXISTS "x_pp_xform" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT,
    "definition" TEXT,
    "status" TEXT DEFAULT 'draft',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);
