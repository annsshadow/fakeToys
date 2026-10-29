-- B2（2026-10-30）：回滚 x_program_invoke 的 token_hash 列。
ALTER TABLE "x_program_invoke"
    DROP COLUMN IF EXISTS "token_hash";