-- 082_program_center_u2_closure.sql 的回滚：移除 x_program_agent.enable。
ALTER TABLE IF EXISTS "x_program_agent"
    DROP COLUMN IF EXISTS "enable";
