-- B2（2026-10-30）：x_program_invoke 增加 token 哈希列。
--
-- 背景：`POST /invoke/{flag}/client/{client}/token/{token}/execute` 的 token
-- 此前只判非空（零验签）。本迁移为 x_program_invoke 增加 token_hash 存储列，
-- 只存哈希（格式 `v1:<salt_hex>:<hmac_hex>`，HMAC-SHA256 以随机盐为密钥），
-- 不存明文。方案细节见 docs/plans/2026-10-30-b2-invoke-token-design.md。
ALTER TABLE "x_program_invoke"
    ADD COLUMN IF NOT EXISTS "token_hash" TEXT;