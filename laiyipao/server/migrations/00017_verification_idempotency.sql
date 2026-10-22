-- 00017_verification_idempotency.sql — 验真幂等 + 装备获取途径的落地
--
-- # 为什么需要（2026-10-21，P3-5）
--
-- `replay_verifications` 此前**既无唯一约束也无去重**：
-- 同一场战报重复验真 200 次 → 写进 200 行。
--
-- 后果不是「表大了点」：
--
--  1. **写放大**。身份限流 20/分钟只是缓解，不是幂等 ——
--     一个脚本就能把表撑大。
--  2. **统计口径被污染**。后台的「验真一致率」是
--     `checked` / `mismatched` 两个计数；同一场对局验 20 次，
--     分母就多 19 —— 一致率被同一件事重复计数。
--  3. **「验了没有」这个事实被淹没**。真正的结论是
--     「这一局的哈希对不上」，而列表里躺着 200 条几乎一样的记录。
--
-- # 幂等的定义
--
-- 同一场战报 + 同一个验真者 + 同一个 actual_hash → **只算一条**。
--
-- 「同一个 actual_hash」是关键：如果验真者**换了**一个哈希提交，
-- 那是一条**新信息**（可能推翻了之前的结论），必须留下。
--
-- ⚠️⚠️ 2026-10-21（00018 修订）本段原注释有**两处**与事实不符，
-- 而其中一处正是后来那个 P1 缺陷的根源。**请以本段为准**：
--
--   1. 原注释写「唯一索引是**部分索引**」—— 它**没有** WHERE 子句，
--      不是部分索引。
--   2. 原注释写「NULL 参与唯一索引比较时『都算相等』」—— **这是错的**。
--      PostgreSQL 的唯一索引默认按 NULLS DISTINCT 处理：
--      多个 NULL **互不相等**。
--
-- 真正的问题不是 NULL，而是 **COALESCE 本身**：
-- `verifier_id` 指向 `users(id)`、`admin_verifier_id` 指向
-- `admin_users(id)` —— **两个完全独立的 ID 空间，各自从 1 开始**。
-- COALESCE 把它们折叠成同一个数，于是
--
--   玩家 users.id=1        对战报 #42 验真，哈希 "0123…" → (42, 1, NULL, "0123…")
--   运营 admin_users.id=1  对**同一场**战报验真，**同一个哈希** → (42, NULL, 1, "0123…")
--
-- 索引值相同 → 冲突 → `DO NOTHING` → **运营那一行被静默丢弃**，
-- 而接口照样返回 `matched: true`。
--
-- 00018 把它拆成两个**部分**唯一索引，各管自己那类验真者。
-- 本文件（00017）已发布，**DDL 不再改动**（已上线的库不会重跑它）；
-- 这里只把注释订正成事实，并把订正后的去重语句放进 00018。
--
-- # 迁移的幂等性
--
-- 建索引前先删重复行：保留**最早**的那条（同一次验真的第一条），
-- 其余删掉。保留最早而不是最晚，是因为 `created_at` 是事实时间。
--
-- +goose Up

-- 先删重复行（保留最早的一条）。
--
-- ⚠️ 用 `ctid` 而不是 `id`：`replay_verifications` 没有主键
-- （migration 00004 建表时就没有），只有 `battle_id` 索引。
-- `ctid` 是 PostgreSQL 的行标识，用来在同一次 DELETE 里引用自己。
DELETE FROM replay_verifications v
 WHERE v.ctid IN (
    SELECT ctid FROM (
        SELECT ctid,
               ROW_NUMBER() OVER (
                   PARTITION BY battle_id,
                                COALESCE(verifier_id, admin_verifier_id),
                                actual_hash
                   ORDER BY created_at, ctid
               ) AS rn
          FROM replay_verifications
    ) t
    WHERE t.rn > 1
 );

-- 幂等唯一索引（部分索引：只约束「恰好一个验真者非空」的行）。
CREATE UNIQUE INDEX IF NOT EXISTS idx_replay_verifications_unique
    ON replay_verifications (battle_id, COALESCE(verifier_id, admin_verifier_id), actual_hash);

-- +goose Down
DROP INDEX IF EXISTS idx_replay_verifications_unique;
