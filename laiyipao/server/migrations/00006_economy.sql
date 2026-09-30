-- 00006_economy.sql — 货币流水、签到、任务、商店、订单、广告激励、兑换码、公告、离线收益

-- +goose Up

-- 每一笔货币变动都留痕，后台经济看板直接聚合这张表
CREATE TABLE wallet_flows (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    currency    TEXT        NOT NULL,                       -- coin/gem/energy/keys
    delta       BIGINT      NOT NULL,
    balance     BIGINT      NOT NULL,
    reason      TEXT        NOT NULL,                       -- battle_reward/upgrade/signin/shop/...
    ref_id      BIGINT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_wallet_flows_user ON wallet_flows (user_id, created_at DESC);
CREATE INDEX idx_wallet_flows_reason ON wallet_flows (reason, created_at DESC);

CREATE TABLE sign_in_calendar (
    day_index INTEGER PRIMARY KEY,                          -- 1..7，对应新手七日
    reward    JSONB   NOT NULL
);

CREATE TABLE user_sign_ins (
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    sign_date  DATE        NOT NULL,
    day_index  INTEGER     NOT NULL,
    reward     JSONB       NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, sign_date)
);

CREATE TABLE tasks (
    id      INTEGER PRIMARY KEY,
    code    TEXT    NOT NULL UNIQUE,
    name    TEXT    NOT NULL,
    scope   TEXT    NOT NULL,                               -- daily/weekly/achievement
    target  INTEGER NOT NULL,
    metric  TEXT    NOT NULL,                               -- kills/clears/wins/reactions/signin
    reward  JSONB   NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE user_tasks (
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    task_id    INTEGER     NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    task_date  DATE        NOT NULL,                        -- daily/weekly 用周期起点日期
    progress   INTEGER     NOT NULL DEFAULT 0,
    claimed_at TIMESTAMPTZ,
    PRIMARY KEY (user_id, task_id, task_date)
);
CREATE INDEX idx_user_tasks_date ON user_tasks (user_id, task_date);

CREATE TABLE shop_items (
    id             INTEGER PRIMARY KEY,
    code           TEXT    NOT NULL UNIQUE,
    name           TEXT    NOT NULL,
    category       TEXT    NOT NULL,                        -- currency/boost/keys/starter
    price          JSONB   NOT NULL,                        -- {coin|gem: amount}
    payload        JSONB   NOT NULL,                        -- 发放内容（grant 是 PG 保留字，故用 payload）
    limit_per_day  INTEGER NOT NULL DEFAULT 0,
    sort_order     INTEGER NOT NULL DEFAULT 0,
    enabled        BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE user_purchases (
    id             BIGSERIAL PRIMARY KEY,
    user_id        BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    item_id        INTEGER NOT NULL REFERENCES shop_items (id) ON DELETE CASCADE,
    purchase_date  DATE   NOT NULL,
    price          JSONB  NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_user_purchases_limit ON user_purchases (user_id, item_id, purchase_date);

-- 支付订单：真实支付需要商户号，当前走 mock 回调，协议与状态机保持一致，接凭证即生效
CREATE TABLE orders (
    id           BIGSERIAL PRIMARY KEY,
    user_id      BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    order_no     TEXT        NOT NULL UNIQUE,
    sku          TEXT        NOT NULL,
    amount       INTEGER     NOT NULL,                       -- 分
    status       TEXT        NOT NULL DEFAULT 'pending',    -- pending/paid/delivered/refunded
    provider     TEXT        NOT NULL DEFAULT 'mock',       -- mock/wechat
    provider_txn TEXT        NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    delivered_at TIMESTAMPTZ
);
CREATE INDEX idx_orders_status ON orders (status, created_at DESC);

-- 激励广告（复活 / 双倍奖励）：真实接入需流量主广告位 ID，当前 mock
CREATE TABLE ad_rewards (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    kind       TEXT        NOT NULL,                        -- revive/double
    battle_id  BIGINT,
    reward     JSONB       NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_ad_rewards_daily ON ad_rewards (user_id, created_at DESC);

CREATE TABLE redeem_codes (
    id         INTEGER PRIMARY KEY,
    code       TEXT    NOT NULL UNIQUE,
    reward     JSONB   NOT NULL,
    max_uses   INTEGER NOT NULL DEFAULT 1,
    used_count INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMPTZ,
    enabled    BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chk_redeem_uses CHECK (used_count <= max_uses)
);

CREATE TABLE redeem_usages (
    id         BIGSERIAL PRIMARY KEY,
    code_id    INTEGER NOT NULL REFERENCES redeem_codes (id) ON DELETE CASCADE,
    user_id    BIGINT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (code_id, user_id)
);

CREATE TABLE announcements (
    id           BIGSERIAL PRIMARY KEY,
    title        TEXT        NOT NULL,
    body         TEXT        NOT NULL,
    published    BOOLEAN     NOT NULL DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 离线收益：按离线时长结算，领取时清零
CREATE TABLE offline_rewards (
    user_id        BIGINT PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    last_claim_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
