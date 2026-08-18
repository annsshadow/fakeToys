-- 030: add columns that the *_core_entity SeaORM entities select but migration 029
-- omitted or that the deployed tables never had. Without these, the list
-- endpoints 500 with "column X does not exist" / "column X.type does not exist".
-- Uses ADD COLUMN IF NOT EXISTS for idempotency (PostgreSQL 9.6+).

-- x_script: script_list / script_create select & insert creator_person, deleted_at.
ALTER TABLE "x_script" ADD COLUMN IF NOT EXISTS "creator_person" TEXT NOT NULL DEFAULT '';
ALTER TABLE "x_script" ADD COLUMN IF NOT EXISTS "deleted_at" TIMESTAMP;

-- x_org_group: group_list selects parent_id, level (deployed table only had
-- unit_id/type/creator/create_time).
ALTER TABLE "x_org_group" ADD COLUMN IF NOT EXISTS "parent_id" TEXT;
ALTER TABLE "x_org_group" ADD COLUMN IF NOT EXISTS "level" INTEGER NOT NULL DEFAULT 0;

-- x_org_identity: identity_list selects person_id (deployed table only had
-- identity_id/unit_id/creator/create_time).
ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "person_id" TEXT NOT NULL DEFAULT '';
