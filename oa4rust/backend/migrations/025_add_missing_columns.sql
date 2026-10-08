-- Migration 025: Add missing columns
-- 修复 migration 008 遗漏的 FILE_FILE.content 列与 migration 024 遗漏的 x_ai_chat.creator 列

-- x_ai_chat.creator — 对话发起人
ALTER TABLE x_ai_chat ADD COLUMN IF NOT EXISTS creator VARCHAR(255);

-- FILE_FILE.content — 附件原始内容（base64 编码），用于下载与 Office 预览
ALTER TABLE file_file ADD COLUMN IF NOT EXISTS content TEXT;