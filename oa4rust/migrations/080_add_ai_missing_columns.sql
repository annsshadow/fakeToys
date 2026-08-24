-- 050: AI 模块端点闭合（plan002 U2）所需缺失列。
--   * x_ai_chat.extra        — chat/write/completion/extra 写入对话扩展数据
--   * x_ai_index.content     — index/cms/doc/{docId} 回读文档正文
--   * x_ai_index.synced      — index/sync/to/knowledge 批量同步标记

ALTER TABLE "x_ai_chat"  ADD COLUMN IF NOT EXISTS "extra"   JSONB;

ALTER TABLE "x_ai_index" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_ai_index" ADD COLUMN IF NOT EXISTS "synced"  BOOLEAN NOT NULL DEFAULT false;
