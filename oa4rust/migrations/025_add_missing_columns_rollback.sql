-- Rollback for migration 025: Add missing columns

ALTER TABLE x_ai_chat DROP COLUMN IF EXISTS creator;

ALTER TABLE "FILE_FILE" DROP COLUMN IF EXISTS content;