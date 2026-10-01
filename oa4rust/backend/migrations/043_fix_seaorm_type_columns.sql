-- Migration 043: fix remaining column mismatches surfaced by parity probe.
--
-- 1. bbs_subject_info is missing the `content` column. The SeaORM entity
--    (bbs_subject_info::Model) declares `content: Option<String>`, so the
--    generated SELECT references it and fails with "column does not exist".
--
-- 2. x_org_identity / x_cal_calendar / x_org_definition were created with a
--    `type_` column (trailing underscore), but the SeaORM entities name the
--    field `type_`, which SeaORM maps to the column `type`
--    (PascalCase `Type` -> snake_case `type`). The query therefore references
--    `type` and fails. O2OA's canonical column is the quoted reserved word
--    `type` (used by the majority of tables), so rename to match. No raw SQL
--    references `type_`, so the rename is safe.
--
-- 3. auth_person_identity was created with `person_unique`, but every query
--    references `pi.person_id` (e.g. batch_query.rs, auth/src/lib.rs). Rename
--    to the canonical `person_id`. No Rust code references `person_unique` as
--    a column, so the rename is safe (dependent indexes update automatically).

ALTER TABLE "bbs_subject_info" ADD COLUMN IF NOT EXISTS "content" TEXT;

ALTER TABLE "auth_person_identity" RENAME COLUMN "person_unique" TO "person_id";

DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x_org_identity' AND column_name = 'type_'
  ) THEN
    ALTER TABLE "x_org_identity" RENAME COLUMN "type_" TO "type";
  END IF;
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x_cal_calendar' AND column_name = 'type_'
  ) THEN
    ALTER TABLE "x_cal_calendar" RENAME COLUMN "type_" TO "type";
  END IF;
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_name = 'x_org_definition' AND column_name = 'type_'
  ) THEN
    ALTER TABLE "x_org_definition" RENAME COLUMN "type_" TO "type";
  END IF;
END $$;
