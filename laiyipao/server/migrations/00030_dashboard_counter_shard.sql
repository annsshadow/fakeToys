-- Copyright (C) 2026 annsshadow
-- SPDX-License-Identifier: AGPL-3.0-or-later

-- 第 149 轮：看板增量计数器按 battle_records.id % 16 分片。
--
-- # 为什么
--
-- 00025/00026 把看板的战报统计挪到写侧触发器，但每关只有**一行**
-- `dashboard_battle_stats`、每个 (level, reaction) 也只有**一行**。
-- 1M 战报下玩家集中在少数关卡，c=8 结算时所有事务都 UPDATE 同一行：
--
--   dashboard_battle_stats 的 upsert 实测 mean 1.409ms / max 60.3ms
--   （另一次 1.203ms），而 battle_records 本身（关掉两条触发器）仅 0.227ms，
--   触发器开销 ≈1.3–1.5ms/笔 —— 时间几乎全在行锁排队，不在 SQL 计算。
--
-- 修法：与 R146 的直方图同型 —— 把一行拆成 16 份，写路径按
-- `battle_records.id % 16` 选分片。分片键只能取**不可变的主键 id**：
-- 同一条战报的 +1（INSERT）与 -1（DELETE/UPDATE 撤销）必须落在同一行，
-- 否则减号落空、读侧 SUM 只涨不跌，且没有任何报错。
--
-- # 存量回填
--
-- 旧行全部落在 shard=0，但历史战报删除时按 id%16 扣减，会打到不存在的分片
-- （计数变负、SUM 漂移）。所以 Up 必须按新规则**重建**两张表，
-- 而不是像 R146 那样保留旧行。重建只读 battle_records，不动业务行。
--
-- # 读侧
--
-- 读侧从「直接读一行」改为「按 level_id / (level_id,reaction) SUM 聚合」：
-- 100 关 × 16 片 = 最多 1600 行，聚合仍是亚毫秒级；语义与旧单行口径逐位相同。
--
-- # 回滚
--
-- Down 先把多分片行 SUM 合并回单行（否则 PK 冲突），再删 shard 列、
-- 恢复 00026 的 5 参数 apply / 旧 sync 与 3 参数 reaction apply。
-- 合并有损但可重算：重新 Up 会按 battle_records 重建。

-- +goose Up

ALTER TABLE dashboard_battle_stats
    ADD COLUMN IF NOT EXISTS shard SMALLINT NOT NULL DEFAULT 0;

ALTER TABLE dashboard_reaction_counts
    ADD COLUMN IF NOT EXISTS shard SMALLINT NOT NULL DEFAULT 0;

ALTER TABLE dashboard_battle_stats
    DROP CONSTRAINT IF EXISTS dashboard_battle_stats_pkey;

ALTER TABLE dashboard_battle_stats
    ADD CONSTRAINT dashboard_battle_stats_pkey PRIMARY KEY (level_id, shard);

ALTER TABLE dashboard_reaction_counts
    DROP CONSTRAINT IF EXISTS dashboard_reaction_counts_pkey;

ALTER TABLE dashboard_reaction_counts
    ADD CONSTRAINT dashboard_reaction_counts_pkey PRIMARY KEY (level_id, reaction, shard);

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_battle_stats_apply(
    p_level_id INT, p_result TEXT, p_duration BIGINT, p_wave INT,
    p_shard SMALLINT, p_sign INT
) RETURNS void AS $$
BEGIN
    INSERT INTO dashboard_battle_stats AS s
        (level_id, shard, attempts, wins, duration_sum, wave_sum)
    VALUES (p_level_id,
            p_shard,
            p_sign,
            CASE WHEN p_result = 'win' THEN p_sign ELSE 0 END,
            p_sign * p_duration,
            p_sign * p_wave)
    ON CONFLICT (level_id, shard) DO UPDATE
       SET attempts     = s.attempts + EXCLUDED.attempts,
           wins         = s.wins + EXCLUDED.wins,
           duration_sum = s.duration_sum + EXCLUDED.duration_sum,
           wave_sum     = s.wave_sum + EXCLUDED.wave_sum;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_battle_stats_sync()
RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM dashboard_battle_stats_apply(
            NEW.level_id, NEW.result, NEW.duration_ms::bigint,
            NEW.wave_reached::int, (NEW.id % 16)::smallint, 1);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM dashboard_battle_stats_apply(
            OLD.level_id, OLD.result, OLD.duration_ms::bigint,
            OLD.wave_reached::int, (OLD.id % 16)::smallint, -1);
    ELSIF OLD.level_id <> NEW.level_id
       OR OLD.result <> NEW.result
       OR OLD.duration_ms <> NEW.duration_ms
       OR OLD.wave_reached <> NEW.wave_reached THEN
        PERFORM dashboard_battle_stats_apply(
            OLD.level_id, OLD.result, OLD.duration_ms::bigint,
            OLD.wave_reached::int, (OLD.id % 16)::smallint, -1);
        PERFORM dashboard_battle_stats_apply(
            NEW.level_id, NEW.result, NEW.duration_ms::bigint,
            NEW.wave_reached::int, (NEW.id % 16)::smallint, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_reaction_counts_apply(
    p_level_id INT, p_reactions JSONB, p_shard SMALLINT, p_sign INT
) RETURNS void AS $$
BEGIN
    IF jsonb_typeof(p_reactions) <> 'object' THEN
        RETURN;
    END IF;
    INSERT INTO dashboard_reaction_counts AS c (level_id, reaction, shard, count)
    SELECT p_level_id, e.key, p_shard, p_sign * SUM(e.value::bigint)
      FROM jsonb_each_text(p_reactions) AS e(key, value)
     GROUP BY e.key
    ON CONFLICT (level_id, reaction, shard)
        DO UPDATE SET count = c.count + EXCLUDED.count;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_reaction_counts_sync()
RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM dashboard_reaction_counts_apply(
            NEW.level_id, NEW.reactions_used, (NEW.id % 16)::smallint, 1);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM dashboard_reaction_counts_apply(
            OLD.level_id, OLD.reactions_used, (OLD.id % 16)::smallint, -1);
    ELSIF OLD.level_id <> NEW.level_id
       OR OLD.reactions_used <> NEW.reactions_used THEN
        PERFORM dashboard_reaction_counts_apply(
            OLD.level_id, OLD.reactions_used, (OLD.id % 16)::smallint, -1);
        PERFORM dashboard_reaction_counts_apply(
            NEW.level_id, NEW.reactions_used, (NEW.id % 16)::smallint, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- 旧签名已无调用方，删掉避免同名重载混淆（与 00026 的 DROP 做法一致）。
DROP FUNCTION IF EXISTS dashboard_battle_stats_apply(INT, TEXT, BIGINT, INT, INT);
DROP FUNCTION IF EXISTS dashboard_reaction_counts_apply(INT, JSONB, INT);

-- 存量重建：按 battle_records 的 id 分片重算，保证历史删除也落在正确分片。
DELETE FROM dashboard_battle_stats;
INSERT INTO dashboard_battle_stats
    (level_id, shard, attempts, wins, duration_sum, wave_sum)
SELECT level_id,
       (id % 16)::smallint,
       COUNT(*),
       COUNT(*) FILTER (WHERE result = 'win'),
       COALESCE(SUM(duration_ms), 0),
       COALESCE(SUM(wave_reached), 0)
  FROM battle_records
 GROUP BY level_id, (id % 16)::smallint;

DELETE FROM dashboard_reaction_counts;
INSERT INTO dashboard_reaction_counts (level_id, reaction, shard, count)
SELECT b.level_id, e.key, (b.id % 16)::smallint, SUM(e.value::bigint)
  FROM battle_records b,
       LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
 WHERE jsonb_typeof(b.reactions_used) = 'object'
 GROUP BY b.level_id, e.key, (b.id % 16)::smallint;

-- +goose Down

-- 先恢复旧签名函数（5 参数 apply / 3 参数 reaction apply）与原 sync。
-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_battle_stats_apply(
    p_level_id INT, p_result TEXT, p_duration BIGINT, p_wave INT, p_sign INT
) RETURNS void AS $$
BEGIN
    INSERT INTO dashboard_battle_stats AS s
        (level_id, attempts, wins, duration_sum, wave_sum)
    VALUES (p_level_id,
            p_sign,
            CASE WHEN p_result = 'win' THEN p_sign ELSE 0 END,
            p_sign * p_duration,
            p_sign * p_wave)
    ON CONFLICT (level_id) DO UPDATE
       SET attempts     = s.attempts + EXCLUDED.attempts,
           wins         = s.wins + EXCLUDED.wins,
           duration_sum = s.duration_sum + EXCLUDED.duration_sum,
           wave_sum     = s.wave_sum + EXCLUDED.wave_sum;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_battle_stats_sync()
RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM dashboard_battle_stats_apply(
            NEW.level_id, NEW.result, NEW.duration_ms::bigint,
            NEW.wave_reached::int, 1);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM dashboard_battle_stats_apply(
            OLD.level_id, OLD.result, OLD.duration_ms::bigint,
            OLD.wave_reached::int, -1);
    ELSIF OLD.level_id <> NEW.level_id
       OR OLD.result <> NEW.result
       OR OLD.duration_ms <> NEW.duration_ms
       OR OLD.wave_reached <> NEW.wave_reached THEN
        PERFORM dashboard_battle_stats_apply(
            OLD.level_id, OLD.result, OLD.duration_ms::bigint,
            OLD.wave_reached::int, -1);
        PERFORM dashboard_battle_stats_apply(
            NEW.level_id, NEW.result, NEW.duration_ms::bigint,
            NEW.wave_reached::int, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_reaction_counts_apply(
    p_level_id INT, p_reactions JSONB, p_sign INT
) RETURNS void AS $$
BEGIN
    IF jsonb_typeof(p_reactions) <> 'object' THEN
        RETURN;
    END IF;
    INSERT INTO dashboard_reaction_counts AS c (level_id, reaction, count)
    SELECT p_level_id, e.key, p_sign * SUM(e.value::bigint)
      FROM jsonb_each_text(p_reactions) AS e(key, value)
     GROUP BY e.key
    ON CONFLICT (level_id, reaction)
        DO UPDATE SET count = c.count + EXCLUDED.count;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_reaction_counts_sync()
RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM dashboard_reaction_counts_apply(NEW.level_id, NEW.reactions_used, 1);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM dashboard_reaction_counts_apply(OLD.level_id, OLD.reactions_used, -1);
    ELSIF OLD.level_id <> NEW.level_id
       OR OLD.reactions_used <> NEW.reactions_used THEN
        PERFORM dashboard_reaction_counts_apply(OLD.level_id, OLD.reactions_used, -1);
        PERFORM dashboard_reaction_counts_apply(NEW.level_id, NEW.reactions_used, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

DROP FUNCTION IF EXISTS dashboard_battle_stats_apply(INT, TEXT, BIGINT, INT, SMALLINT, INT);
DROP FUNCTION IF EXISTS dashboard_reaction_counts_apply(INT, JSONB, SMALLINT, INT);

-- 多分片先合并成 shard=0 的单行，再删其余分片，最后删 shard 列
-- （直接 DELETE 全表再 INSERT 会读到空表，必须在同一快照里先聚合）。
INSERT INTO dashboard_battle_stats AS s
    (level_id, shard, attempts, wins, duration_sum, wave_sum)
SELECT level_id, 0::smallint, SUM(attempts), SUM(wins),
       SUM(duration_sum), SUM(wave_sum)
  FROM dashboard_battle_stats
 GROUP BY level_id
ON CONFLICT (level_id, shard) DO UPDATE
   SET attempts     = EXCLUDED.attempts,
       wins         = EXCLUDED.wins,
       duration_sum = EXCLUDED.duration_sum,
       wave_sum     = EXCLUDED.wave_sum;

DELETE FROM dashboard_battle_stats WHERE shard <> 0;

INSERT INTO dashboard_reaction_counts AS c (level_id, reaction, shard, count)
SELECT level_id, reaction, 0::smallint, SUM(count)
  FROM dashboard_reaction_counts
 GROUP BY level_id, reaction
ON CONFLICT (level_id, reaction, shard) DO UPDATE
   SET count = EXCLUDED.count;

DELETE FROM dashboard_reaction_counts WHERE shard <> 0;

ALTER TABLE dashboard_battle_stats
    DROP CONSTRAINT IF EXISTS dashboard_battle_stats_pkey;
ALTER TABLE dashboard_battle_stats DROP COLUMN shard;
ALTER TABLE dashboard_battle_stats
    ADD CONSTRAINT dashboard_battle_stats_pkey PRIMARY KEY (level_id);

ALTER TABLE dashboard_reaction_counts
    DROP CONSTRAINT IF EXISTS dashboard_reaction_counts_pkey;
ALTER TABLE dashboard_reaction_counts DROP COLUMN shard;
ALTER TABLE dashboard_reaction_counts
    ADD CONSTRAINT dashboard_reaction_counts_pkey PRIMARY KEY (level_id, reaction);
