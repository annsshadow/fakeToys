-- Migration 042: complete the calendar column set (column-cascade resolution).
--
-- The previous clean PG log only surfaced cal_calendar.createor and
-- cal_event.start_time, but Postgres reports only the first missing column per
-- query. The calendar_assemble_control handler selects the full column lists
-- below, so all referenced columns must exist:
--
--   cal_calendar : id, name, type, target, color, description, source,
--                  createor, is_public, status
--   cal_event    : id, calendar_id, title, content, location, start_time,
--                  end_time, all_day, visibility, status, createor
--
-- Types are derived from how the handler reads each column:
--   createor/start_time/end_time/visibility/status -> String -> TEXT
--   all_day                                       -> Bool   -> BOOLEAN
--
-- Both tables are currently empty, so NOT NULL DEFAULT is safe to add.

ALTER TABLE "cal_calendar" ADD COLUMN IF NOT EXISTS "createor" TEXT;

ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "start_time" TEXT;
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "end_time" TEXT;
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "all_day" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "visibility" TEXT;
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "status" TEXT;
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "createor" TEXT;
