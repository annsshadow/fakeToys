-- plan002 U2（program_center 终扫闭合）：
-- Java AgentAction {flag}/disable|enable|execute 需要启用位；
-- x_program_agent 生成表（032）无该列，补可空默认的布尔列。
-- 幂等：IF NOT EXISTS，可重复执行；回滚见 082_program_center_u2_closure_rollback.sql。

ALTER TABLE IF EXISTS "x_program_agent"
    ADD COLUMN IF NOT EXISTS "enable" BOOLEAN NOT NULL DEFAULT TRUE;
