-- 071_query_u2_closures rollback
DROP INDEX IF EXISTS idx_x_query_statement_name;
DROP INDEX IF EXISTS idx_x_query_statement_query;
DROP TABLE IF EXISTS "x_query_statement";

ALTER TABLE "x_query_stat" DROP COLUMN IF EXISTS "config";
ALTER TABLE "x_query_stat" DROP COLUMN IF EXISTS "permission";
ALTER TABLE "x_query_view" DROP COLUMN IF EXISTS "bundle_data";
ALTER TABLE "x_query_view" DROP COLUMN IF EXISTS "bundle_data_v2";
ALTER TABLE "x_query_view" DROP COLUMN IF EXISTS "excel_data";
ALTER TABLE "x_query_design" DROP COLUMN IF EXISTS "flag";
ALTER TABLE "x_query_design" DROP COLUMN IF EXISTS "icon";
ALTER TABLE "x_query_import_model" DROP COLUMN IF EXISTS "flag";
ALTER TABLE "x_query_import_model" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_query_table" DROP COLUMN IF EXISTS "status";
ALTER TABLE "x_query_table" DROP COLUMN IF EXISTS "reloaded";
ALTER TABLE "x_query_input" DROP COLUMN IF EXISTS "content";
