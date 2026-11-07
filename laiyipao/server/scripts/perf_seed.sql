-- Copyright (C) 2026 annsshadow
-- SPDX-License-Identifier: AGPL-3.0-or-later
--
-- perf_seed.sql — R60 百万级压测的**仿真种子数据**（不是脱敏真实数据）
--
-- ## 数据来源与性质
--
-- 全部由本脚本用 generate_series 生成，**不是**线上导出的脱敏数据；
-- 不含任何真实账号、凭证、昵称或玩家行为。适合压容量与索引命中，
-- 不适合用来推断真实玩家的分布（真实分布见「已知偏差」）。
--
-- ## 规模与分布
--
--   users          1,000,000  id=1..1000000，guest_token='perf_<id>'，
--                            nickname='P<id>'，全部游客、全部正常状态；
--                            created_at 均匀铺在 90 天、last_login_at 铺在 30 天。
--   battle_records 1,000,000  每用户恰好 1 条；level_id=1+(id%100)；胜率 80%；
--                            reactions_used 为小 jsonb（胜局两键、败局一键），
--                            用来让看板的 reaction 增量表真的有数据。
--   user_progress  1,000,000  每用户 1 行；level_exp=(id%5000)，max_stage=(id%100)。
--   level_stars    1,000,000  每用户 1 行；level_id=1+(id%100)；clears 为 id%3。
--
-- 已知偏差：真实玩家的战报/星级是**长尾**（少数重度玩家上千条），这里刻意
-- 做成每个用户 1 条，用于测「全表聚合」；重度玩家路径由单独的 auth 用户
-- （额外插 1000 条）覆盖。
--
-- ## 用法（必须先建库、迁移、seed 游戏内容）
--
--   CREATE DATABASE laiyipao_r60_perf;
--   DATABASE_URL=...laiyipao_r60_perf go run ./cmd/migrate
--   DATABASE_URL=...laiyipao_r60_perf go run ./cmd/seed
--   docker exec -i lyp-pg psql -d laiyipao_r60_perf -v ON_ERROR_STOP=1 < perf_seed.sql
--
-- ## 触发器：批量导入必须「关 → 灌 → 回填」（R65）
--
-- 00025/00026 在 battle_records 上挂了逐行触发器，把每份战报累加进
-- `dashboard_battle_stats` / `dashboard_reaction_counts`。这对**单条结算**
-- 是正确的（读写同一事务、无漂移），但对**批量导入**是灾难：
--
--   - 1M 行逐行触发 = 1M 次 `INSERT ... ON CONFLICT UPDATE`，每次都要写
--     WAL、抢 100 行的行锁、更新索引；实测 12 分钟仍未完成；
--   - 两个触发器各跑一遍，计数表被反复原地更新，默认 fillfactor 下
--     100 行的表膨胀到 43MB（R63 记录）。
--
-- 所以本脚本显式 `DISABLE TRIGGER USER` → 灌入 → **按同一口径回填** →
-- `ENABLE TRIGGER USER`。回填 SQL 与 00025/00026 的迁移回填逐字一致，
-- 保证「批量导入后的计数」与「逐条结算出来的计数」共用同一个定义。
--
-- 同库同工具 10 万行 A/B（R65 实测，唯一变量是有没有 DISABLE）：
--   battle_records 插入   34.97s（触发器开） → 1.13s（触发器关，31×）
--   整脚本                37.34s            → 3.31s（11×）
--   1M 全量（触发器关）   34.5s 完成（battle 插入 12.0s），两张增量表
--                          稳定在 56kB / 64kB、死元组 0
--   （R63 用默认 fillfactor 且不关触发器直接灌 1M：12 分钟未完成、43MB）
--
-- ⚠️ 整个导入包在一个事务里（`BEGIN ... COMMIT`）：
--   - 中途失败会整体回滚，**不会**把触发器留在关闭状态 —— 这比单纯
--     `DISABLE` 更安全，因为 DDL 在 PG 里是事务性的；
--   - 因此脚本只适合**独占的空演练库**：大事务期间持有
--     `battle_records` 上的 ShareRowExclusiveLock。
--
-- ## 自校验（不通过就报错退出）
--
-- 灌入结束后、提交前，脚本会把两张增量表与 `battle_records` 的**直接聚合**
-- 做对称差比对（两个方向都查，避免只查「多出来的行」漏掉「少掉的行」）：
--
--   dashboard_battle_stats    ↔ COUNT / wins / duration_sum / wave_sum
--   dashboard_reaction_counts ↔ jsonb_each_text 逐键求和
--
-- 任一不一致 → `RAISE EXCEPTION` → psql 非零退出、事务回滚。
-- 这样「下次压测又踩同一个坑」会在灌数阶段就暴露，而不是等看板数字对不上。
--
-- ## 清理与重复执行
--
--   DROP DATABASE laiyipao_r60_perf WITH (FORCE);
--   本脚本只写这个演练库，不触碰开发库。
--
--   重复执行：脚本开头有「users 必须为空」的保护，非空库会直接报错退出，
--   不会重复灌数。要重来请先 DROP/CREATE 库再迁移。

\set ON_ERROR_STOP on

-- 幂等安全阀：只在全新库上灌数。
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM users LIMIT 1) THEN
    RAISE EXCEPTION 'perf_seed: users 非空，拒绝重复灌数（请重建演练库）';
  END IF;
END $$;

-- 本地演练库：牺牲崩溃持久性换插入速度（压测期间不会重启容器）。
SET synchronous_commit = off;
\timing on

BEGIN;

-- 绕过逐行增量触发器，灌完再一次性回填（见文件头的 R65 说明）。
-- DISABLE 在事务内，失败回滚即自动恢复，不会留下「触发器被永久关掉」的库。
ALTER TABLE battle_records DISABLE TRIGGER USER;

INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
SELECT 'perf_' || n,
       'P' || n,
       TRUE,
       1,
       now() - ((n % 9000)::float / 9000) * interval '90 days',
       now() - ((n % 3000)::float / 3000) * interval '30 days'
FROM generate_series(1, 1000000) AS n;

INSERT INTO battle_records (user_id, level_id, result, stars, score, kills, leaked,
                            wave_reached, duration_ms, shots, hits, reactions_used, created_at)
-- ⚠️ user_id 通过 guest_token 回查，不能写死 `n`：
-- 前一次执行失败回滚后 `users.id` 的序列**不会回退**，第二次插入的 id
-- 会从 1001 起，写死 `n` 会触发 `battle_records_user_id_fkey` 外键错误。
SELECT u.id,
       1 + (n % 100),
       CASE WHEN n % 10 < 8 THEN 'win' ELSE 'lose' END,
       CASE WHEN n % 10 < 8 THEN 3 ELSE 0 END,
       (n % 100000)::bigint,
       (n % 50),
       (n % 10),
       (n % 20),
       1000 + (n % 60000),
       (n % 500),
       (n % 300),
       -- 与真实分布同形状的小对象：胜局两个键、败局一个键（值非零）。
       CASE WHEN n % 10 < 8
            THEN jsonb_build_object('steam_burst', 1 + (n % 4), 'freeze', 1 + (n % 2))
            ELSE jsonb_build_object('steam_burst', 1 + (n % 3))
       END,
       now() - ((n % 9000)::float / 9000) * interval '90 days'
FROM generate_series(1, 1000000) AS n
JOIN users u ON u.guest_token = 'perf_' || n;

INSERT INTO user_progress (user_id, max_stage, level_exp, total_battles, total_kills)
SELECT u.id, n % 100, (n % 5000)::bigint, 1, n % 500
FROM generate_series(1, 1000000) AS n
JOIN users u ON u.guest_token = 'perf_' || n;

INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
SELECT u.id, 1 + (n % 100), 1 + (n % 3), (n % 100000)::bigint, n % 3, (n % 2000)::bigint
FROM generate_series(1, 1000000) AS n
JOIN users u ON u.guest_token = 'perf_' || n;

-- 回填增量表：与 00025/00026 的回填口径逐字一致（覆盖写，可重复执行）。
-- R149：计数器按 battle_records.id % 16 分片，回填也必须按分片写。
INSERT INTO dashboard_battle_stats
    (level_id, shard, attempts, wins, duration_sum, wave_sum)
SELECT level_id,
       (id % 16)::smallint,
       COUNT(*)::bigint,
       COUNT(*) FILTER (WHERE result = 'win')::bigint,
       COALESCE(SUM(duration_ms), 0)::bigint,
       COALESCE(SUM(wave_reached), 0)::bigint
  FROM battle_records
 GROUP BY level_id, (id % 16)::smallint
ON CONFLICT (level_id, shard) DO UPDATE
   SET attempts     = EXCLUDED.attempts,
       wins         = EXCLUDED.wins,
       duration_sum = EXCLUDED.duration_sum,
       wave_sum     = EXCLUDED.wave_sum;

INSERT INTO dashboard_reaction_counts (level_id, reaction, shard, count)
SELECT b.level_id, e.key, (b.id % 16)::smallint, SUM(e.value::bigint)::bigint
  FROM battle_records b,
       LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
 WHERE jsonb_typeof(b.reactions_used) = 'object'
 GROUP BY b.level_id, e.key, (b.id % 16)::smallint
ON CONFLICT (level_id, reaction, shard) DO UPDATE
   SET count = EXCLUDED.count;

-- 让优化器拿到真实统计信息，否则 EXPLAIN/实际计划会低估 100 万行。
ANALYZE users;
ANALYZE battle_records;
ANALYZE user_progress;
ANALYZE level_stars;
ANALYZE dashboard_battle_stats;
ANALYZE dashboard_reaction_counts;

-- 提交前自校验：增量表必须等于直接聚合的**真值**（双向对称差）。
DO $$
DECLARE
    diff BIGINT;
BEGIN
    -- ① 每关战报统计（attempts / wins / duration_sum / wave_sum）
    SELECT COUNT(*) INTO diff FROM (
        SELECT level_id, SUM(attempts), SUM(wins),
               SUM(duration_sum), SUM(wave_sum)
          FROM dashboard_battle_stats
         GROUP BY level_id
        EXCEPT
        SELECT level_id,
               COUNT(*)::bigint,
               (COUNT(*) FILTER (WHERE result = 'win'))::bigint,
               COALESCE(SUM(duration_ms), 0)::bigint,
               COALESCE(SUM(wave_reached), 0)::bigint
          FROM battle_records
         GROUP BY level_id
    ) d;
    diff := diff + (
        SELECT COUNT(*) FROM (
            SELECT level_id,
                   COUNT(*)::bigint,
                   (COUNT(*) FILTER (WHERE result = 'win'))::bigint,
                   COALESCE(SUM(duration_ms), 0)::bigint,
                   COALESCE(SUM(wave_reached), 0)::bigint
              FROM battle_records
             GROUP BY level_id
            EXCEPT
            SELECT level_id, SUM(attempts), SUM(wins),
                   SUM(duration_sum), SUM(wave_sum)
              FROM dashboard_battle_stats
             GROUP BY level_id
        ) d
    );
    IF diff <> 0 THEN
        RAISE EXCEPTION
            'perf_seed 校验失败：dashboard_battle_stats 与 battle_records 直接聚合有 % 行不一致',
            diff;
    END IF;

    -- ② 反应计数（按 level_id + reaction）
    SELECT COUNT(*) INTO diff FROM (
        SELECT level_id, reaction, SUM(count)
          FROM dashboard_reaction_counts
         GROUP BY level_id, reaction
        EXCEPT
        SELECT b.level_id, e.key, SUM(e.value::bigint)::bigint
          FROM battle_records b,
               LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
         WHERE jsonb_typeof(b.reactions_used) = 'object'
         GROUP BY b.level_id, e.key
    ) d;
    diff := diff + (
        SELECT COUNT(*) FROM (
            SELECT b.level_id, e.key, SUM(e.value::bigint)::bigint
              FROM battle_records b,
                   LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
             WHERE jsonb_typeof(b.reactions_used) = 'object'
             GROUP BY b.level_id, e.key
            EXCEPT
            SELECT level_id, reaction, SUM(count)
              FROM dashboard_reaction_counts
             GROUP BY level_id, reaction
        ) d
    );
    IF diff <> 0 THEN
        RAISE EXCEPTION
            'perf_seed 校验失败：dashboard_reaction_counts 与 battle_records 直接聚合有 % 行不一致',
            diff;
    END IF;

    RAISE NOTICE 'perf_seed 自校验通过：两张增量表与直接聚合完全一致';
END $$;

-- 恢复逐行触发器（后续单条写入仍会正常累加）。
ALTER TABLE battle_records ENABLE TRIGGER USER;

COMMIT;

-- 事务外确认触发器确实恢复为启用（防止「关了忘开」的静默状态）。
DO $$
DECLARE
    off_count INT;
BEGIN
    SELECT COUNT(*) INTO off_count
      FROM pg_trigger
     WHERE tgrelid = 'battle_records'::regclass
       AND NOT tgisinternal
       AND tgname IN ('trg_dashboard_battle_stats', 'trg_dashboard_reaction_counts')
       AND tgenabled <> 'O';
    IF off_count <> 0 THEN
        RAISE EXCEPTION 'perf_seed: 导入结束后仍有 % 个增量触发器处于关闭状态', off_count;
    END IF;
END $$;

SELECT 'users' AS t, COUNT(*) AS n FROM users
UNION ALL SELECT 'battle_records', COUNT(*) FROM battle_records
UNION ALL SELECT 'user_progress', COUNT(*) FROM user_progress
UNION ALL SELECT 'level_stars', COUNT(*) FROM level_stars
UNION ALL SELECT 'dashboard_battle_stats', COUNT(*) FROM dashboard_battle_stats
UNION ALL SELECT 'dashboard_reaction_counts', COUNT(*) FROM dashboard_reaction_counts
ORDER BY 1;
