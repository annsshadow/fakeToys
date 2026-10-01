-- plan002 U2: query_assemble_designer / query_assemble_surface v9 缺口闭合所需增量
--   permission 列（POST */permission 族写入口）
--   时间/归属列补齐（新写路径 SET update_time / 归属门禁所需，
--     部分 handler 此前已引用但从未建列，运行时必然报错——一并补齐）
--   surface 执行记录族 import_model_id/status（既有 handler 已引用）
-- Idempotent: safe to run repeatedly.

ALTER TABLE "x_query_import_model" ADD COLUMN IF NOT EXISTS "permission" TEXT;
ALTER TABLE "x_query_design" ADD COLUMN IF NOT EXISTS "permission" TEXT;
ALTER TABLE "x_query_table" ADD COLUMN IF NOT EXISTS "permission" TEXT;
ALTER TABLE "x_query_view" ADD COLUMN IF NOT EXISTS "permission" TEXT;

ALTER TABLE "x_query_design" ADD COLUMN IF NOT EXISTS "update_time" TIMESTAMP;
ALTER TABLE "x_query_view" ADD COLUMN IF NOT EXISTS "update_time" TIMESTAMP;

ALTER TABLE "x_query_entity_property" ADD COLUMN IF NOT EXISTS "entity" TEXT;
ALTER TABLE "x_query_entity_property" ADD COLUMN IF NOT EXISTS "category" TEXT;
ALTER TABLE "x_query_entity_property" ADD COLUMN IF NOT EXISTS "sort_order" INTEGER DEFAULT 0;

ALTER TABLE "x_query_output" ADD COLUMN IF NOT EXISTS "query_flag" TEXT;
ALTER TABLE "x_query_output" ADD COLUMN IF NOT EXISTS "select_file" TEXT;

ALTER TABLE "x_query_import_model_record" ADD COLUMN IF NOT EXISTS "import_model_id" VARCHAR(255);
ALTER TABLE "x_query_import_model_record" ADD COLUMN IF NOT EXISTS "status" VARCHAR(50);

CREATE INDEX IF NOT EXISTS idx_x_query_import_model_query_flag ON x_query_import_model(query_flag);
CREATE INDEX IF NOT EXISTS idx_x_query_import_model_record_model ON x_query_import_model_record(model_flag);
