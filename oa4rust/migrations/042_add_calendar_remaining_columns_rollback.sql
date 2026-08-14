-- Rollback for migration 042: drop the columns added to cal_calendar / cal_event.

ALTER TABLE "cal_calendar" DROP COLUMN IF EXISTS "createor";

ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "start_time";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "end_time";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "all_day";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "visibility";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "status";
ALTER TABLE "cal_event" DROP COLUMN IF EXISTS "createor";
