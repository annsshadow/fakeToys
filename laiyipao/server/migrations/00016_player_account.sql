-- 00016: 玩家账号（用户名 + 密码）
--
-- ## 为什么要有这张迁移
--
-- 此前玩家侧只有两种身份：
--
--   - 游客（`users.guest_token`）：**每个浏览器/设备一个**，
--     凭证存在 localStorage，换设备就是另一个号；
--   - 微信（`users.openid`）：需要 WECHAT_APP_ID/SECRET，本项目未接通。
--
-- 游客通道对「一个人随便玩两下」是合适的，但对**测试环境**是灾难：
--
--   1. 测试数据散落在无数个游客号里 —— 金币、关卡进度、专精点
--      分散在不同 `users` 行，后台看板上是一堆孤立的号；
--   2. 换一个浏览器（或者清一次缓存）就是一个新号，
--      「我刚打的关卡呢」变成一个无法解释的 bug；
--   3. 多人同时测时，每人一个号，**互相看不见对方的进度**，
--      运营想复现「玩家卡在第 27 关」根本无从下手。
--
-- 所以需要「一个共享的、有账号密码的玩家身份」。
--
-- ## 为什么是加列而不是新建表
--
-- 玩家资产（钱包 / 进度 / 专精 / 战报 / 防线）全挂在 `users.id` 上。
-- 新建一张 `player_accounts` 表再 JOIN 一次，等于把「一个玩家」拆成两行，
-- 所有按 `users.id` 关联的查询都要多一次 JOIN —— 而这次改动**完全不需要**那种代价：
-- 游客号与账号只是同一行上的两种凭证。
--
-- ## 列为什么可空
--
-- `username` / `password_hash` 都允许 NULL：
--
--   - 存量游客号（绝大多数行）没有用户名，也没有密码 —— 它们不需要；
--   - 微信号同理。
--
-- NULL 的语义是「这个身份不能用用户名密码登录」，
-- 而不是「密码是空串」—— 后者会让 `password = ''` 变成一条可登录的口令。
--
-- ## 为什么用部分唯一索引而不是 UNIQUE 列
--
-- PostgreSQL 的 UNIQUE 列允许多个 NULL，但**部分唯一索引**让意图更直白：
-- 只有「有用户名」的行走这条约束。且它是 `IF NOT EXISTS` 的，
-- 迁移失败重跑不会因为「索引已存在」而二次失败。
--
-- ## 与 admin_users 的关系
--
-- **完全独立**。管理员账号在 `admin_users` 表，签发的 JWT 是 `kind=admin`；
-- 玩家账号在 `users` 表，签发的是 `kind=user`。
-- 两者的令牌类型不同、权限面不同，客户端也分别存在不同的地方。
--
-- 测试环境可以让两者**用同一套口令**（那是运维的选择，不是数据模型的事），
-- 但库里不共享任何一行。
--
-- ## 回滚
--
-- Down 只删这两列与索引。⚠️ **有损**：共享测试账号的行会失去用户名与密码，
-- 于是它退回成「一个再也登不上的游客号」（它的金币与进度还在）。
-- 回滚前应确认这个账号不再需要，或先把它的数据改名/导出。

-- +goose Up

ALTER TABLE users ADD COLUMN IF NOT EXISTS username TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username
    ON users (username) WHERE username IS NOT NULL;

-- +goose Down

DROP INDEX IF EXISTS idx_users_username;
ALTER TABLE users DROP COLUMN IF EXISTS password_hash;
ALTER TABLE users DROP COLUMN IF EXISTS username;
