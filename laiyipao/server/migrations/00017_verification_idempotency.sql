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
-- ⚠️ 唯一索引是**部分索引**：`verifier_id` 与 `admin_verifier_id`
-- 恰好一个非空（migration 00011 的 CHECK 保证），
-- 所以索引建在 `COALESCE(verifier_id, admin_verifier_id)` 上 ——
-- NULL 参与唯一索引比较时「都算相等」，
-- 直接建 `(battle_id, verifier_id, admin_verifier_id, actual_hash)`
-- 会让**所有 admin 验真**互相冲突（admin_verifier_id 相同、
-- verifier_id 都是 NULL）。
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
