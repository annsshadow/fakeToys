-- Migration 060: Add view_count column to x_cms_document.
-- Fix (plan002 found-issue): the document_id_view_count endpoint in
-- cms_assemble_control executes
--   UPDATE x_cms_document SET view_count = view_count + 1 WHERE id = $1
-- but no earlier migration ever created the view_count column, so the
-- endpoint failed at runtime with a 500 (undefined column).
-- Idempotent; follows the precedent of 058_add_fulltext_search.sql.
-- Rollback file: 060_add_view_count_rollback.sql
--
-- 历史：本文件曾于 b7bfca566 被删除（"archived"，当时改用手工 SQL 加列），
-- 但没有任何迁移接替它的职责——fresh DB 与本库（060 从未记账）上该列缺失、
-- 端点恒 500。此处按原文恢复；checksum 机制对未记账的库自动应用，幂等无害。

ALTER TABLE "x_cms_document" ADD COLUMN IF NOT EXISTS "view_count" BIGINT NOT NULL DEFAULT 0;
