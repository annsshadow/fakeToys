-- Rollback for migration 024: Gap closure tables

DROP INDEX IF EXISTS idx_x_cms_doc_view_count_doc;
DROP INDEX IF EXISTS idx_x_ai_chat_conv;
DROP INDEX IF EXISTS idx_x_cms_commend_doc;

DROP TABLE IF EXISTS x_cms_document_view_count;
DROP TABLE IF EXISTS x_ai_chat;
DROP TABLE IF EXISTS x_cms_commend;
