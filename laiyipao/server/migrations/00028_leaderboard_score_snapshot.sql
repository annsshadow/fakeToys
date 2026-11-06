-- 00028_leaderboard_score_snapshot.sql — 榜单分数物化 + 分数直方图（R131）

-- +goose Up

-- 三榜的分数此前**每个请求现算**：
--   power      = user_progress.level_exp                          （无聚合）
--   stage      = MAX(level_id) WHERE clears > 0                    （1M 行 GROUP BY）
--   efficiency = MIN(min_power_clear) WHERE min_power_clear > 0    （1M 行 GROUP BY）
-- 派生两榜的 GROUP BY 是请求路径上唯一的大头：perf 库实测
-- efficiency 名次 316ms（其中全表 GroupAggregate 223ms）、total 239ms；
-- 榜单前 200 的 GROUP BY 同样要全表聚合一次。
--
-- 这里把「用户 → 该榜分数」物化到 user_progress 两列，并用触发器在**同一事务**
-- 内维护；同时维护按 (kind, score) 的直方图，使「优于我的人数」变成对 ≤5000 行
-- 的小表求和（实测 0.3~0.6ms），「同分且 id 小于我」走 (score, user_id) 索引
-- 范围计数（efficiency 中位 1.7ms、尾排 0.4ms）。
--
-- ⚠️ 为什么不复用已有的 max_stage：max_stage 是「最高已解锁关卡」，
-- 结算在**胜利局**写 GREATEST(max_stage, 关卡号)，语义接近但不等于
-- 「MAX(level_id) WHERE clears > 0」（例如历史/导入数据里两列可以不等）。
-- 榜单口径必须逐字不变，所以另立 clear_stage_max，回填/触发器都用
-- 与 leaderboardScoresSQL 完全相同的表达式。

ALTER TABLE user_progress
    ADD COLUMN IF NOT EXISTS clear_stage_max INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS min_power_clear_ever BIGINT NOT NULL DEFAULT 0;

-- 回填：与旧实时定义逐字一致（只更新 level_stars 里出现过的用户，
-- 其余用户两列保持 0 = 「无成绩、不参与」）。
UPDATE user_progress p
   SET clear_stage_max = s.max_level,
       min_power_clear_ever = s.min_pc
  FROM (
    SELECT user_id,
           COALESCE(MAX(level_id) FILTER (WHERE clears > 0), 0)                 AS max_level,
           COALESCE(MIN(min_power_clear) FILTER (WHERE min_power_clear > 0), 0) AS min_pc
      FROM level_stars
     GROUP BY user_id
  ) s
 WHERE p.user_id = s.user_id;

-- 直方图：kind 只有三个字面量（power/stage/efficiency），行数 ≤ 三榜不同分数之和
-- （perf 库实测 7099 行 / 736kB）。0 分只在 power 参与（见下方 zero_counts）。
CREATE TABLE IF NOT EXISTS leaderboard_score_counts (
    kind  TEXT   NOT NULL,
    score BIGINT NOT NULL,
    cnt   BIGINT NOT NULL,
    PRIMARY KEY (kind, score)
) WITH (fillfactor = 70);

INSERT INTO leaderboard_score_counts (kind, score, cnt)
SELECT 'power', level_exp, COUNT(*) FROM user_progress GROUP BY level_exp
UNION ALL
SELECT 'stage', clear_stage_max::bigint, COUNT(*) FROM user_progress
 WHERE clear_stage_max > 0 GROUP BY clear_stage_max
UNION ALL
SELECT 'efficiency', min_power_clear_ever, COUNT(*) FROM user_progress
 WHERE min_power_clear_ever > 0 GROUP BY min_power_clear_ever;

-- 排名用索引：与榜单 ORDER BY（score <dir>, user_id ASC）同一顺序，
-- 既支持「同分且 id < me」的范围计数，也支持前 N 的索引扫描 + 小排序。
CREATE INDEX IF NOT EXISTS idx_user_progress_stage
    ON user_progress (clear_stage_max DESC, user_id)
    WHERE clear_stage_max > 0;
CREATE INDEX IF NOT EXISTS idx_user_progress_efficiency
    ON user_progress (min_power_clear_ever, user_id)
    WHERE min_power_clear_ever > 0;

-- +goose StatementBegin
-- 直方图增量：把一个人的分数从 old 挪到 new。
--   p_zero_counts=true  → 0 分也算参与（power 榜包含从未成长的账号）
--   p_zero_counts=false → 0 分等于「无成绩、不参与」（stage/efficiency）
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
        -- 计数归零就删行：表始终只保留「确实有人」的分数档。
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
-- user_progress 是分数快照的宿主：任何写入（结算/注册/后台/测试夹具）
-- 都必须同步直方图，否则 /me/rank 的「优于我」会静默错位。
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

-- +goose StatementBegin
-- level_stars 是派生两榜的**唯一**源头：一次结算 upsert 一行，
-- 这里按该用户全部关卡重算 MAX/MIN（走 (user_id, level_id) 主键，行数少），
-- 只在值真的变化时才写 user_progress（避免每局多一次无意义行更新）。
CREATE OR REPLACE FUNCTION level_stars_leaderboard_sync() RETURNS trigger AS $$
DECLARE
    v_user  BIGINT;
    v_stage INTEGER;
    v_eff   BIGINT;
BEGIN
    -- ⚠️ DELETE 触发器里 NEW 记录**未赋值**：直接 `COALESCE(NEW.user_id, ...)`
    -- 会报 55000（record NEW is not assigned yet），所以必须按 TG_OP 分支。
    IF TG_OP = 'DELETE' THEN
        v_user := OLD.user_id;
    ELSE
        v_user := NEW.user_id;
    END IF;
    SELECT COALESCE(MAX(level_id) FILTER (WHERE clears > 0), 0),
           COALESCE(MIN(min_power_clear) FILTER (WHERE min_power_clear > 0), 0)
      INTO v_stage, v_eff
      FROM level_stars
     WHERE user_id = v_user;
    UPDATE user_progress
       SET clear_stage_max = v_stage,
           min_power_clear_ever = v_eff
     WHERE user_id = v_user
       AND (clear_stage_max IS DISTINCT FROM v_stage
            OR min_power_clear_ever IS DISTINCT FROM v_eff);
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

CREATE TRIGGER user_progress_leaderboard_sync
    AFTER INSERT OR UPDATE OR DELETE ON user_progress
    FOR EACH ROW EXECUTE FUNCTION user_progress_leaderboard_sync();

CREATE TRIGGER level_stars_leaderboard_sync
    AFTER INSERT OR UPDATE OR DELETE ON level_stars
    FOR EACH ROW EXECUTE FUNCTION level_stars_leaderboard_sync();

-- +goose Down

DROP TRIGGER IF EXISTS level_stars_leaderboard_sync ON level_stars;
DROP TRIGGER IF EXISTS user_progress_leaderboard_sync ON user_progress;
DROP FUNCTION IF EXISTS level_stars_leaderboard_sync();
DROP FUNCTION IF EXISTS user_progress_leaderboard_sync();
DROP FUNCTION IF EXISTS leaderboard_counts_delta(TEXT, BIGINT, BIGINT, BOOLEAN);
DROP TABLE IF EXISTS leaderboard_score_counts;
DROP INDEX IF EXISTS idx_user_progress_stage;
DROP INDEX IF EXISTS idx_user_progress_efficiency;
ALTER TABLE user_progress
    DROP COLUMN IF EXISTS clear_stage_max,
    DROP COLUMN IF EXISTS min_power_clear_ever;
