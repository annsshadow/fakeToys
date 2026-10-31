-- 00022_my_rank_power_index.sql — `/me/rank` 战力榜索引
--
-- # 为什么需要（2026-10-31 新一轮第 9 轮）
--
-- `MyRank`（`/me/rank`）对 power 榜每次请求都执行：
--
--   COUNT(*) FROM user_progress                                    （总人数）
--   COUNT(*) FROM user_progress WHERE level_exp > me.score OR ...   （名次）
--
-- 而 `user_progress` 只有主键 `(user_id)`，没有 `level_exp` 索引：
-- 两条 COUNT 都是**全表顺序扫描**。玩家每开一次排行榜页都会触发，
-- 用户量到十万级时每次都要扫完整张表。
--
-- 本迁移加 `(level_exp, user_id)` 复合索引：
--   - `COUNT(*)` 可以走 **index-only scan**（不必回表）；
--   - 名次比较 `level_exp > X` 走索引范围扫描，
--     `level_exp = X AND user_id < $1` 走索引精确匹配 + 前缀。
--
-- # 代价
--
-- `user_progress` 是每用户一行、按用户更新的表；索引让 UPDATE 多维护
-- 一个二级索引。读远多于写（进度更新只在结算时），值得。
-- 用 `CREATE INDEX`（非 CONCURRENTLY）是刻意的：本仓库的迁移在单事务里跑。

-- +goose Up

CREATE INDEX idx_user_progress_power
    ON user_progress (level_exp, user_id);

-- +goose Down

DROP INDEX IF EXISTS idx_user_progress_power;