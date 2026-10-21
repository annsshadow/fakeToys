-- 00018_verification_verifier_kind.sql — 修 00017 的唯一索引把两个 ID 空间合并的问题
--
-- # 00017 错在哪
--
-- 00017 建的是：
--
--   UNIQUE (battle_id, COALESCE(verifier_id, admin_verifier_id), actual_hash)
--
-- 而 `verifier_id` 是 `users(id)` 的外键、`admin_verifier_id` 是
-- `admin_users(id)` 的外键 —— **两个完全独立的 ID 空间，各自从 1 开始**。
--
-- `COALESCE` 把这两个空间**折叠成一个数**，于是：
--
--   玩家 users.id=1 对战报 #42 验真，哈希 "0123…"  → 行 (42, 1, NULL, "0123…")
--   运营 admin_users.id=1 对**同一场**战报验真，**同一个哈希** → 行 (42, NULL, 1, "0123…")
--
-- 索引值都是 (42, 1, "0123…") → **冲突** → `DO NOTHING` →
-- **运营那一行被静默丢弃**，而接口照样返回 `matched: true`。
--
-- 后果全是静默的：
--   - 运营以为「已记录」，`replay_verifications` 里却没有他的记录；
--   - `AdminListUsers` / `AdminListBattles` 的验真计数**按战报主人聚合**
--     → 运营的验真被算到**玩家**头上 → 玩家的「可疑度」被虚高。
--
-- # 修法：两个**部分**唯一索引，各自只覆盖自己那类验真者
--
--   UNIQUE (battle_id, verifier_id,      actual_hash) WHERE admin_verifier_id IS NULL
--   UNIQUE (battle_id, admin_verifier_id, actual_hash) WHERE verifier_id      IS NULL
--
-- migration 00011 的 CHECK 约束保证**恰好一个非空**，
-- 所以两个部分索引的谓词是精确的、且互不重叠。
--
-- ⚠️ 部分索引要求 INSERT 的 `ON CONFLICT` 带上**能蕴含该谓词的 WHERE**
-- —— 见 `verification.go` 的 `recordVerification`（按 isAdmin 分两条 SQL）。
--
-- +goose Up

DROP INDEX IF EXISTS idx_replay_verifications_unique;

-- 玩家验真
CREATE UNIQUE INDEX IF NOT EXISTS idx_replay_verifications_user_unique
    ON replay_verifications (battle_id, verifier_id, actual_hash)
    WHERE admin_verifier_id IS NULL;

-- 运营验真
CREATE UNIQUE INDEX IF NOT EXISTS idx_replay_verifications_admin_unique
    ON replay_verifications (battle_id, admin_verifier_id, actual_hash)
    WHERE verifier_id IS NULL;

-- +goose Down

DROP INDEX IF EXISTS idx_replay_verifications_user_unique;
DROP INDEX IF EXISTS idx_replay_verifications_admin_unique;

CREATE UNIQUE INDEX IF NOT EXISTS idx_replay_verifications_unique
    ON replay_verifications (battle_id, COALESCE(verifier_id, admin_verifier_id), actual_hash);
