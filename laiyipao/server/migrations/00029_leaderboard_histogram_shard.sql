-- Copyright (C) 2026 annsshadow
-- SPDX-License-Identifier: AGPL-3.0-or-later

-- 第 146 轮：直方图按 user_id 分片，拆掉「全局单行写热点」。
--
-- # 为什么
--
-- 00028 的直方图对每个 (kind, score) 只有**一行**。结算事务里
-- `UPDATE user_progress` 的 AFTER 触发器要对这一行做「旧档 -1 / 新档 +1」，
-- 于是所有进度相近的玩家（level_exp 每局 +80，分档很粗）在同一时刻
-- 抢同一行 —— 单个 settle 的这条 UPDATE 从空载 0.155ms 涨到 c=8 的
-- 1.78ms（pg_stat_statements 实测，差值全是行锁排队）。
--
-- 修法：把一行拆成 `shard` 份，写路径按 `user_id % 16` 选分片。
-- 分片只由 user_id 决定、**与分数无关** —— 这是正确性前提：
-- 同一个用户的 +1 和 -1 必须落在同一行，否则减号会落空（静默少计）。
--
-- 读路径**一行 SQL 都不用改**：`/me/rank` 的三条查询本来就是
-- 「WHERE kind = ? 的整表 SUM / 范围 SUM」，多一列 shard 不改变语义，
-- 只是把行数从 ≤7099 涨到最多 16 倍。代价与收益都在台账 R146 里量化。
--
-- # 重建
--
-- 旧行全部落在 shard=0，与新写入的分片规则不一致，必须按新规则重算一遍。
-- 重算口径与 00028 的回填逐字相同（power 含 0 分，stage/efficiency 排除 <=0），
-- 只是把计数按 `user_id % 16` 再分一层。

-- +goose Up

ALTER TABLE leaderboard_score_counts
    ADD COLUMN IF NOT EXISTS shard SMALLINT NOT NULL DEFAULT 0;

ALTER TABLE leaderboard_score_counts
    DROP CONSTRAINT IF EXISTS leaderboard_score_counts_pkey;

ALTER TABLE leaderboard_score_counts
    ADD CONSTRAINT leaderboard_score_counts_pkey PRIMARY KEY (kind, score, shard);

DELETE FROM leaderboard_score_counts;

INSERT INTO leaderboard_score_counts (kind, score, shard, cnt)
SELECT 'power', level_exp::bigint, (user_id % 16)::smallint, COUNT(*)
  FROM user_progress
 GROUP BY level_exp, (user_id % 16)::smallint
UNION ALL
SELECT 'stage', clear_stage_max::bigint, (user_id % 16)::smallint, COUNT(*)
  FROM user_progress
 WHERE clear_stage_max > 0
 GROUP BY clear_stage_max, (user_id % 16)::smallint
UNION ALL
SELECT 'efficiency', min_power_clear_ever, (user_id % 16)::smallint, COUNT(*)
  FROM user_progress
 WHERE min_power_clear_ever > 0
 GROUP BY min_power_clear_ever, (user_id % 16)::smallint;

-- +goose StatementBegin
-- 直方图增量：把一个人的分数从 old 挪到 new。
--   p_zero_counts=true  → 0 分也算参与（power 榜包含从未成长的账号）
--   p_zero_counts=false → 0 分等于「无成绩、不参与」（stage/efficiency）
--   p_user_id           → 分片键（% 16 落在 SHARD_COUNT 以内）
--
-- 常见路径（old 与 new 都非空）压成**一条语句**：旧档 -1、旧档归零删除、
-- 新档 +1 在同一个 data-modifying CTE 里完成。三条小语句变一条，
-- 少一次 plpgsql 往返。
CREATE OR REPLACE FUNCTION leaderboard_counts_delta(
    p_kind TEXT, p_old BIGINT, p_new BIGINT, p_zero_counts BOOLEAN, p_user_id BIGINT
) RETURNS void AS $$
DECLARE
    v_old   BIGINT := p_old;
    v_new   BIGINT := p_new;
    v_shard SMALLINT := (p_user_id % 16)::smallint;
BEGIN
    IF NOT p_zero_counts THEN
        IF v_old IS NOT NULL AND v_old <= 0 THEN v_old := NULL; END IF;
        IF v_new IS NOT NULL AND v_new <= 0 THEN v_new := NULL; END IF;
    END IF;
    IF v_old IS NOT DISTINCT FROM v_new THEN
        RETURN;
    END IF;

    IF v_old IS NULL THEN
        INSERT INTO leaderboard_score_counts AS c (kind, score, shard, cnt)
        VALUES (p_kind, v_new, v_shard, 1)
        ON CONFLICT (kind, score, shard) DO UPDATE SET cnt = c.cnt + 1;
        RETURN;
    END IF;

    IF v_new IS NULL THEN
        UPDATE leaderboard_score_counts SET cnt = cnt - 1
         WHERE kind = p_kind AND score = v_old AND shard = v_shard AND cnt > 1;
        DELETE FROM leaderboard_score_counts
         WHERE kind = p_kind AND score = v_old AND shard = v_shard AND cnt <= 1;
        RETURN;
    END IF;

    -- 旧档只降不删（cnt 仍 > 0）时，UPDATE 命中即结束；
    -- cnt 原本就是 1 时 UPDATE 不命中，DELETE 收掉空行。
    WITH dec AS (
        UPDATE leaderboard_score_counts SET cnt = cnt - 1
         WHERE kind = p_kind AND score = v_old AND shard = v_shard AND cnt > 1
        RETURNING 1
    ), del AS (
        DELETE FROM leaderboard_score_counts
         WHERE kind = p_kind AND score = v_old AND shard = v_shard AND cnt <= 1
           AND NOT EXISTS (SELECT 1 FROM dec)
        RETURNING 1
    )
    INSERT INTO leaderboard_score_counts AS c (kind, score, shard, cnt)
    VALUES (p_kind, v_new, v_shard, 1)
    ON CONFLICT (kind, score, shard) DO UPDATE SET cnt = c.cnt + 1;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION user_progress_leaderboard_sync() RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM leaderboard_counts_delta('power', NULL, NEW.level_exp, TRUE, NEW.user_id);
        PERFORM leaderboard_counts_delta('stage', NULL, NEW.clear_stage_max::bigint, FALSE, NEW.user_id);
        PERFORM leaderboard_counts_delta('efficiency', NULL, NEW.min_power_clear_ever, FALSE, NEW.user_id);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM leaderboard_counts_delta('power', OLD.level_exp, NULL, TRUE, OLD.user_id);
        PERFORM leaderboard_counts_delta('stage', OLD.clear_stage_max::bigint, NULL, FALSE, OLD.user_id);
        PERFORM leaderboard_counts_delta('efficiency', OLD.min_power_clear_ever, NULL, FALSE, OLD.user_id);
    ELSE
        IF OLD.level_exp IS DISTINCT FROM NEW.level_exp THEN
            PERFORM leaderboard_counts_delta('power', OLD.level_exp, NEW.level_exp, TRUE, NEW.user_id);
        END IF;
        IF OLD.clear_stage_max IS DISTINCT FROM NEW.clear_stage_max THEN
            PERFORM leaderboard_counts_delta('stage', OLD.clear_stage_max::bigint, NEW.clear_stage_max::bigint, FALSE, NEW.user_id);
        END IF;
        IF OLD.min_power_clear_ever IS DISTINCT FROM NEW.min_power_clear_ever THEN
            PERFORM leaderboard_counts_delta('efficiency', OLD.min_power_clear_ever, NEW.min_power_clear_ever, FALSE, NEW.user_id);
        END IF;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

DROP FUNCTION IF EXISTS leaderboard_counts_delta(TEXT, BIGINT, BIGINT, BOOLEAN);

-- +goose Down

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION leaderboard_counts_delta(
    p_kind TEXT, p_old BIGINT, p_new BIGINT, p_zero_counts BOOLEAN
) RETURNS void AS $$
DECLARE
    v_old BIGINT := p_old;
    v_new BIGINT := p_new;
BEGIN
    IF NOT p_zero_counts THEN
        IF v_old IS NOT NULL AND v_old <= 0 THEN v_old := NULL; END IF;
        IF v_new IS NOT NULL AND v_new <= 0 THEN v_new := NULL; END IF;
    END IF;
    IF v_old IS NOT DISTINCT FROM v_new THEN
        RETURN;
    END IF;
    IF v_old IS NOT NULL THEN
        UPDATE leaderboard_score_counts SET cnt = cnt - 1
         WHERE kind = p_kind AND score = v_old;
        DELETE FROM leaderboard_score_counts
         WHERE kind = p_kind AND score = v_old AND cnt <= 0;
    END IF;
    IF v_new IS NOT NULL THEN
        INSERT INTO leaderboard_score_counts AS c (kind, score, cnt)
        VALUES (p_kind, v_new, 1)
        ON CONFLICT (kind, score) DO UPDATE SET cnt = c.cnt + 1;
    END IF;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION user_progress_leaderboard_sync() RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM leaderboard_counts_delta('power', NULL, NEW.level_exp, TRUE);
        PERFORM leaderboard_counts_delta('stage', NULL, NEW.clear_stage_max::bigint, FALSE);
        PERFORM leaderboard_counts_delta('efficiency', NULL, NEW.min_power_clear_ever, FALSE);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM leaderboard_counts_delta('power', OLD.level_exp, NULL, TRUE);
        PERFORM leaderboard_counts_delta('stage', OLD.clear_stage_max::bigint, NULL, FALSE);
        PERFORM leaderboard_counts_delta('efficiency', OLD.min_power_clear_ever, NULL, FALSE);
    ELSE
        IF OLD.level_exp IS DISTINCT FROM NEW.level_exp THEN
            PERFORM leaderboard_counts_delta('power', OLD.level_exp, NEW.level_exp, TRUE);
        END IF;
        IF OLD.clear_stage_max IS DISTINCT FROM NEW.clear_stage_max THEN
            PERFORM leaderboard_counts_delta('stage', OLD.clear_stage_max::bigint, NEW.clear_stage_max::bigint, FALSE);
        END IF;
        IF OLD.min_power_clear_ever IS DISTINCT FROM NEW.min_power_clear_ever THEN
            PERFORM leaderboard_counts_delta('efficiency', OLD.min_power_clear_ever, NEW.min_power_clear_ever, FALSE);
        END IF;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

DROP FUNCTION IF EXISTS leaderboard_counts_delta(TEXT, BIGINT, BIGINT, BOOLEAN, BIGINT);

ALTER TABLE leaderboard_score_counts
    DROP CONSTRAINT IF EXISTS leaderboard_score_counts_pkey;

DELETE FROM leaderboard_score_counts;

INSERT INTO leaderboard_score_counts (kind, score, cnt)
SELECT 'power', level_exp::bigint, COUNT(*) FROM user_progress GROUP BY level_exp
UNION ALL
SELECT 'stage', clear_stage_max::bigint, COUNT(*) FROM user_progress
 WHERE clear_stage_max > 0 GROUP BY clear_stage_max
UNION ALL
SELECT 'efficiency', min_power_clear_ever, COUNT(*) FROM user_progress
 WHERE min_power_clear_ever > 0 GROUP BY min_power_clear_ever;

ALTER TABLE leaderboard_score_counts DROP COLUMN shard;

ALTER TABLE leaderboard_score_counts
    ADD CONSTRAINT leaderboard_score_counts_pkey PRIMARY KEY (kind, score);
