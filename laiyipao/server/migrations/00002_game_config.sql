-- 00002_game_config.sql — 关卡、敌人、元素抗性、技能、合成、装备、宝石、皮肤、专精树

-- 100 关由种子化生成器产出（章节参数 + 难度曲线 + seed），生成后落库为可逐关手改的配置行
-- +goose Up

CREATE TABLE levels (
    id            INTEGER PRIMARY KEY,                   -- 1..100
    chapter       INTEGER     NOT NULL,
    name          TEXT        NOT NULL,
    seed          BIGINT      NOT NULL,
    base_hp       BIGINT      NOT NULL,                   -- 防线血量，不随关卡回复
    wave_count    INTEGER     NOT NULL,
    difficulty    INTEGER     NOT NULL,                   -- 难度系数（千分比，定点数）
    energy_cost   INTEGER     NOT NULL DEFAULT 5,
    star_targets  JSONB       NOT NULL,                   -- 三档星星门槛 [wave, score]
    terrain_config JSONB     NOT NULL DEFAULT '[]',       -- 见 terrain_config
    is_boss       BOOLEAN     NOT NULL DEFAULT FALSE,
    enabled       BOOLEAN     NOT NULL DEFAULT TRUE,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE level_waves (
    level_id   INTEGER NOT NULL REFERENCES levels (id) ON DELETE CASCADE,
    wave_index INTEGER NOT NULL,
    spawns     JSONB   NOT NULL,                          -- [{enemy_id,count,interval,delay}]
    PRIMARY KEY (level_id, wave_index)
);

CREATE TABLE enemies (
    id               INTEGER PRIMARY KEY,
    code             TEXT    NOT NULL UNIQUE,
    name             TEXT    NOT NULL,
    category         TEXT    NOT NULL,                    -- normal/ranged/flying/special/elite/boss
    hp               BIGINT  NOT NULL,
    speed            INTEGER NOT NULL,                    -- 定点 x1000（像素/秒）
    armor            INTEGER NOT NULL DEFAULT 0,
    shield_hp        BIGINT  NOT NULL DEFAULT 0,
    attack           INTEGER NOT NULL DEFAULT 0,
    attack_range     INTEGER NOT NULL DEFAULT 0,
    attack_interval  INTEGER NOT NULL DEFAULT 0,          -- 毫秒
    fly_height       INTEGER NOT NULL DEFAULT 0,
    burrow           BOOLEAN NOT NULL DEFAULT FALSE,
    is_boss          BOOLEAN NOT NULL DEFAULT FALSE
);

-- I-1 元素抗性矩阵：每个 BOSS 必有高抗 + 负抗组合，强制换元素搭配。
-- resist_pct 为千分比（-500 = 吃双倍伤害，+500 = 几乎免疫），
-- 与 domain 包中的定点整数约定一致，不要与百分比混淆。
CREATE TABLE enemy_element_resist (
    enemy_id   INTEGER NOT NULL REFERENCES enemies (id) ON DELETE CASCADE,
    element    TEXT    NOT NULL,                         -- fire/ice/lightning/corrosion/kinetic
    resist_pct INTEGER NOT NULL,                         -- 千分比 -500..500
    PRIMARY KEY (enemy_id, element),
    CONSTRAINT chk_resist_range CHECK (resist_pct BETWEEN -500 AND 500)
);

CREATE TABLE skills (
    id                INTEGER PRIMARY KEY,
    code              TEXT    NOT NULL UNIQUE,
    name              TEXT    NOT NULL,
    family            TEXT    NOT NULL,                   -- 8 系
    element           TEXT    NOT NULL,                   -- 主元素
    kind              TEXT    NOT NULL,                   -- active/passive
    descr             TEXT    NOT NULL DEFAULT '',
    base_damage       INTEGER NOT NULL,
    heat_cost         INTEGER NOT NULL,
    cooldown_ms       INTEGER NOT NULL,
    pierce            INTEGER NOT NULL DEFAULT 0,
    aoe_radius        INTEGER NOT NULL DEFAULT 0,
    apply_element     TEXT    NOT NULL DEFAULT '',
    apply_stacks      INTEGER NOT NULL DEFAULT 0,
    projectile_speed  INTEGER NOT NULL DEFAULT 0,         -- 定点 x1000
    chain             INTEGER NOT NULL DEFAULT 0,         -- 弹射次数
    unlock_level      INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX idx_skills_family ON skills (family);

-- I-2 / I-3：合成配方 = 构筑方向，A+B 合成更高阶
CREATE TABLE skill_recipes (
    id              INTEGER PRIMARY KEY,
    output_skill_id INTEGER NOT NULL REFERENCES skills (id) ON DELETE CASCADE,
    a_skill_id      INTEGER NOT NULL REFERENCES skills (id) ON DELETE CASCADE,
    b_skill_id      INTEGER NOT NULL REFERENCES skills (id) ON DELETE CASCADE,
    output_tier     INTEGER NOT NULL,
    UNIQUE (a_skill_id, b_skill_id)
);

-- 6 部位 × 3 阶；element 为契合标签（I-3：与技能同系给大加成，异系给不足 1/3 的小额）
CREATE TABLE equipment (
    id             INTEGER PRIMARY KEY,
    code           TEXT    NOT NULL UNIQUE,
    name           TEXT    NOT NULL,
    slot           TEXT    NOT NULL,                      -- weapon/helmet/armor/gloves/boots/charm
    tier           INTEGER NOT NULL,
    element        TEXT    NOT NULL,
    descr          TEXT    NOT NULL DEFAULT '',
    base_armor     INTEGER NOT NULL DEFAULT 0,
    base_bonus_pct INTEGER NOT NULL DEFAULT 0,
    unlock_level   INTEGER NOT NULL DEFAULT 1
);

-- 宝石属性目录：8 种属性。品质（gray..gold）是实例属性，
-- 存在 user_gems.quality 上，不在目录表里 —— 一枚蓝宝石和一枚金宝石
-- 是同一个 attr 的两个实例，不是两个目录条目。
CREATE TABLE gems (
    id      INTEGER PRIMARY KEY,
    code    TEXT NOT NULL UNIQUE,
    name    TEXT NOT NULL,
    attr    TEXT NOT NULL
);

-- 洗练随机词条池
CREATE TABLE gem_affixes (
    id         BIGSERIAL PRIMARY KEY,
    gem_id     INTEGER NOT NULL REFERENCES gems (id) ON DELETE CASCADE,
    affix      TEXT    NOT NULL,
    min_value  INTEGER NOT NULL,
    max_value  INTEGER NOT NULL
);
CREATE INDEX idx_gem_affixes_gem ON gem_affixes (gem_id);

-- 皮肤附带机制型词条（改变机制而非只加数值）
CREATE TABLE skins (
    id          INTEGER PRIMARY KEY,
    code        TEXT    NOT NULL UNIQUE,
    name        TEXT    NOT NULL,
    rarity      TEXT    NOT NULL,
    passive     TEXT    NOT NULL DEFAULT '',
    unlock_type TEXT    NOT NULL DEFAULT 'shop'
);

-- I-3 专精树：每系 12 节点（3 层 × 4 槽），每层 4 选 2，点数有限
CREATE TABLE mastery_trees (
    family TEXT PRIMARY KEY,
    name   TEXT NOT NULL
);

CREATE TABLE mastery_nodes (
    id             INTEGER PRIMARY KEY,
    family         TEXT    NOT NULL REFERENCES mastery_trees (family) ON DELETE CASCADE,
    layer          INTEGER NOT NULL,                      -- 1..3
    slot           INTEGER NOT NULL,                      -- 0..3
    name           TEXT    NOT NULL,
    kind           TEXT    NOT NULL,                      -- element_cap/reaction_mult/heat_cap/extra_slot/mechanic
    value          INTEGER NOT NULL,
    prereq_family  TEXT    NOT NULL DEFAULT '',
    prereq_layer   INTEGER NOT NULL DEFAULT 0,
    UNIQUE (family, layer, slot)
);
