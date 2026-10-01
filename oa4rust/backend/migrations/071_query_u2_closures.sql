-- plan002 U2: query_assemble_designer / query_assemble_surface 端点全量闭合
-- 1) x_query_statement: Java Statement 实体对应表（designer/surface statement 族 CRUD+execute 落点）
-- 2) 补齐既有 parity 表缺失列（handler 真实查库所需，此前从未建列）:
--    x_query_stat.config/permission, x_query_view.bundle_data/bundle_data_v2/excel_data,
--    x_query_design.flag/icon, x_query_import_model.flag/content,
--    x_query_table.status/reloaded, x_query_input.content

CREATE TABLE IF NOT EXISTS "x_query_statement" (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    alias VARCHAR(255),
    query_flag VARCHAR(255),
    entity_class VARCHAR(500),
    entity_category VARCHAR(100),
    type VARCHAR(50),
    data TEXT,
    counting_data TEXT,
    permission TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_query_statement_query ON x_query_statement(query_flag);
CREATE INDEX IF NOT EXISTS idx_x_query_statement_name ON x_query_statement(name);

ALTER TABLE "x_query_stat" ADD COLUMN IF NOT EXISTS "config" TEXT;
ALTER TABLE "x_query_stat" ADD COLUMN IF NOT EXISTS "permission" TEXT;
ALTER TABLE "x_query_view" ADD COLUMN IF NOT EXISTS "bundle_data" TEXT;
ALTER TABLE "x_query_view" ADD COLUMN IF NOT EXISTS "bundle_data_v2" TEXT;
ALTER TABLE "x_query_view" ADD COLUMN IF NOT EXISTS "excel_data" TEXT;
ALTER TABLE "x_query_design" ADD COLUMN IF NOT EXISTS "flag" TEXT;
ALTER TABLE "x_query_design" ADD COLUMN IF NOT EXISTS "icon" TEXT;
ALTER TABLE "x_query_import_model" ADD COLUMN IF NOT EXISTS "flag" TEXT;
ALTER TABLE "x_query_import_model" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_query_table" ADD COLUMN IF NOT EXISTS "status" TEXT;
ALTER TABLE "x_query_table" ADD COLUMN IF NOT EXISTS "reloaded" BOOLEAN DEFAULT FALSE;
ALTER TABLE "x_query_input" ADD COLUMN IF NOT EXISTS "content" TEXT;
