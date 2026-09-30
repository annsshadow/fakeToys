-- 00007_admin.sql — 管理员与审计

-- +goose Up

CREATE TABLE admin_users (
    id            BIGSERIAL PRIMARY KEY,
    username      TEXT        NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL,                     -- bcrypt
    role          TEXT        NOT NULL DEFAULT 'ops',       -- admin/ops/readonly
    status        SMALLINT    NOT NULL DEFAULT 1,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at TIMESTAMPTZ
);

CREATE TABLE admin_tokens (
    id         BIGSERIAL PRIMARY KEY,
    admin_id   BIGINT      NOT NULL REFERENCES admin_users (id) ON DELETE CASCADE,
    token_hash TEXT        NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 所有后台写操作留痕，不可删除
CREATE TABLE admin_audit_logs (
    id         BIGSERIAL PRIMARY KEY,
    admin_id   BIGINT REFERENCES admin_users (id) ON DELETE SET NULL,
    username   TEXT        NOT NULL DEFAULT '',
    action     TEXT        NOT NULL,
    target     TEXT        NOT NULL DEFAULT '',
    detail     JSONB       NOT NULL DEFAULT '{}',
    ip         TEXT        NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_audit_created ON admin_audit_logs (created_at DESC);
CREATE INDEX idx_audit_action ON admin_audit_logs (action, created_at DESC);
