-- 00008: 出战选牌决策（I-6 重放闭环的最后一环）
--
-- 背景：战斗在波次间隙进入 card_select 阶段，玩家从 3 张手牌里选 1 张。
-- 该选择会改变后续战斗（技能升格 / 属性加成 / 机制词条），
-- 因此**必须能被第三方重放复现**，否则验真会误判正常对局为伪造。
--
-- 存的是"每波选中的那张牌的索引"，-1 表示整波跳过。
-- 选中的索引是稳定的：同一关卡 + 同一种子 → 同样的手牌 → 同样的索引。
--
-- 为什么不用 JSONB 数组而是 TEXT：
-- 数组极小（每波一个整数，100 关最多 800 个），
-- 用紧凑文本比 JSONB 省空间也省解析开销，且便于直接比对。

-- +goose Up
ALTER TABLE battle_records
    ADD COLUMN IF NOT EXISTS card_picks TEXT NOT NULL DEFAULT '';

COMMENT ON COLUMN battle_records.card_picks IS
    '每波选中的手牌索引（-1=跳过），逗号分隔。I-6 重放复现选牌决策用。';

-- +goose Down
ALTER TABLE battle_records
    DROP COLUMN IF EXISTS card_picks;
