-- 00011: 让运营也能发起验真
--
-- ## 背景
--
-- 验真机制本身是**真的**（不是循环论证）：客户端用
-- `replay as runReplay` 重跑引擎算出 `replay_hash`，
-- 服务端比对其与结算时记录的是否一致。
--
-- 但它此前**只有玩家侧入口** `POST /api/v1/battle/verify`，
-- 而这一条链路有个结构性后果：
--
--   **能采取行动的人（运营），恰恰是唯一不能发起验真的人。**
--
-- 于是 `replay_verifications` 里只有玩家自己提交的比对，
-- 一次「不匹配」被记录、被统计（看板的「验真一致率」），
-- 却**不撤销奖励、不标记用户**，运营也查不出是哪几场
-- —— 前两轮已把后两件事补上（用户列表 / 战报列表的验真列）。
--
-- 剩下这一段：**没人能主动去验**。
-- 缺了它，「一致率」这个数字的分母永远只由玩家的自愿行为决定。
--
-- ## 为什么需要改表
--
-- `replay_verifications.verifier_id` 是 `BIGINT NOT NULL REFERENCES users(id)`，
-- 而运营在 `admin_users` 表里 —— 两条外键指向不同的表，
-- 所以运营的验真记录**根本插不进去**。
--
-- 修法：`verifier_id` 改为可空，新增 `admin_verifier_id`，
-- 并加 CHECK 保证「两个验真人字段恰好有一个非空」。
--
-- ## 为什么是 CHECK 而不是「都允许为空」
--
-- 两个都允许为空的话，就出现「这条验真记录没有验真人」的行。
-- 那会让「谁验的」这个问题**静默无解** ——
-- 而这一列的全部价值就是回答「谁验的」。
--
-- ## 存量数据
--
-- 已有行全部 `verifier_id` 非空、`admin_verifier_id` 为空，
-- **恰好满足**新 CHECK，所以本迁移**不需要**回填任何数据。
--
-- ## 回滚
--
-- Down 里把 `admin_verifier_id` 上的记录删掉（它们在旧结构下无处安放），
-- 再恢复 NOT NULL。这一步**有损**：运营发起的验真记录会消失。
-- 但保留它们会导致旧代码读到 NULL verifier_id 而行为不确定，
-- 所以选择显式删除并在此说明。

-- +goose Up

ALTER TABLE replay_verifications
  ALTER COLUMN verifier_id DROP NOT NULL;

ALTER TABLE replay_verifications
  ADD COLUMN admin_verifier_id BIGINT REFERENCES admin_users (id) ON DELETE SET NULL;

-- 恰好一个验真人。num_nonnulls() 是 PostgreSQL 9.6+ 内置的，
-- 不引入新依赖。
ALTER TABLE replay_verifications
  ADD CONSTRAINT replay_verifications_one_verifier
  CHECK (num_nonnulls(verifier_id, admin_verifier_id) = 1);

-- 运营查询自己发起的验真
CREATE INDEX idx_replay_verifications_admin
  ON replay_verifications (admin_verifier_id, created_at DESC)
  WHERE admin_verifier_id IS NOT NULL;

-- +goose Down

-- 有损：先删掉运营发起的验真记录（它们在旧结构下无处安放）
DELETE FROM replay_verifications WHERE admin_verifier_id IS NOT NULL;

DROP INDEX IF EXISTS idx_replay_verifications_admin;

ALTER TABLE replay_verifications
  DROP CONSTRAINT IF EXISTS replay_verifications_one_verifier;

ALTER TABLE replay_verifications
  DROP COLUMN IF EXISTS admin_verifier_id;

ALTER TABLE replay_verifications
  ALTER COLUMN verifier_id SET NOT NULL;
