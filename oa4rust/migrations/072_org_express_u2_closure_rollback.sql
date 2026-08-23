-- Migration 071 rollback: 撤销 express U2 收尾支撑结构

DROP INDEX IF EXISTS idx_x_org_empower_log_from;
DROP TABLE IF EXISTS x_org_empower_log;

DROP INDEX IF EXISTS idx_x_empower_to_identity;
DROP INDEX IF EXISTS idx_x_empower_from_identity;
ALTER TABLE "x_empower" DROP COLUMN IF EXISTS "to_identity";
ALTER TABLE "x_empower" DROP COLUMN IF EXISTS "from_identity";

DROP INDEX IF EXISTS idx_x_org_unit_type;
ALTER TABLE "x_org_unit" DROP COLUMN IF EXISTS "type";

DROP INDEX IF EXISTS idx_x_org_group_parent;
ALTER TABLE "x_org_group" DROP COLUMN IF EXISTS "parent_id";
