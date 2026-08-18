-- 044: fix column types / add missing columns surfaced by the parity probe.
--
-- 1) bbs_forum_info / bbs_section_info: the SeaORM entities declare
--    `disable: bool`, so SeaORM SELECTs a `disable` column. It does not exist yet.
ALTER TABLE "bbs_forum_info" ADD COLUMN IF NOT EXISTS "disable" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "bbs_section_info" ADD COLUMN IF NOT EXISTS "disable" BOOLEAN NOT NULL DEFAULT false;

-- 2) x_org_group_member: org_assemble_personal queries `m.role_id`, which is missing.
ALTER TABLE "x_org_group_member" ADD COLUMN IF NOT EXISTS "role_id" TEXT;

-- 3) cal_calendar.is_public is declared TEXT by an earlier migration, but the Rust
--    model treats it as BOOLEAN (struct field `bool`, INSERT/UPDATE bind a bool,
--    and the public-calendar query does `WHERE is_public = true`). Comparing a
--    TEXT column to a boolean literal fails with "operator does not exist: text = boolean".
--    Convert the column to BOOLEAN so the model and the SQL are type-consistent.
--    The table is currently empty, so the USING cast is a no-op but kept defensive.
ALTER TABLE "cal_calendar" ALTER COLUMN "is_public" TYPE BOOLEAN
    USING (CASE
        WHEN "is_public" IN ('true', 't', '1', 'y', 'yes') THEN true
        WHEN "is_public" IN ('false', 'f', '0', 'n', 'no') THEN false
        ELSE false
    END);
