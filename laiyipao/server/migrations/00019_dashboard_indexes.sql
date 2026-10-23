-- 00019_dashboard_indexes.sql — 看板按时间范围扫描所需的索引
--
-- # 为什么需要（2026-10-23）
--
-- 看板（`AdminDashboard`）有四处按 `created_at` 做**范围扫描**：
--
--   users            WHERE created_at >= CURRENT_DATE          （今日新增）
--   battle_records   WHERE created_at >= CURRENT_DATE          （今日战报）
--   battle_records   LEFT JOIN ... ON br.created_at >= d AND br.created_at < d+1day  （14 日 DAU）
--   wallet_flows     WHERE created_at >= ...                   （货币净流入）
--
-- 而现有索引的**首列都不是 `created_at`**：
--
--   idx_battle_records_user   (user_id, created_at DESC)
--   idx_battle_records_level  (level_id, created_at DESC)
--   idx_wallet_flows_user     (user_id, created_at DESC)
--   idx_wallet_flows_reason   (reason, created_at DESC)
--
-- 复合索引只有「最左前缀」可用于范围条件，所以上面四条范围扫描
-- **一条都走不到索引** —— 全是顺序扫描。数据量小时看不出来，
-- 战报到百万级时看板会从"秒开"变成"转圈"。
--
-- # 为什么 battle_records 用两列 (created_at, user_id)
--
-- 14 日 DAU 是 `COUNT(DISTINCT user_id)`，而 `user_id` 就在索引里时
-- PostgreSQL 可以走 **index-only scan**（不必回表）。单列索引则必须
-- 回表取 user_id，代价是每天桶都要随机读一遍。
--
-- # 为什么 wallet_flows 用单列
--
-- 它的用途是 `SUM(delta) FILTER (WHERE currency = ...)` 按时间窗聚合，
-- 取的是 `delta` 而不是 `user_id`，两列索引的收益有限，单列足够。
--
-- # 代价
--
-- 两个索引会让写入多一点开销（战报与流水都是 append-only，写入量远小于读）。
-- 用 `CREATE INDEX`（非 CONCURRENTLY）是刻意的：本仓库的迁移在单事务里跑，
-- `CONCURRENTLY` 不能进事务块。数据量到需要 CONCURRENTLY 时，
-- 应改为在发布窗口手工建索引，而不是把它塞进迁移。

-- +goose Up

CREATE INDEX IF NOT EXISTS idx_battle_records_created
    ON battle_records (created_at, user_id);

CREATE INDEX IF NOT EXISTS idx_wallet_flows_created
    ON wallet_flows (created_at);

-- +goose Down
--
-- 下行只删索引，不动数据 —— 索引是纯附加结构，删掉不影响任何行的语义，
-- 只是看板那几段范围扫描会退回顺序扫描。
--
-- ⚠️ 本子项目此前**没有**任何 `_rollback.sql` 文件（那是 oa4rust 的惯例），
-- 下行统一走 goose 的 Down 段（`go run ./cmd/migrate -down`）。
DROP INDEX IF EXISTS idx_wallet_flows_created;
DROP INDEX IF EXISTS idx_battle_records_created;
