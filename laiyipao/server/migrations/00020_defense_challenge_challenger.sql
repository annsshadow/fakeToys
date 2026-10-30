-- 00020_defense_challenge_challenger.sql — 玩家「我的挑战记录」索引
--
-- # 为什么需要（2026-10-30 实质优化第 18 轮）
--
-- `defense_challenges` 此前只有 `(defense_id, created_at DESC)` 索引：
-- 防守方视角（`GET /admin/defenses` 按防线查挑战）有索引可用，
-- 但**挑战者视角**（`GET /me/challenges` 按 challenger_id 查自己的
-- 挑战历史）没有任何索引 —— 玩家多了之后就是全表扫描。
--
-- 新端点按 `challenger_id` 过滤 + `created_at DESC` 排序，
-- 两列复合索引正好覆盖（最左前缀 challenger_id 也单独可用）。
--
-- # 代价
--
-- 每次挑战落一行（append-only，写入量远小于读）。用 `CREATE INDEX`
-- （非 CONCURRENTLY）是刻意的：本仓库的迁移在单事务里跑。

-- +goose Up

CREATE INDEX idx_defense_challenges_challenger
    ON defense_challenges (challenger_id, created_at DESC);

-- +goose Down

DROP INDEX IF EXISTS idx_defense_challenges_challenger;