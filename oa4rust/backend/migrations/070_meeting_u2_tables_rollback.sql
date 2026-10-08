DROP INDEX IF EXISTS idx_meeting_room_photo_room;
ALTER TABLE "x_meeting_room_photo" DROP COLUMN IF EXISTS "room_id";
ALTER TABLE "x_meeting" DROP COLUMN IF EXISTS "completed_time";
DROP INDEX IF EXISTS idx_meeting_attachment_meeting;
DROP TABLE IF EXISTS "x_meeting_attachment";
