-- Rollback for 030: drop the columns added in 030.

ALTER TABLE "x_script" DROP COLUMN IF EXISTS "creator_person";
ALTER TABLE "x_script" DROP COLUMN IF EXISTS "deleted_at";
ALTER TABLE "x_org_group" DROP COLUMN IF EXISTS "parent_id";
ALTER TABLE "x_org_group" DROP COLUMN IF EXISTS "level";
ALTER TABLE "x_org_identity" DROP COLUMN IF EXISTS "person_id";
