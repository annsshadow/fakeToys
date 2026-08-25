-- 050 rollback

ALTER TABLE "x_ai_chat"  DROP COLUMN IF EXISTS "extra";

ALTER TABLE "x_ai_index" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_ai_index" DROP COLUMN IF EXISTS "synced";
