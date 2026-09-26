-- 00004_battle.sql — 战斗结算与防作弊（battle_token 防重放 + 回放哈希验真）

-- +goose Up

-- 开局申请的一次性凭证。结算必须匹配一条未使用、未过期的 token，
-- 从根本上杜绝"重放同一份战报反复领奖"。
CREATE TABLE battle_tokens (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_id   INTEGER     NOT NULL REFERENCES levels (id) ON DELETE CASCADE,
    seed       BIGINT      NOT NULL,                       -- 战斗随机种子，确定性的根
    issued_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at    TIMESTAMPTZ,
    revived    BOOLEAN     NOT NULL DEFAULT FALSE          -- 广告复活
);
CREATE INDEX idx_battle_tokens_user ON battle_tokens (user_id, issued_at DESC);

CREATE TABLE battle_records (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_id        INTEGER     NOT NULL,
    battle_token_id BIGINT      REFERENCES battle_tokens (id) ON DELETE SET NULL,
    result          TEXT        NOT NULL,                   -- win / lose
    stars           INTEGER     NOT NULL DEFAULT 0,
    score           BIGINT      NOT NULL DEFAULT 0,
    power_after     BIGINT      NOT NULL DEFAULT 0,
    kills           INTEGER     NOT NULL DEFAULT 0,
    leaked          INTEGER     NOT NULL DEFAULT 0,
    hp_left         INTEGER     NOT NULL DEFAULT 0,
    wave_reached    INTEGER     NOT NULL DEFAULT 0,
    duration_ms     INTEGER     NOT NULL DEFAULT 0,
    shots           INTEGER     NOT NULL DEFAULT 0,
    hits            INTEGER     NOT NULL DEFAULT 0,
    reactions       INTEGER     NOT NULL DEFAULT 0,         -- I-1 触发次数
    heat_max        INTEGER     NOT NULL DEFAULT 0,         -- I-2 峰值热量
    elements_used   JSONB       NOT NULL DEFAULT '{}',      -- {fire:12, ice:0,...}
    reactions_used  JSONB       NOT NULL DEFAULT '{}',      -- {steam_burst:3,...}
    terrain_used    JSONB       NOT NULL DEFAULT '[]',      -- I-4 地形触发
    build_snapshot  JSONB       NOT NULL DEFAULT '{}',
    replay_hash     TEXT        NOT NULL DEFAULT '',        -- I-6 FNV64，16 位十六进制
    loot            JSONB       NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_battle_records_user ON battle_records (user_id, created_at DESC);
CREATE INDEX idx_battle_records_level ON battle_records (level_id, created_at DESC);
CREATE INDEX idx_battle_records_hash ON battle_records (replay_hash);

CREATE TABLE level_stars (
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_id   INTEGER     NOT NULL,
    stars      INTEGER     NOT NULL DEFAULT 0,
    best_score BIGINT      NOT NULL DEFAULT 0,
    clears     INTEGER     NOT NULL DEFAULT 0,
    min_power_clear BIGINT NOT NULL DEFAULT 0,              -- L-2 效率榜：通关时最低战力
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, level_id)
);
CREATE INDEX idx_level_stars_minpower ON level_stars (level_id, min_power_clear)
    WHERE min_power_clear > 0;

-- I-6：任何人可用 battle_id + 同种子本地重放来证伪分数
CREATE TABLE replay_verifications (
    id            BIGSERIAL PRIMARY KEY,
    battle_id     BIGINT      NOT NULL REFERENCES battle_records (id) ON DELETE CASCADE,
    verifier_id   BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    expected_hash TEXT        NOT NULL,
    actual_hash   TEXT        NOT NULL,
    matched       BOOLEAN     NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_replay_verifications_battle ON replay_verifications (battle_id);
