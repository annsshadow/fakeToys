-- 00014_gem_affix_pool.sql —— 宝石洗练闭环的前置 DDL

-- +goose Up

-- # 为什么这一版需要动表
--
-- 第 150 轮把洗练券接上消费端点时，发现两处「内容与结构脱节」：
--
-- 1. `gem_affixes` 表建好了却**一直是空的**，而且**没有 `(gem_id, affix)` 唯一约束**。
--    没有约束的话，播种里的 `ON CONFLICT (gem_id, affix)` 会直接报错
--    （PG：no unique or exclusion constraint matching the ON CONFLICT
--    specification）—— 症状是「整个种子跑不起来」，而真实原因在很前面一行。
--    有约束之后，重复播种才是幂等的。
--
-- 2. `gems` 表缺 `descr` 列。`domain.SeedGem` 一直带着 Descr，
--    客户端从 `/config` 读到的是**内存里的**那一版，而后台与
--    `GET /me/gems` 读的是**数据库里的**另一版（没有描述）。
--    同一份内容在两条读路径上不一致，而两边都「看起来正常」。

-- # 唯一约束
--
-- 同一种宝石里同名词条只能有一条，否则 roll 出来的两条会重复加值，
-- 玩家看到的是「这颗宝石有两条一样的词条」。
ALTER TABLE gem_affixes
    ADD CONSTRAINT gem_affixes_gem_id_affix_key UNIQUE (gem_id, affix);

-- # 描述列
--
-- 已有行没有描述，留空串（而不是 NULL）：`domain.SeedGem.Descr` 对所有
-- 8 种宝石都有值，播种会把它们补齐；留空串是「尚未灌种子」的中间态。
ALTER TABLE gems
    ADD COLUMN IF NOT EXISTS descr TEXT NOT NULL DEFAULT '';

UPDATE gems SET descr = '攻击力提升' WHERE code = 'gem_atk'   AND descr = '';
UPDATE gems SET descr = '提高元素系数 —— 元素伤害的核心' WHERE code = 'gem_elem'  AND descr = '';
UPDATE gems SET descr = '暴击率提升' WHERE code = 'gem_crit'     AND descr = '';
UPDATE gems SET descr = '提高反应倍率' WHERE code = 'gem_react'  AND descr = '';
UPDATE gems SET descr = '提高元素上限 —— 无可替代' WHERE code = 'gem_cap'  AND descr = '';
UPDATE gems SET descr = '提高热量上限' WHERE code = 'gem_heat'      AND descr = '';
UPDATE gems SET descr = '提高护甲' WHERE code = 'gem_armor'         AND descr = '';
UPDATE gems SET descr = '提高技能伤害倍率' WHERE code = 'gem_skill'   AND descr = '';

-- +goose Down

-- ⚠️ 回滚会**丢掉所有已洗练出来的词条**（数据不可逆）。
-- 这是刻意的：约束一旦放开，重复词条就会重新进入 roll 的候选集。
--
-- 理由写到迁移里，是为了让「为什么 Down 不对称」在半年后仍可查 ——
-- 否则下一个人会以为这里只是漏写了 Down。
ALTER TABLE gems DROP COLUMN IF EXISTS descr;
ALTER TABLE gem_affixes DROP CONSTRAINT IF EXISTS gem_affixes_gem_id_affix_key;
