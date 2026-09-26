-- 00005_defense.sql — I-5 防线值守（异步对抗）
-- 服务端只存快照与结算，不跑战斗引擎：挑战在客户端用同款引擎模拟后回传哈希。

-- +goose Up

CREATE TABLE defenses (
    id                BIGSERIAL PRIMARY KEY,
    owner_id          BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name              TEXT        NOT NULL DEFAULT '我的防线',
    power             BIGINT      NOT NULL DEFAULT 0,
    element_coverage  INTEGER     NOT NULL DEFAULT 0,      -- 5 元素覆盖数，I-3 搭配质量
    mastery_done      INTEGER     NOT NULL DEFAULT 0,
    snapshot          JSONB       NOT NULL,                 -- {skills,equipment,gems,works,mastery}
    snapshot_hash     TEXT        NOT NULL,
    wins              INTEGER     NOT NULL DEFAULT 0,
    losses            INTEGER     NOT NULL DEFAULT 0,
    shielded_until    TIMESTAMPTZ,                          -- 24h 护盾，防被偷
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_defenses_power ON defenses (power DESC);
-- 一人一条防线：挑战对象是"玩家当前构筑"，历史快照不保留
CREATE UNIQUE INDEX idx_defenses_owner ON defenses (owner_id);

CREATE TABLE defense_challenges (
    id             BIGSERIAL PRIMARY KEY,
    defense_id     BIGINT      NOT NULL REFERENCES defenses (id) ON DELETE CASCADE,
    challenger_id  BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    seed           BIGINT      NOT NULL,
    won            BOOLEAN     NOT NULL,
    duration_ms    INTEGER     NOT NULL DEFAULT 0,
    hp_left_pct    INTEGER     NOT NULL DEFAULT 0,         -- 挑战者剩余防线血量百分比
    replay_hash    TEXT        NOT NULL DEFAULT '',
    loot           JSONB       NOT NULL DEFAULT '{}',
    settled        BOOLEAN     NOT NULL DEFAULT FALSE,      -- 幂等：只有一条能 settled
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (defense_id, challenger_id, created_at)
);
CREATE INDEX idx_defense_challenges_defense ON defense_challenges (defense_id, created_at DESC);

CREATE TABLE user_daily_challenges (
    user_id         BIGINT     NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    challenge_date  DATE       NOT NULL,
    attempts        INTEGER    NOT NULL DEFAULT 0,          -- 每日 3 次挑战机会
    stolen_times    INTEGER    NOT NULL DEFAULT 0,          -- 每日最多被偷 2 次
    PRIMARY KEY (user_id, challenge_date)
);

-- 防线三工事
CREATE TABLE defense_works (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    code       TEXT    NOT NULL,                            -- slow_belt / block_wall / tesla_grid
    level      INTEGER NOT NULL DEFAULT 1,
    slot       INTEGER NOT NULL                             -- 0..2
);
CREATE UNIQUE INDEX idx_defense_works_slot ON defense_works (user_id, slot);
