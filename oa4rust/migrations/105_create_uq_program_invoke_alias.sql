-- 105_create_uq_program_invoke_alias.sql
-- 轮62 配套约束：x_program_invoke 的 alias 承担第二查找键语义（u2_invoke_get 按
-- name/alias 任一命中），name 已有 uq_program_invoke_name 兜底并发双建，alias 无
-- 约束时 check-then-insert 竞态可产生重复别名。部分唯一索引只约束非空别名。
CREATE UNIQUE INDEX IF NOT EXISTS uq_program_invoke_alias
    ON x_program_invoke (alias)
    WHERE alias IS NOT NULL AND alias <> '';
