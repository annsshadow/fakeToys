-- 067 rollback

DROP INDEX IF EXISTS idx_x_org_personcard_name;
DROP TABLE IF EXISTS x_org_personcard CASCADE;
DROP INDEX IF EXISTS idx_x_org_person_status;
DROP INDEX IF EXISTS idx_x_org_identity_person;

ALTER TABLE "x_org_identity" DROP COLUMN IF EXISTS "pinyin_initial";
ALTER TABLE "x_org_identity" DROP COLUMN IF EXISTS "person_id";
ALTER TABLE "x_org_role" DROP COLUMN IF EXISTS "pinyin_initial";
ALTER TABLE "x_org_group" DROP COLUMN IF EXISTS "pinyin_initial";
ALTER TABLE "x_org_unit" DROP COLUMN IF EXISTS "pinyin_initial";

ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "pinyin_initial";
ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "icon";
ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "password_expired_time";
ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "lock_expired_time";
ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "status_des";
ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "status";
ALTER TABLE "x_org_person" DROP COLUMN IF EXISTS "password";
