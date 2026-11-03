-- 115_o10_fix_x_org_definition_columns 的回滚：按列删除，幂等。
ALTER TABLE "x_org_definition" DROP COLUMN IF EXISTS "data";
ALTER TABLE "x_org_definition" DROP COLUMN IF EXISTS "creator";
ALTER TABLE "x_org_definition" DROP COLUMN IF EXISTS "create_time";
ALTER TABLE "x_org_definition" DROP COLUMN IF EXISTS "update_time";
