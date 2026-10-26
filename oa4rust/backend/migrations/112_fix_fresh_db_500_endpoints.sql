-- 112_fix_fresh_db_500_endpoints.sql
-- 第三轮 P0：收口一批「handler SQL 引用、但 schema 里没有」的 fresh-DB 必 500 端点。
--
-- 来源：2026-10-26 只读勘察 + CI run 40262029237 的 PG 容器日志在线实证
-- （plan: docs/plans/2026-10-26-fresh-db-500-closure-round.md，F-1/F-3/F-5/F-7）：
--
--   F-1  process_bam/src/lib.rs:20-23,66      x_process_bam.status   （列不存在）
--   F-5  program_center/src/lib.rs:3838       x_program_field.sort_order（列不存在，仅 ORDER BY）
--   F-3  processplatform_assemble_surface/src/lib.rs:887,975
--        "PP_E_APPLICATION" 大写视图从未被任何生效中的迁移创建——
--        唯一来源 archive/089_fix_500_endpoints.sql.disabled（27335205f 因
--        "views over nonexistent uppercase tables" 被 archive，runner 排除
--        archive，从未应用；当年只把"列"手工应用到各库，视图没有）
--   P0-4 遗留表 4 张（FI002-LEGACY-TABLES 收尾）：3× pp_c_touch_*（handler 只
--        UPDATE xupdatetime）+ x_process_transition（gateway_fork 读 4 列 +
--        WHERE 2 列）。migrations 从不建这些表（Java 时代遗留）。
--
-- 类型判据（沿用 111 的「代码绑定」口径，无活库条件下把编译期已知信息写进 DDL）：
--   与 'running'/'completed'/'expired' 等字符串字面量比较的列        → TEXT
--   仅出现在 ORDER BY 的列（值不被读取）                             → INTEGER（Java sort_order 惯例）
--   handler 以 String 读取/绑定的列                                  → TEXT
--   IS NULL 比较的时间列                                            → TIMESTAMP
--
-- ⚠️ live DB 已有这些列/表/视图时（Java 时代或 089 手工应用的库），
-- ADD COLUMN IF NOT EXISTS / CREATE TABLE IF NOT EXISTS / pg_class 守卫
-- 全部是 no-op——不覆盖既有形状，这正是想要的。
--
-- 每条语句前以 TO_REGCLASS / pg_class 判存在（沿用 101/111 范式），
-- 避免个别环境缺表时整批失败。
-- Rollback file: 112_fix_fresh_db_500_endpoints_rollback.sql

DO $$
BEGIN

-- ── F-1 x_process_bam.status ─────────────────────────────────────
-- process_bam/src/lib.rs:20-23（state_summary 三个 FILTER）与 :66（state_running）：
--   COUNT(*) FILTER (WHERE status = 'running' AND deleted_at IS NULL) ...
--   WHERE status = 'running' AND deleted_at IS NULL
-- 036:353-375 建的 x_process_bam 只有 running/completed/expired/total 计数器列，无 status。
-- CI PG 日志实证：column "status" does not exist。
IF TO_REGCLASS('public.x_process_bam') IS NOT NULL THEN
    ALTER TABLE "x_process_bam" ADD COLUMN IF NOT EXISTS "status" TEXT;
END IF;

-- ── F-5 x_program_field.sort_order ───────────────────────────────
-- program_center/src/lib.rs:3838：ORDER BY entity, sort_order。
-- 该列只进 ORDER BY、不被 SELECT/读取 ⇒ 类型无代码约束，按 Java sort_order
-- 命名惯例取 INTEGER；fresh DB 全 NULL 时 ASC 排后（PG NULLS LAST），语义诚实。
-- CI PG 日志实证：column "sort_order" does not exist。
IF TO_REGCLASS('public.x_program_field') IS NOT NULL THEN
    ALTER TABLE "x_program_field" ADD COLUMN IF NOT EXISTS "sort_order" INTEGER;
END IF;

-- ── F-3 "PP_E_APPLICATION" 大写视图 ──────────────────────────────
-- processplatform_assemble_surface/src/lib.rs:887（application_list_key_key，
--   WHERE xkey = $1）与 :975（application_list_terminal_terminal，WHERE xterminal = $1）。
-- 定义逐字照搬 archive/089_fix_500_endpoints.sql.disabled:6-7 的 PP_E_APPLICATION
-- （混合大小写列清单已与 032:700-724 的 DDL 逐列核对一致；xkey/xterminal 均映射 xid）。
--
-- ⚠️ 刻意不用 CREATE OR REPLACE：089 当年可能被手工应用到某些库、定义未知，
-- replace 有覆盖风险。只在「视图不存在」时创建（pg_class relname + relkind='v'）。
-- PostgreSQL 对带引号大写标识符大小写敏感：relname 存 'PP_E_APPLICATION'。
IF NOT EXISTS (SELECT 1 FROM pg_class WHERE relname = 'PP_E_APPLICATION' AND relkind = 'v') THEN
    CREATE VIEW "PP_E_APPLICATION" AS
        SELECT xid, xname, xalias, xdescription, "xapplicationCategory", xicon, "xiconHue", "xcreatorPerson", "xlastUpdateTime", "xlastUpdatePerson", xproperties, "xcreateTime", "xupdateTime", xid AS xkey, xid AS xterminal FROM "pp_e_application";
END IF;

-- ── P0-4 遗留表：3× pp_c_touch_*（FI002-LEGACY-TABLES）────────────
-- processplatform_assemble_surface/src/lib.rs:20311,20331,20351（touch_expire /
-- touch_passexpired / touch_touchdetained）：整表 UPDATE ... SET xupdatetime = NOW()。
-- 只建 handler 实际引用的最小形状（不照抄 archive/089 的 id + xupdateTime 旧驼峰
-- 形状——现 handler 已改小写 xupdatetime，照抄建了也不解决 500）。
-- Java 时代这些表在 live 库已存在且有行：IF NOT EXISTS 是 no-op，行为不变；
-- fresh DB 建成空表 ⇒ 三个端点从 500 变 200（count=0，诚实）。
CREATE TABLE IF NOT EXISTS "pp_c_touch_expire" (
    "xupdatetime" TIMESTAMP
);
CREATE TABLE IF NOT EXISTS "pp_c_touch_passexpired" (
    "xupdatetime" TIMESTAMP
);
CREATE TABLE IF NOT EXISTS "pp_c_touch_detained" (
    "xupdatetime" TIMESTAMP
);

-- ── P0-4 遗留表：x_process_transition（FI002-LEGACY-TABLES）───────
-- processplatform_service_processing/src/lib.rs:3916（gateway_fork，admin 门）：
--   SELECT id, work_id, to_activity, condition FROM x_process_transition
--   WHERE gateway_instance = $1 AND deleted_at IS NULL
-- 六个列全部来自 handler：id（PK）/ work_id / to_activity（String 读取）、
-- condition（SELECT 但不在循环里读取，取 TEXT）、gateway_instance（$1 String 绑定）、
-- deleted_at（IS NULL 比较 ⇒ TIMESTAMP）。索引按 WHERE 谓词建（gateway_instance）。
CREATE TABLE IF NOT EXISTS "x_process_transition" (
    "id" TEXT PRIMARY KEY,
    "work_id" TEXT,
    "to_activity" TEXT,
    "condition" TEXT,
    "gateway_instance" TEXT,
    "deleted_at" TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_x_process_transition_gateway_instance
    ON "x_process_transition" ("gateway_instance");

END $$;
