-- Rollback for 044_fix_columns_and_types.sql
ALTER TABLE "bbs_forum_info" DROP COLUMN IF EXISTS "disable";
ALTER TABLE "bbs_section_info" DROP COLUMN IF EXISTS "disable";
ALTER TABLE "x_org_group_member" DROP COLUMN IF EXISTS "role_id";
ALTER TABLE "cal_calendar" ALTER COLUMN "is_public" TYPE TEXT
    USING (CASE WHEN "is_public" THEN 'true' ELSE 'false' END);
