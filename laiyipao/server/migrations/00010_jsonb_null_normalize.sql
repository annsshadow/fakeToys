-- 00010: 把 jsonb `null` 归一化成 `{}` / `[]`
--
-- ## 背景
--
-- 上报集合字段（`elements_used` / `reactions_used` / `terrain_used`）
-- 此前直接 `json.Marshal` 入库。Go 的 `json.Marshal(map[string]int(nil))`
-- 返回 `[]byte("null")`，于是 nil map 存进去就是 **jsonb `null`**
-- （注意不是 SQL NULL —— `WHERE col IS NULL` 查不到它们）。
--
-- ## 危害（实测）
--
-- 1. **统计静默漏行**
--    后台的反应分布查询写的是 `WHERE reactions_used <> '{}'::jsonb`，
--    而 `null <> '{}'` 在 SQL 里求值为 NULL（不是 true），
--    所以这些行被排除。实测 58 行里有 8 行（13.8%）带 jsonb null。
--
--    ⚠️ 这个 bug **不能**用「少统计了多少反应次数」量化。
--    `reactions`（标量）与 `sum(reactions_used)`（map 之和）在引擎里
--    是相邻两行自增、本应恒等；但库里现有的 58 行**全是 e2e 手搓的
--    payload**，两者互相矛盾（标量 90 / map 之和 53，全表 0 行相等）。
--    拿它们相减得到的「漏了多少」是拿矛盾数据算出来的，没有意义。
--
--    另需注意：本迁移**恢复不了**已丢失的明细。
--    null → {} 只保证「今后不再产生新的 null」，
--    那 8 行原本就没写过 per-key 明细，改成 {} 之后它们仍然是空的。
--
-- 2. **任何 JSONB 函数调用直接报错**
--    `jsonb_each_text(jsonb 'null')` 抛「不能在非对象上调用」——
--    也就是说将来谁写一条更合理的聚合 SQL，会当场崩掉。
--    本轮修查询时第一版就撞上了这个错。
--
-- ## 触发条件不需要恶意
--
-- 客户端省字段（e2e 的「谎报星级」用例就是故意的）、
-- 第三方工具、老版本客户端，都会走到这里。
--
-- ## 为什么不靠写侧修就够了
--
-- 写侧已经归一化（`orEmptyMap` / `orEmptySlice`），
-- 但**存量数据不会自己变**，而读侧不该依赖写侧 ——
-- 历史导入、第三方直连、手工修数据都可能再带 `null` 进来。
-- 所以读侧的查询也改成了 `jsonb_typeof(...) = 'object'`。
--
-- ## 幂等
--
-- 只改 jsonb 类型为 null 的行，重复执行是安全的。

-- +goose Up

-- 1) 三个上报集合：null → 空对象 / 空数组
UPDATE battle_records
   SET elements_used  = '{}'::jsonb
 WHERE jsonb_typeof(elements_used) = 'null';

UPDATE battle_records
   SET reactions_used = '{}'::jsonb
 WHERE jsonb_typeof(reactions_used) = 'null';

UPDATE battle_records
   SET terrain_used   = '[]'::jsonb
 WHERE jsonb_typeof(terrain_used) = 'null';

-- 2) 结算时的卡牌序列是 TEXT 而非 JSONB，空串已经是它的「空」表示，
--    这里不动。build_snapshot / loot 理论上不会是 null
--    （服务端自己构造的），但顺手也归一化，成本极低。
UPDATE battle_records
   SET build_snapshot = '{}'::jsonb
 WHERE build_snapshot IS NULL OR jsonb_typeof(build_snapshot) = 'null';

UPDATE battle_records
   SET loot = '{}'::jsonb
 WHERE loot IS NULL OR jsonb_typeof(loot) = 'null';

-- +goose Down
-- 回滚无法恢复「哪些行原本是 null」——
-- 那份信息一旦被抹掉就找不回来了。
-- Down 段因此是**有意的空操作**：
--   `{}` 与 `null` 在业务语义上等价（都是「本局没用到」/「没采集到」），
--   把它们改回去只会让统计重新开始漏数据。
SELECT 1;
