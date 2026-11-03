-- 00026: 关卡列表的通关统计改读增量表（补 wave_sum）
--
-- ## 为什么需要（R64）
--
-- 用 `pg_stat_statements` 按**总耗时**给管理端流量排序，第一名不是看板，
-- 而是 `/admin/levels`：单次 245ms（单跑）/ 501ms（并发 10），
-- 8 秒压测里 230 次调用累计 115 秒，比第二名（`/admin/battles` 的
-- `COUNT(*)`，45ms × 2177 次 = 99 秒）还高。
--
-- 计划实测（1M 战报）：
--
--   Parallel Seq Scan battle_records  104.9ms（读 ~100MB 堆页）
--   HashAggregate                     130ms（按 level_id 聚合 1M 行）
--   总计                              344.9ms（冷缓存）
--
-- 与 R63 的结论一致：**全表聚合拿不到索引的好处**（覆盖索引实测
-- 104ms seq vs 108ms 强制 index-only，净收益≈0）。所以把「每关的
-- 战报计数 / 平均波次」也挪到写入侧，复用 R63 的
-- `dashboard_battle_stats`，只补一列 `wave_sum`。
--
-- ## 口径
--
--   attempts = COUNT(*)                     → 已有 attempts
--   clears   = COUNT(*) FILTER (result=win) → 已有 wins
--   avg_wave = AVG(wave_reached)            → 新增 wave_sum
--              = wave_sum / NULLIF(attempts, 0)
--
-- `AVG` 与 `SUM/COUNT` 在 numeric 下逐位相等（旧读侧 SQL 是
-- `COALESCE(AVG(br.wave_reached), 0)`）；无战报的关卡在 LEFT JOIN 后
-- 仍是 0，与旧口径一致。
--
-- ## 更新语义
--
-- 沿用 R63 的触发器：INSERT +1、DELETE -1、UPDATE 变化时先 -OLD 再 +NEW。
-- 本迁移把 `wave_reached` 也纳入「变化」判据 —— 只改波次不改结果时，
-- 旧函数会漏更新，`avg_wave` 就会停在旧值。
--
-- ## 回滚
--
-- Down 恢复 4 参数版 apply/sync（不再累加 wave_sum）并删除该列。
-- ⚠️ 回滚后**旧二进制**才能正确读关卡统计；新代码引用了 wave_sum，
-- 必须先回滚代码再回滚迁移（与 R59 的先后顺序约定一致）。
-- 回填列有损（重新 Up 会按 battle_records 重算），不影响战报行语义。

-- +goose Up

ALTER TABLE dashboard_battle_stats
    ADD COLUMN IF NOT EXISTS wave_sum BIGINT NOT NULL DEFAULT 0;

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

-- 旧 4 参数版本已无调用方，删掉避免同名重载混淆。
DROP FUNCTION IF EXISTS dashboard_battle_stats_apply(INT, TEXT, BIGINT, INT);

-- 存量回填：只 UPDATE 已有行；没有战报的关卡 wave_sum 保持 0。
UPDATE dashboard_battle_stats s
   SET wave_sum = a.wave_sum
  FROM (SELECT level_id, SUM(wave_reached)::bigint AS wave_sum
          FROM battle_records
         GROUP BY level_id) a
 WHERE a.level_id = s.level_id;

-- +goose Down

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_battle_stats_apply(
    p_level_id INT, p_result TEXT, p_duration BIGINT, p_sign INT
) RETURNS void AS $$
BEGIN
    INSERT INTO dashboard_battle_stats AS s (level_id, attempts, wins, duration_sum)
    VALUES (p_level_id,
            p_sign,
            CASE WHEN p_result = 'win' THEN p_sign ELSE 0 END,
            p_sign * p_duration)
    ON CONFLICT (level_id) DO UPDATE
       SET attempts     = s.attempts + EXCLUDED.attempts,
           wins         = s.wins + EXCLUDED.wins,
           duration_sum = s.duration_sum + EXCLUDED.duration_sum;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_battle_stats_sync()
RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM dashboard_battle_stats_apply(
            NEW.level_id, NEW.result, NEW.duration_ms::bigint, 1);
    ELSIF TG_OP = 'DELETE' THEN
        PERFORM dashboard_battle_stats_apply(
            OLD.level_id, OLD.result, OLD.duration_ms::bigint, -1);
    ELSIF OLD.level_id <> NEW.level_id
       OR OLD.result <> NEW.result
       OR OLD.duration_ms <> NEW.duration_ms THEN
        PERFORM dashboard_battle_stats_apply(
            OLD.level_id, OLD.result, OLD.duration_ms::bigint, -1);
        PERFORM dashboard_battle_stats_apply(
            NEW.level_id, NEW.result, NEW.duration_ms::bigint, 1);
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
-- +goose StatementEnd

DROP FUNCTION IF EXISTS dashboard_battle_stats_apply(INT, TEXT, BIGINT, INT, INT);

ALTER TABLE dashboard_battle_stats DROP COLUMN IF EXISTS wave_sum;
