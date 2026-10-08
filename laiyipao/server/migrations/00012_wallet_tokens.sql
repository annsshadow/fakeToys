-- 00012_wallet_tokens.sql —— 把「令牌类货币」从 user_wallets 里拆出来（第 74 轮）

-- +goose Up

-- # 为什么需要这张表
--
-- `Buy` 用 `grantWallet` 发货，而 `grantWallet` 走的是 `walletColumns` 白名单：
--
--     var walletColumns = map[string]string{
--         "coin": "coin", "gem": "gem", "energy": "energy", "keys": "keys",
--     }
--
-- 遇到不在表里的 key 就返回 `ErrBadInput: 未知货币`。
--
-- 而种子数据里有三个商品的 payload **不在表里**：
--
--     shop_retry         -> {"revive_token": 1}
--     shop_mastery_reset -> {"mastery_reset": 1}
--     shop_gem_wash      -> {"gem_wash_token": 1}
--
-- 结果：**商城 8 件商品里有 3 件永远买不了**，
-- 客户端只看到一句「未知货币 "revive_token"」的 400。
--
-- # 为什么不用 ALTER TABLE 加三列
--
-- 1. `walletColumns` 的形状是「key -> 列名」，加列就得同步改 Wallet 结构体、
--    LoadWallet 的查询、客户端的展示面 —— 而这些 token
--    **目前没有任何消费方**（没有「用掉复活币」的端点）。
--    为一个还没有消费方的数值铺一整条读取链路，是投机性设计。
--
-- 2. 真正的语义差别在于：**token 不是货币，是道具**。
--    货币有单价、有流水分析、有通胀控制；道具只有「有没有」和「几个」。
--    把它们塞进同一张表会让 `SUM(coin)` 之类的聚合悄悄把它们算进去。
--
-- 3. 将来再加工具（比如「双倍体力券」）时，这张表不用改迁移。
--
-- # 为什么不建一张只有三列的固定表
--
-- 主键是 `(user_id, token)`，加 token **不需要迁移**。
-- 同时 `grantWallet` 侧有一份**显式白名单** `walletTokens` ——
-- 未知 key 仍然报「未知货币」，所以打错字（比如 `{"cino": 1}`）
-- 不会被静默地变成一个叫 `cino` 的新道具。
--
-- 白名单是刻意的摩擦：新增一种道具必须先在代码里显式声明，
-- 顺便回答「这个道具被谁消耗」这个问题。

CREATE TABLE user_tokens (
    user_id    BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token      TEXT   NOT NULL,
    balance    BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, token),
    CONSTRAINT chk_user_tokens_nonneg CHECK (balance >= 0)
);

CREATE INDEX idx_user_tokens_token ON user_tokens (token);

COMMENT ON TABLE user_tokens IS
    '道具类余额（非货币）。walletColumns 管货币、walletTokens 管道具，两者互不重叠。';

-- +goose Down
DROP TABLE IF EXISTS user_tokens;