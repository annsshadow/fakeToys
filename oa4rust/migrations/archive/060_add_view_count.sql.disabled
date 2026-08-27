-- Migration 060: Add view_count column to x_cms_document.
-- Fix (plan002 found-issue): the document_id_view_count endpoint in
-- cms_assemble_control executes
--   UPDATE x_cms_document SET view_count = view_count + 1 WHERE id = $1
-- but no earlier migration ever created the view_count column, so the
-- endpoint failed at runtime with a 500 (undefined column).
-- Idempotent; follows the precedent of 058_add_fulltext_search.sql.
-- Rollback file: 060_add_view_count_rollback.sql

ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "view_count" BIGINT NOT NULL DEFAULT 0;
