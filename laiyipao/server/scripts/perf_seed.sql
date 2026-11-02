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
--   battle_records 1,000,000  每用户恰好 1 条；level_id=1+(id%100)；胜率 80%。
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

INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
SELECT 'perf_' || n,
       'P' || n,
       TRUE,
       1,
       now() - ((n % 9000)::float / 9000) * interval '90 days',
       now() - ((n % 3000)::float / 3000) * interval '30 days'
FROM generate_series(1, 1000000) AS n;

INSERT INTO battle_records (user_id, level_id, result, stars, score, kills, leaked,
                            wave_reached, duration_ms, shots, hits, created_at)
SELECT n,
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
       now() - ((n % 9000)::float / 9000) * interval '90 days'
FROM generate_series(1, 1000000) AS n;

INSERT INTO user_progress (user_id, max_stage, level_exp, total_battles, total_kills)
SELECT n, n % 100, (n % 5000)::bigint, 1, n % 500
FROM generate_series(1, 1000000) AS n;

INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
SELECT n, 1 + (n % 100), 1 + (n % 3), (n % 100000)::bigint, n % 3, (n % 2000)::bigint
FROM generate_series(1, 1000000) AS n;

-- 让优化器拿到真实统计信息，否则 EXPLAIN/实际计划会低估 100 万行。
ANALYZE users;
ANALYZE battle_records;
ANALYZE user_progress;
ANALYZE level_stars;

SELECT 'users' AS t, COUNT(*) AS n FROM users
UNION ALL SELECT 'battle_records', COUNT(*) FROM battle_records
UNION ALL SELECT 'user_progress', COUNT(*) FROM user_progress
UNION ALL SELECT 'level_stars', COUNT(*) FROM level_stars
ORDER BY 1;
