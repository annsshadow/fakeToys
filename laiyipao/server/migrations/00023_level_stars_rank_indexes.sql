-- 00023_level_stars_rank_indexes.sql — 关卡榜 / 效率榜索引
--
-- # 为什么需要（2026-10-31 新一轮第 11 轮）
--
-- `MyRank`（`/me/rank`）与 `Leaderboard` 的 stage / efficiency 榜都基于
-- `level_stars` 的聚合：
--
--   stage:       SELECT user_id, MAX(level_id)::bigint AS score
--                FROM level_stars WHERE clears > 0 GROUP BY user_id
--   efficiency:  SELECT user_id, MIN(min_power_clear) AS score
--                FROM level_stars WHERE min_power_clear > 0 GROUP BY user_id
--
-- `level_stars` 的主键是 `(user_id, level_id)`，虽然按 user_id 有序，
-- 但 `clears` / `min_power_clear` 不在键里 —— 过滤条件必须**回表**检查，
-- 聚合无法走 index-only scan，`GROUP BY user_id` 变成全表扫描 + 排序。
--
-- 本迁移加两个**部分索引**（PostgreSQL 部分索引的 WHERE 条件与查询
-- 过滤完全一致，行数和扫描代价都最小化）：
--
--   (user_id, level_id) WHERE clears > 0               → stage 榜
--   (user_id, min_power_clear) WHERE min_power_clear > 0 → efficiency 榜
--
-- 两者首列都是 user_id：既覆盖 `GROUP BY user_id` 的有序分组，
-- 又能在 `MyRank` 中让 `WHERE user_id = $1` 直接命中索引。
--
-- # 代价
--
-- `level_stars` 每用户每关一行、通关后更新：两个部分索引只覆盖
-- 有成绩的行（clears>0 / min_power_clear>0），写入开销有限。
-- 用 `CREATE INDEX`（非 CONCURRENTLY）是刻意的：本仓库的迁移在单事务里跑。

-- +goose Up

CREATE INDEX idx_level_stars_stage
    ON level_stars (user_id, level_id)
    WHERE clears > 0;

CREATE INDEX idx_level_stars_efficiency
    ON level_stars (user_id, min_power_clear)
    WHERE min_power_clear > 0;

-- +goose Down

DROP INDEX IF EXISTS idx_level_stars_stage;
DROP INDEX IF EXISTS idx_level_stars_efficiency;