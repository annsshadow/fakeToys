-- 00024: 登出轮换游客凭证的「宽限期别名」
--
-- ## 为什么需要这张表（R38 / B2）
--
-- `Logout` 会把 `users.guest_token` 换成一枚新随机串（旧的那枚立即失效），
-- 新凭证**只能**通过响应体交回客户端。这件事在客户端已经尽力：
-- 成功响应时先存新凭证再清 access/refresh。
--
-- 但「响应丢失」是服务端无法排除的情况：请求已经提交、玩家却只看到
-- 网络失败。此时客户端按既有约定保留旧 `guest_token` 并清掉本地令牌，
-- 于是下一次冷启动会带着**已作废**的旧凭证去 `GuestLogin` ——
-- 服务端按「未知凭证 → 建新号」的语义，给玩家造出一个**空账号**，
-- 原账号的金币/关卡/专精全部找不回来。
--
-- 用玩家视角描述就是「既不能用新（没收到）也不能用旧（已失效）」，
-- 与 R33 的刷新令牌轮换同族：一次瞬时故障把用户锁在门外。
--
-- ## 这张表怎么用
--
-- 登出时把**旧凭证**写进来，映射到仍属于该玩家的 `user_id`，
-- 并带一个短期 `expires_at`；`GuestLogin` 在正常按凭证匹配之前先查这里，
-- 命中就把账号原样找回，并把**当前**（轮换后的）`guest_token` 一起返回，
-- 让客户端把本地那枚旧凭证换成有效的。
--
-- ## 为什么是独立表而不是 users 上的列
--
-- 这是纯短期的「找回凭据」，不是玩家的持久属性：过期即失去意义，
-- 且同一个玩家可能连续登出多次（需要保留多条历史旧凭证）。
-- 独立表还可以直接按 `expires_at` 清理，不给 `users` 行增加写放大。
--
-- ## 宽限期为什么短
--
-- 旧凭证在宽限期内**仍然可用**，这是对「登出立即切断长期凭证」的
-- 有意让步：宽限越长，被盗凭证在玩家登出后的可用窗口越长。
-- 15 分钟足够覆盖「响应丢失 → 重试 / 冷启动」的正常恢复路径，
-- 又远短于 refresh token 的 30 天有效期。
--
-- ## 回滚
--
-- Down 直接删表。⚠️ 有损但可接受：正在宽限期内的玩家会失去这次找回机会，
-- 下次冷启动仍可能被造新号 —— 即回到本迁移之前的旧行为。

-- +goose Up

CREATE TABLE IF NOT EXISTS guest_token_aliases (
    token      TEXT PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 登出时顺手清理过期行，以及排查时的按时间扫描，都走这个索引。
CREATE INDEX IF NOT EXISTS idx_guest_token_aliases_expires
    ON guest_token_aliases (expires_at);

-- +goose Down

DROP TABLE IF EXISTS guest_token_aliases;
