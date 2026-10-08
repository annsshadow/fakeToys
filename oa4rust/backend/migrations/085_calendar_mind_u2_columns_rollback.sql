-- plan002 U2：085_calendar_mind_u2_columns.sql 的回滚（幂等）。

DROP TABLE IF EXISTS "cal_calendar_follow";
DROP TABLE IF EXISTS "cal_message";
DROP TABLE IF EXISTS "x_mind_share";

ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "master_id";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "rfc_text";

ALTER TABLE "cal_calendar" DROP COLUMN IF EXISTS "follow_enabled";

ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "parent_id";
ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "folder_id";
ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "icon";
ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "description";
ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "shared";
ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "file_version";
ALTER TABLE "x_mind" DROP COLUMN IF EXISTS "creator_unit";
