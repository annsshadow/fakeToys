-- Rollback for migration 043.

ALTER TABLE "bbs_subject_info" DROP COLUMN IF EXISTS "content";

ALTER TABLE "auth_person_identity" RENAME COLUMN "person_id" TO "person_unique";

DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x_org_identity' AND column_name = 'type'
  ) THEN
    ALTER TABLE "x_org_identity" RENAME COLUMN "type" TO "type_";
  END IF;
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x_cal_calendar' AND column_name = 'type'
  ) THEN
    ALTER TABLE "x_cal_calendar" RENAME COLUMN "type" TO "type_";
  END IF;
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x_org_definition' AND column_name = 'type'
  ) THEN
    ALTER TABLE "x_org_definition" RENAME COLUMN "type" TO "type_";
  END IF;
END $$;
