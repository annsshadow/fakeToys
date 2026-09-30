-- 00001_core.sql — 账号、鉴权、钱包
-- 来一炮 · PostgreSQL 15+

-- +goose Up

CREATE TABLE users (
    id            BIGSERIAL PRIMARY KEY,
    guest_token   TEXT        NOT NULL UNIQUE,          -- 游客设备凭证（微信登录后保留，用于数据合并）
    openid        TEXT        UNIQUE,                   -- 微信 code2session 后回填
    unionid       TEXT,
    nickname      TEXT        NOT NULL DEFAULT '',
    avatar_url    TEXT        NOT NULL DEFAULT '',
    is_guest      BOOLEAN     NOT NULL DEFAULT TRUE,
    status        SMALLINT    NOT NULL DEFAULT 1,       -- 1 正常 / 2 封禁
    ban_reason    TEXT        NOT NULL DEFAULT '',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_users_openid ON users (openid) WHERE openid IS NOT NULL;

CREATE TABLE refresh_tokens (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash TEXT        NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_refresh_tokens_user ON refresh_tokens (user_id);

-- 四种货币，各只有一个用途（见 GAME_DESIGN I-3）：coin 基础升级 / gem 专精洗练 / energy 进关 / keys 门票
CREATE TABLE user_wallets (
    user_id            BIGINT PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    coin               BIGINT      NOT NULL DEFAULT 0,
    gem                INTEGER     NOT NULL DEFAULT 0,
    energy             INTEGER     NOT NULL DEFAULT 0,
    keys               INTEGER     NOT NULL DEFAULT 0,
    energy_updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chk_wallet_nonneg CHECK (coin >= 0 AND gem >= 0 AND energy >= 0 AND keys >= 0)
);

-- 游戏内配置表版本控制：客户端按 version 拉取增量
CREATE TABLE game_configs (
    key        TEXT PRIMARY KEY,
    version    INTEGER     NOT NULL DEFAULT 1,
    value      JSONB       NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
