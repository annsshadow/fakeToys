-- Migration 058: Add PostgreSQL full-text search indexes for cms_document, bbs_subject, and message.
-- Uses websearch_to_tsquery for natural-language full-text search.
-- x_cms_document previously lacked searchable content columns; add title and content if missing.
-- Rollback file: 058_add_fulltext_search_rollback.sql

-- cms_document: add searchable columns if they do not exist
ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "title" TEXT;
ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "content" TEXT;

-- cms_document full-text search vector (title + content)
CREATE INDEX IF NOT EXISTS idx_x_cms_document_fts
    ON "x_cms_document"
    USING GIN (to_tsvector('simple', COALESCE(title, '') || ' ' || COALESCE(content, '')));

-- bbs_subject_info full-text search vector (title + content)
CREATE INDEX IF NOT EXISTS idx_bbs_subject_info_fts
    ON "bbs_subject_info"
    USING GIN (to_tsvector('simple', COALESCE(title, '') || ' ' || COALESCE(content, '')));

-- message full-text search vector (content)
CREATE INDEX IF NOT EXISTS idx_x_message_fts
    ON "x_message"
    USING GIN (to_tsvector('simple', COALESCE(content, '')));
