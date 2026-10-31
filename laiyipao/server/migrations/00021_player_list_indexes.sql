-- 00021_player_list_indexes.sql — 玩家「我的订单 / 兑换 / 购买记录」索引
--
-- # 为什么需要（2026-10-31 新一轮第 2 轮）
--
-- 三个玩家侧列表端点都按 user_id 过滤 + created_at DESC 排序取前 N 条，
-- 但底表索引要么没有 user_id 首列、要么排序键不匹配：
--
--   orders           只有 idx_orders_status (status, created_at DESC)，
--                    没有任何 user_id 索引 → MyOrders 每次全表扫描 + 排序
--   redeem_usages    只有 UNIQUE (code_id, user_id)，
--                    没有 user_id 索引 → MyRedemptions 每次全表扫描 + 排序
--   user_purchases   只有 idx_user_purchases_limit (user_id, item_id, purchase_date)，
--                    查询按 created_at DESC 排序 → 过滤能走最左前缀，
--                    但排序必须单独做
--
-- 三个端点都挂在 /me 下（订单 / 兑换 / 购买记录），玩家每次打开
-- 「我的」页都会拉。数据量小时全表扫描看不出来；订单与兑换是持续增长的
-- append-only 流水表，到十万行以上时每次打开都白扫全表。
--
-- # 列序为什么是 (user_id, created_at DESC)
--
-- 查询形态是 `WHERE user_id = $1 ORDER BY created_at DESC LIMIT 20`：
-- 复合索引首列 user_id 精确匹配，第二列 created_at 直接给出排序顺序，
-- LIMIT 20 时 PostgreSQL 走 Index Scan 顺序读前 20 条，完全避免排序。
--
-- # 代价
--
-- 三张表都是 append-only（只增不改），写入开销极小。用 CREATE INDEX
-- （非 CONCURRENTLY）是刻意的：本仓库的迁移在单事务里跑。

-- +goose Up

CREATE INDEX idx_orders_user
    ON orders (user_id, created_at DESC);

CREATE INDEX idx_redeem_usages_user
    ON redeem_usages (user_id, created_at DESC);

CREATE INDEX idx_user_purchases_user
    ON user_purchases (user_id, created_at DESC);

-- +goose Down

DROP INDEX IF EXISTS idx_orders_user;
DROP INDEX IF EXISTS idx_redeem_usages_user;
DROP INDEX IF EXISTS idx_user_purchases_user;