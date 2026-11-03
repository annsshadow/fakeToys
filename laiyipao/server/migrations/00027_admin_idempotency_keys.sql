-- 00027: 管理端写接口的幂等键（R86）
--
-- ## 解决什么（探针实证，idempotency_probe_test）
--
-- 「同一请求打两次」的改前后果：
--
--   POST /admin/users/:id/grant     余额 +200（钱，最危险）
--   POST /admin/announcements       公告 2 行
--   POST /admin/redeem-codes/batch  随机码 10 个
--
-- 客户端网络抖动重试、运营连点、脚本重跑都会真的执行两遍。
-- 已幂等的（不改）：指定 code 建码（code UNIQUE）、玩家兑换
-- （redeem_usages UNIQUE）、任务领奖（claimed_at）、结算（used_at 抢占）。
--
-- ## 语义
--
-- `Idempotency-Key` 请求头（白名单 8~128 字符）+ 端点 + 请求体 sha256：
-- 有效期内重放返回**第一次的响应**（同状态码同响应体），不再执行；
-- 同 key 换请求体 → 409 idempotency_conflict（不能拿旧结果糊弄新请求）；
-- 不带 key → 现状行为（老客户端零影响）。
--
-- # 为什么记录必须与业务写在**同一个事务**
--
-- 先写幂等表再执行业务会留下「已处理」的假记录：业务失败时事务回滚
-- 只回滚业务，幂等记录还在 —— 第二次重试反而被拒/重放假结果，
-- 这比重复执行更糟。同事务则失败整体回滚，重试干净重来。
--
-- ## 窗口与清理
--
-- 有效期 24 小时：客户端重试/运营连点都在分钟级；24h 覆盖「隔夜重跑
-- 脚本」，更长窗口收益递减而存储增长。清理是**写入时顺带**（每次
-- INSERT 后 DELETE 过期行一批）——管理端写操作低频，单表体积稳定，
-- 不需要后台任务/分区。
--
-- ## 并发
--
-- 并发同 key：PK (endpoint, key) 冲突时 `ON CONFLICT DO NOTHING`
-- 会**阻塞**到对方事务结束（PK 冲突的规范行为）——对方提交则本方
-- 回滚业务并重查对方响应重放；对方回滚则本方正常插入。
-- 不存在「两个都执行」的窗口。
--
-- ## 回滚
--
-- Down 直接 DROP：表只是重放缓存，删掉不影响任何业务行；
-- 重放在飞的请求退化为「可能重复执行」（R86 之前的行为）。

-- +goose Up

CREATE TABLE IF NOT EXISTS admin_idempotency_keys (
    endpoint     TEXT        NOT NULL,
    key          TEXT        NOT NULL,
    request_hash TEXT        NOT NULL,                -- 请求体 sha256（hex）
    status       INT         NOT NULL,
    -- ⚠️ response 必须是 TEXT 而不是 JSONB：幂等重放的契约是
    -- 「与第一次**逐字节**相同」，而 jsonb 会按自己的规范重排键序、
    -- 重写空白 —— 存进去的字节出来就不是同一份了（R86 实测：
    -- {"wallet":{"coin":...}} 变成 {"wallet": {"gem": ...}}）。
    response     TEXT        NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (endpoint, key)
);

-- +goose Down

DROP TABLE IF EXISTS admin_idempotency_keys;
