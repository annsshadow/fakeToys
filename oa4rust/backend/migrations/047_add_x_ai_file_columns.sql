-- Add o2server X.AI_FILE business columns referenced by the ai crate file handlers.
-- Migrations 032/034 stubbed x_ai_file with only id + audit columns; the handlers
-- (file_download / file_download_scale / file_delete) query the x-prefixed business
-- columns per the o2server column convention used by sibling tables (e.g. x_ai_model).
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xid" TEXT;
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xname" TEXT;
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xlength" BIGINT;
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xstorage" TEXT;
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xcreator" TEXT;
ALTER TABLE "x_ai_file" ADD COLUMN IF NOT EXISTS "xcreateTime" TEXT;
