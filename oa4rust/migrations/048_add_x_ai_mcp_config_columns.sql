-- Add o2server X_AI_MCP_CONFIG business columns referenced by the ai crate config handlers.
-- Migration 032 stubbed x_ai_mcp_config with only id + audit columns; the handlers
-- (config_list_mcp_paging / config_get_mcp) query name/url/default_model/enabled/
-- temperature/is_base/is_extended/max_tokens.
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "name" TEXT;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "url" TEXT;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "default_model" TEXT;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "enabled" BOOLEAN;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "temperature" DOUBLE PRECISION;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "is_base" BOOLEAN;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "is_extended" BOOLEAN;
ALTER TABLE "x_ai_mcp_config" ADD COLUMN IF NOT EXISTS "max_tokens" BIGINT;
