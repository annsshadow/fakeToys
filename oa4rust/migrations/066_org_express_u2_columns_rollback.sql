-- Rollback of 066_org_express_u2_columns.sql

DROP INDEX IF EXISTS idx_x_org_identity_person;
ALTER TABLE "x_org_identity" DROP COLUMN IF EXISTS "major";
ALTER TABLE "x_org_identity" DROP COLUMN IF EXISTS "person_id";
