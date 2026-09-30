-- 00003_progression.sql — 养成：技能、插槽、装备、宝石、皮肤、专精

-- +goose Up

CREATE TABLE user_progress (
    user_id        BIGINT PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    max_stage      INTEGER     NOT NULL DEFAULT 0,
    level_exp      BIGINT      NOT NULL DEFAULT 0,
    mastery_points INTEGER     NOT NULL DEFAULT 3,          -- 专精点有限：初始 3，每 10 级 +1
    total_battles  BIGINT      NOT NULL DEFAULT 0,
    total_kills    BIGINT      NOT NULL DEFAULT 0,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE user_skills (
    user_id  BIGINT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skills (id) ON DELETE CASCADE,
    level    INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (user_id, skill_id)
);

-- 0..3 主动槽，4 被动槽（I-2）
CREATE TABLE user_skill_slots (
    user_id  BIGINT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    slot     INTEGER NOT NULL,
    skill_id INTEGER NOT NULL REFERENCES skills (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, slot)
);

CREATE TABLE user_equipment (
    user_id      BIGINT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    equipment_id INTEGER NOT NULL REFERENCES equipment (id) ON DELETE CASCADE,
    slot         TEXT    NOT NULL,
    equipped     BOOLEAN NOT NULL DEFAULT FALSE,
    level        INTEGER NOT NULL DEFAULT 1,
    star        INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (user_id, equipment_id)
);
CREATE INDEX idx_user_equipment_equipped ON user_equipment (user_id, equipped) WHERE equipped;

CREATE TABLE user_gems (
    id           BIGSERIAL PRIMARY KEY,
    user_id      BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    gem_id       INTEGER     NOT NULL REFERENCES gems (id) ON DELETE CASCADE,
    quality      TEXT        NOT NULL,
    affixes      JSONB       NOT NULL DEFAULT '[]',       -- 洗练结果 [{affix,value}]
    equipped_slot TEXT       NOT NULL DEFAULT '',
    acquired_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_user_gems_user ON user_gems (user_id);

CREATE TABLE user_skins (
    user_id      BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    skin_id      INTEGER     NOT NULL REFERENCES skins (id) ON DELETE CASCADE,
    equipped     BOOLEAN     NOT NULL DEFAULT FALSE,
    acquired_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, skin_id)
);

-- I-3 专精树：每层 4 选 2，点数有限
CREATE TABLE user_mastery_nodes (
    user_id  BIGINT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    node_id  INTEGER NOT NULL REFERENCES mastery_nodes (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, node_id)
);

-- L-3 阵型预设
CREATE TABLE user_presets (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name       TEXT        NOT NULL,
    slots      JSONB       NOT NULL,                        -- {skills:[], equipment:[], gems:[], works:[]}
    is_current BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX idx_user_presets_current ON user_presets (user_id) WHERE is_current;
