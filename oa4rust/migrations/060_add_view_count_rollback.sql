-- Rollback for 060_add_view_count.sql: drop the view_count column.

ALTER TABLE "x_cms_document" DROP COLUMN IF EXISTS "view_count";
