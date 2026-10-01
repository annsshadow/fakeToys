-- 083_query_u2_v9_closure 回滚
ALTER TABLE "x_query_import_model" DROP COLUMN IF EXISTS "permission";
ALTER TABLE "x_query_design" DROP COLUMN IF EXISTS "permission";
ALTER TABLE "x_query_table" DROP COLUMN IF EXISTS "permission";
ALTER TABLE "x_query_view" DROP COLUMN IF EXISTS "permission";

ALTER TABLE "x_query_design" DROP COLUMN IF EXISTS "update_time";
ALTER TABLE "x_query_view" DROP COLUMN IF EXISTS "update_time";

ALTER TABLE "x_query_entity_property" DROP COLUMN IF EXISTS "entity";
ALTER TABLE "x_query_entity_property" DROP COLUMN IF EXISTS "category";
ALTER TABLE "x_query_entity_property" DROP COLUMN IF EXISTS "sort_order";

ALTER TABLE "x_query_output" DROP COLUMN IF EXISTS "query_flag";
ALTER TABLE "x_query_output" DROP COLUMN IF EXISTS "select_file";

ALTER TABLE "x_query_import_model_record" DROP COLUMN IF EXISTS "import_model_id";
ALTER TABLE "x_query_import_model_record" DROP COLUMN IF EXISTS "status";

DROP INDEX IF EXISTS idx_x_query_import_model_query_flag;
DROP INDEX IF EXISTS idx_x_query_import_model_record_model;
