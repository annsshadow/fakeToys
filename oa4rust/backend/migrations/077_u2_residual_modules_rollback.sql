-- 077_u2_residual_modules.sql 回滚：删除本迁移新建的表/索引/列。
-- 幂等（IF EXISTS），可重复执行。对已有列（072 建立的 x_org_empower_log、
-- x_query_table_data 及更早的 x_correlation 基础列）不做删除，仅撤销本迁移新增部分。

-- ── 9) 动态表数据行 ────────────────────────────────────────────────────────
DROP INDEX IF EXISTS "idx_x_query_table_data_flag_bundle";
ALTER TABLE "x_query_table_data" DROP COLUMN IF EXISTS "bundle";

-- ── 8) 神经网络任务 ────────────────────────────────────────────────────────
DROP INDEX IF EXISTS "idx_x_query_neural_job_model";
DROP TABLE IF EXISTS "x_query_neural_job";

-- ── 7) 索引附加文档 ────────────────────────────────────────────────────────
DROP INDEX IF EXISTS "uq_x_query_index_extra_doc";
DROP TABLE IF EXISTS "x_query_index_extra";

-- ── 6) 索引任务状态 ────────────────────────────────────────────────────────
DROP INDEX IF EXISTS "uq_x_query_index_state";
DROP TABLE IF EXISTS "x_query_index_state";

-- ── 5) 关联内容表新增列与索引 ──────────────────────────────────────────────
DROP INDEX IF EXISTS "idx_x_correlation_from_site";
DROP INDEX IF EXISTS "idx_x_correlation_target";
DROP INDEX IF EXISTS "idx_x_correlation_from";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "view";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "site";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "target_creator_person";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "target_start_time";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "target_category";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "target_title";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "target_bundle";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "target_type";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "from_bundle";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "from_type";
ALTER TABLE "x_correlation" DROP COLUMN IF EXISTS "person";

-- ── 4) 授权日志新增列与索引 ────────────────────────────────────────────────
DROP INDEX IF EXISTS "idx_x_org_empower_log_title";
DROP INDEX IF EXISTS "idx_x_org_empower_log_to_person";
DROP INDEX IF EXISTS "idx_x_org_empower_log_from_person";
ALTER TABLE "x_org_empower_log" DROP COLUMN IF EXISTS "activity_name";
ALTER TABLE "x_org_empower_log" DROP COLUMN IF EXISTS "activity";
ALTER TABLE "x_org_empower_log" DROP COLUMN IF EXISTS "empower_time";
ALTER TABLE "x_org_empower_log" DROP COLUMN IF EXISTS "title";
ALTER TABLE "x_org_empower_log" DROP COLUMN IF EXISTS "to_person";
ALTER TABLE "x_org_empower_log" DROP COLUMN IF EXISTS "from_person";

-- ── 3) 人员扩展属性 ────────────────────────────────────────────────────────
DROP INDEX IF EXISTS "uq_x_org_person_extend_person_type";
DROP TABLE IF EXISTS "x_org_person_extend";

-- ── 2) 用户自定义数据 ──────────────────────────────────────────────────────
DROP INDEX IF EXISTS "uq_x_org_definition_name";
DROP TABLE IF EXISTS "x_org_definition";

-- ── 2) 扫码绑定记录 ────────────────────────────────────────────────────────
DROP INDEX IF EXISTS "idx_x_org_bind_record_create";
DROP TABLE IF EXISTS "x_org_bind_record";

