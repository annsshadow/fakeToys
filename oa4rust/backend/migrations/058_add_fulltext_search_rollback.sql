-- Rollback migration 058: drop full-text search indexes and remove added columns.
DROP INDEX IF EXISTS idx_x_cms_document_fts;
DROP INDEX IF EXISTS idx_bbs_subject_info_fts;
DROP INDEX IF EXISTS idx_x_message_fts;

ALTER TABLE "x_cms_document" DROP COLUMN IF EXISTS "title";
ALTER TABLE "x_cms_document" DROP COLUMN IF EXISTS "content";
