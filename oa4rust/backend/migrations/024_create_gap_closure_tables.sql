-- Migration 024: Gap closure tables for new API endpoints

-- x_cms_commend — 评论/推荐表
-- (Already present in migration 023; CREATE TABLE IF NOT EXISTS ensures idempotency)
CREATE TABLE IF NOT EXISTS x_cms_commend (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    person_id VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- x_ai_chat — AI 对话记录表
CREATE TABLE IF NOT EXISTS x_ai_chat (
    id VARCHAR(255) PRIMARY KEY,
    conversation_id VARCHAR(255),
    role VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- x_cms_document_view_count — 文档阅读计数表
CREATE TABLE IF NOT EXISTS x_cms_document_view_count (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL UNIQUE,
    view_count INTEGER DEFAULT 0,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_x_cms_commend_doc ON x_cms_commend(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_ai_chat_conv ON x_ai_chat(conversation_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_doc_view_count_doc ON x_cms_document_view_count(doc_id);
