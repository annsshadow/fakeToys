-- x_file: file assemble control handlers persist and select file metadata
-- columns (path, size, folder_id) that were missing from the base table.
ALTER TABLE "x_file" ADD COLUMN IF NOT EXISTS "path" TEXT;
ALTER TABLE "x_file" ADD COLUMN IF NOT EXISTS "size" BIGINT;
ALTER TABLE "x_file" ADD COLUMN IF NOT EXISTS "folder_id" TEXT;
