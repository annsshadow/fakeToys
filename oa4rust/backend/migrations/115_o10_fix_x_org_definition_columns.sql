-- O10（2026-11-03）：x_org_definition 列补齐。
-- 背景：029 与 077 重复定义同名表（均为 CREATE TABLE IF NOT EXISTS），029 先执行生效，
-- 形状缺 data/creator/create_time/update_time；077 的形状才是 personal/definition
-- handler 的语义本体（个人定义 JSON 存 data，归属存 creator）。scratch 库（全量迁移）
-- 实测 upsert 必炸 42703 "column data does not exist" → 500（IDOR 矩阵 O10 轮实证）。
-- 采用追加 ALTER（不改历史迁移，已有库亦可补齐）；回滚见 115_*_rollback.sql。
ALTER TABLE "x_org_definition" ADD COLUMN IF NOT EXISTS "data" TEXT;
ALTER TABLE "x_org_definition" ADD COLUMN IF NOT EXISTS "creator" TEXT;
ALTER TABLE "x_org_definition" ADD COLUMN IF NOT EXISTS "create_time" TIMESTAMP;
ALTER TABLE "x_org_definition" ADD COLUMN IF NOT EXISTS "update_time" TIMESTAMP;
