-- 078_cms_u2_batch3.sql 回滚：仅撤销本迁移新增的列与索引。
-- 幂等（IF EXISTS），可重复执行。

DROP INDEX IF EXISTS "idx_x_cms_data_document_batch_name";
ALTER TABLE "x_cms_data_document" DROP COLUMN IF EXISTS "batch_name";
