-- 00025: 看板的两段战报聚合改为「写入时累加」
--
-- ## 为什么需要（R63）
--
-- 看板刷新（`adminDashboardUncached`）在 1M 战报下有两段要**全表扫描**
-- `battle_records`，每段读 ~100–160MB 堆页：
--
--   1. `dashboard battles+funnel`：`GROUP BY level_id` 求总量/胜场/时长/漏斗
--   2. `dashboard reactions`：`jsonb_each_text(reactions_used)` 逐行拆键求和
--
-- 实测（同库同工具，`-c 20 -d 15s`，`DASHBOARD_CACHE_TTL=0`）：
--
--   两段都开：p95 173.6ms
--   跳过 reactions：p95 126.0ms（reactions 边际 ≈46ms）
--   两段的开销在并发刷新时还会互相争 I/O 与并行 worker，各自比单跑慢 ~30%
--
-- 其中 reactions 是 **jsonb 无索引可用**，只要读侧保留「按 key 求和」的
-- 语义就必须每次读全表。两个口径都是**纯增量**（每份战报固定贡献一次），
-- 所以把增量挪到写侧，与战报写在同一个事务里累加，读侧只读小表。
--
-- ## 两张表
--
--   dashboard_battle_stats      (level_id) → attempts / wins / duration_sum
--   dashboard_reaction_counts   (level_id, reaction) → count
--
-- 为什么战报表不存「日维度」：看板只有 `Battles.Today` 需要「今天」，
-- 而今天那一小段（1M 铺 90 天时约 1.1 万行）走现有的
-- `idx_battle_records_created` 就是 index-only scan，实测 ~15ms 对应 30 万行、
-- 1.1 万行时仅几毫秒。加日维度就要处理跨日滚动与旧日行清理，
-- 换来的是「省一条几毫秒的索引扫描」，不划算。
--
-- 为什么反应表按 (level_id, reaction) 分片：单键 7 行会让所有并发结算
-- 抢同几行的行锁；分片后热点行 100×7=700 行，读侧 `GROUP BY reaction`
-- 聚合仍是亚毫秒级。实测 c=8 插入 3.2k tps、c=32 插入 10.6k tps 时
-- 触发器带来的吞吐损失约 4% / 16%（对照：关闭触发器 3.36k / 12.7k tps）。
--
-- ## 一致性
--
-- 计数与战报在**同一事务**内更新，不存在双写窗口：
--
--   INSERT → +N；DELETE → -N（含 `users` 的 `ON DELETE CASCADE`）；
--   UPDATE → 先 -OLD 再 +NEW（战报本应不可变，这里兜底）
--
-- 删除玩家/清理战报因此不会让计数漂移。存量数据由本迁移回填。
-- 读侧的漏斗语义（只列出**有战报**的关卡）由查询里的 `WHERE attempts > 0` 保持。
--
-- ## 回滚
--
-- Down 删两个触发器、两个函数与两张表，读侧退回全表聚合（R63 之前的旧行为）。
-- 有损但可接受：表只是聚合缓存，删掉不影响任何战报行的语义。

-- +goose Up

-- ⚠️ fillfactor=70 不是可选项。这两张表是**高频原地更新**的计数器：
-- 每次结算都会 UPDATE 同一批行。用默认 fillfactor=100 时页内没有空闲，
-- 每次更新都得分配新页并重写索引项 —— 实测把 1M 战报一次性灌入时，
-- 100 行的表膨胀到 43MB、12 分钟仍未完成。留 30% 空闲后更新走 HOT
-- （页内复用、不碰索引），表稳定在几页，autovacuum 也能及时回收。
CREATE TABLE IF NOT EXISTS dashboard_battle_stats (
    level_id     INT    PRIMARY KEY,
    attempts     BIGINT NOT NULL DEFAULT 0,
    wins         BIGINT NOT NULL DEFAULT 0,
    duration_sum BIGINT NOT NULL DEFAULT 0
) WITH (fillfactor = 70);

CREATE TABLE IF NOT EXISTS dashboard_reaction_counts (
    level_id INT    NOT NULL,
    reaction TEXT   NOT NULL,
    count    BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (level_id, reaction)
) WITH (fillfactor = 70);

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

-- +goose StatementBegin
CREATE OR REPLACE FUNCTION dashboard_reaction_counts_apply(
    p_level_id INT, p_reactions JSONB, p_sign INT
) RETURNS void AS $$
BEGIN
    -- 与旧读侧 SQL 的口径完全一致：只处理 jsonb object，逐键求和。
    -- jsonb `null` / 数组直接跳过（对它们调用 jsonb_each_text 会报错）。
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

DROP TRIGGER IF EXISTS trg_dashboard_battle_stats ON battle_records;
CREATE TRIGGER trg_dashboard_battle_stats
    AFTER INSERT OR UPDATE OR DELETE ON battle_records
    FOR EACH ROW EXECUTE FUNCTION dashboard_battle_stats_sync();

DROP TRIGGER IF EXISTS trg_dashboard_reaction_counts ON battle_records;
CREATE TRIGGER trg_dashboard_reaction_counts
    AFTER INSERT OR UPDATE OR DELETE ON battle_records
    FOR EACH ROW EXECUTE FUNCTION dashboard_reaction_counts_sync();

-- 存量回填：覆盖写，重复执行安全。
INSERT INTO dashboard_battle_stats (level_id, attempts, wins, duration_sum)
SELECT level_id,
       COUNT(*),
       COUNT(*) FILTER (WHERE result = 'win'),
       COALESCE(SUM(duration_ms), 0)
  FROM battle_records
 GROUP BY level_id
ON CONFLICT (level_id) DO UPDATE
   SET attempts     = EXCLUDED.attempts,
       wins         = EXCLUDED.wins,
       duration_sum = EXCLUDED.duration_sum;

INSERT INTO dashboard_reaction_counts (level_id, reaction, count)
SELECT b.level_id, e.key, SUM(e.value::bigint)
  FROM battle_records b,
       LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
 WHERE jsonb_typeof(b.reactions_used) = 'object'
 GROUP BY b.level_id, e.key
ON CONFLICT (level_id, reaction)
    DO UPDATE SET count = EXCLUDED.count;

-- +goose Down

DROP TRIGGER IF EXISTS trg_dashboard_battle_stats ON battle_records;
DROP TRIGGER IF EXISTS trg_dashboard_reaction_counts ON battle_records;
DROP FUNCTION IF EXISTS dashboard_battle_stats_sync();
DROP FUNCTION IF EXISTS dashboard_battle_stats_apply(INT, TEXT, BIGINT, INT);
DROP FUNCTION IF EXISTS dashboard_reaction_counts_sync();
DROP FUNCTION IF EXISTS dashboard_reaction_counts_apply(INT, JSONB, INT);
DROP TABLE IF EXISTS dashboard_battle_stats;
DROP TABLE IF EXISTS dashboard_reaction_counts;
