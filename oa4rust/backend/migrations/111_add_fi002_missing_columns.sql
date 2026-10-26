-- 111_add_fi002_missing_columns.sql
-- FI-002 第二轮：把「handler SQL 引用、但 migrations 从未建」的 41 个列幂等补齐。
--
-- 来源：scripts/fi002_column_scan.py（2026-10-25 R2 固化的静态对撞尺）首跑读数。
-- 那轮把这 41 对登记进 docs/audits/artifacts/fi002_unknown_columns.tsv 基线，
-- 标「待有库轮次按 live DB 类型补迁移」。本轮（R8）在没有活库的条件下，
-- 改以**代码绑定类型**为证据定类型 —— 每条 ALTER 上方都写了出处行号：
--
--   绑定到 String/&str/Option<String> 的列 → TEXT
--   用 `Value::Bool(row.get(...))` 读的列 → BOOLEAN（全批仅 x_program_module.vip）
--   INSERT 里由 NOW() 赋值的列 → TIMESTAMP（全批仅 x_meeting_checkin.checkin_time）
--
-- ⚠️ 为什么类型可以这样定：tokio-postgres 的 `ToSql` 与 `FromSql` 都按 Rust 侧类型
-- 选线格式，绑定 String 的列若建成 INTEGER 会当场类型不匹配；反过来 TEXT 对
-- 任何字符串绑定都成立。所以「按绑定类型定」不是猜，是把编译期已知的信息写进 DDL。
--
-- ⚠️ 仍有一处**无法静态判定**：live DB 若已有该列（历史手工 ALTER），
-- ADD COLUMN IF NOT EXISTS 是 no-op，不会覆盖既有类型 —— 这正是想要的。
--
-- 列名大小写：`"xtrackingNumber"` 在代码里是**带引号**引用（PostgreSQL 引号标识符
-- 大小写敏感），故迁移里必须同名；其余列代码均不加引号，按小写建。
--
-- 每条 ALTER 前以 TO_REGCLASS 判表存在（沿用 101 的范式），避免个别环境缺表时整批失败。
-- Rollback file: 111_add_fi002_missing_columns_rollback.sql

DO $$
BEGIN

-- ── auth_person_identity ──────────────────────────────────────
IF TO_REGCLASS('public.auth_person_identity') IS NOT NULL THEN
    -- auth/src/lib.rs:350 `WHERE pi.person_id = $1`（person_id 为 String）
    ALTER TABLE "auth_person_identity" ADD COLUMN IF NOT EXISTS "person_id" TEXT;
END IF;

-- ── pp_c_draft ────────────────────────────────────────────────
IF TO_REGCLASS('public.pp_c_draft') IS NOT NULL THEN
    -- processplatform_assemble_surface:7712 `SET xstatus = $1`
    ALTER TABLE "pp_c_draft" ADD COLUMN IF NOT EXISTS "xstatus" TEXT;
END IF;

-- ── pp_c_keylock ──────────────────────────────────────────────
IF TO_REGCLASS('public.pp_c_keylock') IS NOT NULL THEN
    -- :2916 SELECT 后 `Value::String(row.get("xidentity"))`
    ALTER TABLE "pp_c_keylock" ADD COLUMN IF NOT EXISTS "xidentity" TEXT;
    -- :2916 同上
    ALTER TABLE "pp_c_keylock" ADD COLUMN IF NOT EXISTS "xtitle" TEXT;
END IF;

-- ── pp_e_file ─────────────────────────────────────────────────
IF TO_REGCLASS('public.pp_e_file') IS NOT NULL THEN
    -- processplatform_assemble_designer:1139 `let content: Option<String>`
    ALTER TABLE "pp_e_file" ADD COLUMN IF NOT EXISTS "xcontent" TEXT;
END IF;

-- ── pp_e_script ───────────────────────────────────────────────
IF TO_REGCLASS('public.pp_e_script') IS NOT NULL THEN
    -- processplatform_assemble_designer:3195 `row_to_script_data` 读 String
    ALTER TABLE "pp_e_script" ADD COLUMN IF NOT EXISTS "xcode" TEXT;
END IF;

-- ── x_attendance_appeal_info ──────────────────────────────────
IF TO_REGCLASS('public.x_attendance_appeal_info') IS NOT NULL THEN
    -- attendance/src/lib.rs:486 INSERT 绑定 `&appeal_date`（String）
    ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "appeal_date" TEXT;
    -- attendance/src/lib.rs:486 INSERT 绑定 `&reason`（String）
    ALTER TABLE "x_attendance_appeal_info" ADD COLUMN IF NOT EXISTS "reason" TEXT;
END IF;

-- ── x_attendance_statistic_dd_unit_day ────────────────────────
IF TO_REGCLASS('public.x_attendance_statistic_dd_unit_day') IS NOT NULL THEN
    -- attendance_assemble_control:6840 `row.get::<_, Option<String>>("o2_user")`
    ALTER TABLE "x_attendance_statistic_dd_unit_day" ADD COLUMN IF NOT EXISTS "o2_user" TEXT;
END IF;

-- ── x_attendance_statistic_dd_unit_month ──────────────────────
IF TO_REGCLASS('public.x_attendance_statistic_dd_unit_month') IS NOT NULL THEN
    -- 同族表，与 _day 同一套 `stat_row_to_value` 读取
    ALTER TABLE "x_attendance_statistic_dd_unit_month" ADD COLUMN IF NOT EXISTS "o2_user" TEXT;
END IF;

-- ── x_bbs_section ─────────────────────────────────────────────
IF TO_REGCLASS('public.x_bbs_section') IS NOT NULL THEN
    -- bbs_assemble_control/u2.rs:1557 INSERT 绑定 `&creator`（String）
    ALTER TABLE "x_bbs_section" ADD COLUMN IF NOT EXISTS "creator" TEXT;
END IF;

-- ── x_bbs_subject_attachment ──────────────────────────────────
IF TO_REGCLASS('public.x_bbs_subject_attachment') IS NOT NULL THEN
    -- bbs_assemble_control/u2.rs:2480 `row_opt_json::<String>(row, "extension")`
    ALTER TABLE "x_bbs_subject_attachment" ADD COLUMN IF NOT EXISTS "extension" TEXT;
END IF;

-- ── x_cms_index ───────────────────────────────────────────────
IF TO_REGCLASS('public.x_cms_index') IS NOT NULL THEN
    -- 同族 x_cms_column.status 先例（093 建表 `TEXT DEFAULT 'active'`）
    ALTER TABLE "x_cms_index" ADD COLUMN IF NOT EXISTS "status" TEXT DEFAULT 'active';
END IF;

-- ── x_cms_viewfieldconfig ─────────────────────────────────────
IF TO_REGCLASS('public.x_cms_viewfieldconfig') IS NOT NULL THEN
    -- cms_assemble_control:7398 `Value::String(row.get("creator"))`
    ALTER TABLE "x_cms_viewfieldconfig" ADD COLUMN IF NOT EXISTS "creator" TEXT;
END IF;

-- ── x_console_message ─────────────────────────────────────────
IF TO_REGCLASS('public.x_console_message') IS NOT NULL THEN
    -- console/src/lib.rs:186 INSERT 绑定 `&token`（String）
    ALTER TABLE "x_console_message" ADD COLUMN IF NOT EXISTS "xtoken" TEXT;
END IF;

-- ── x_express_subscribe ───────────────────────────────────────
IF TO_REGCLASS('public.x_express_subscribe') IS NOT NULL THEN
    -- express/src/lib.rs:136 INSERT 绑定 `&callback`（String）
    ALTER TABLE "x_express_subscribe" ADD COLUMN IF NOT EXISTS "xcallback" TEXT;
    -- express/src/lib.rs:136 INSERT 绑定 `&company`（String）
    ALTER TABLE "x_express_subscribe" ADD COLUMN IF NOT EXISTS "xcompany" TEXT;
    -- express/src/lib.rs:135 **带引号混合大小写**，必须同名建列
    ALTER TABLE "x_express_subscribe" ADD COLUMN IF NOT EXISTS "xtrackingNumber" TEXT;
END IF;

-- ── x_general_assemble_excel ──────────────────────────────────
IF TO_REGCLASS('public.x_general_assemble_excel') IS NOT NULL THEN
    -- general_assemble_control:1312 INSERT 绑定 `&flag`（String）
    ALTER TABLE "x_general_assemble_excel" ADD COLUMN IF NOT EXISTS "flag" TEXT;
    -- general_assemble_control:1159 `Option<String>`
    ALTER TABLE "x_general_assemble_excel" ADD COLUMN IF NOT EXISTS "name" TEXT;
END IF;

-- ── x_general_assemble_office ─────────────────────────────────
IF TO_REGCLASS('public.x_general_assemble_office') IS NOT NULL THEN
    -- general_assemble_control:2008 INSERT 绑定 `&html_content`（String）
    ALTER TABLE "x_general_assemble_office" ADD COLUMN IF NOT EXISTS "html_content" TEXT;
END IF;

-- ── x_meeting_checkin ─────────────────────────────────────────
IF TO_REGCLASS('public.x_meeting_checkin') IS NOT NULL THEN
    -- meeting_assemble_control:2348 INSERT 绑定 `&id`（String）
    ALTER TABLE "x_meeting_checkin" ADD COLUMN IF NOT EXISTS "meeting_id" TEXT;
    -- meeting_assemble_control:2348 INSERT 绑定 `&person`（&str）
    ALTER TABLE "x_meeting_checkin" ADD COLUMN IF NOT EXISTS "person" TEXT;
    -- meeting_assemble_control:2347 `VALUES (..., NOW())`
    ALTER TABLE "x_meeting_checkin" ADD COLUMN IF NOT EXISTS "checkin_time" TIMESTAMP;
END IF;

-- ── x_portal ──────────────────────────────────────────────────
IF TO_REGCLASS('public.x_portal') IS NOT NULL THEN
    -- portal/src/lib.rs:80 `Option<String>`
    ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "config" TEXT;
    -- portal/src/lib.rs 同段 `Option<String>`
    ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "pages" TEXT;
END IF;

-- ── x_portal_layout ───────────────────────────────────────────
IF TO_REGCLASS('public.x_portal_layout') IS NOT NULL THEN
    -- portal_assemble_surface:584 `Value::String(row.get("category"))`
    ALTER TABLE "x_portal_layout" ADD COLUMN IF NOT EXISTS "category" TEXT;
    -- 同上
    ALTER TABLE "x_portal_layout" ADD COLUMN IF NOT EXISTS "content" TEXT;
    -- 同上
    ALTER TABLE "x_portal_layout" ADD COLUMN IF NOT EXISTS "name" TEXT;
END IF;

-- ── x_portal_page_version ─────────────────────────────────────
IF TO_REGCLASS('public.x_portal_page_version') IS NOT NULL THEN
    -- portal_assemble_designer:1984 `Option<String>`
    ALTER TABLE "x_portal_page_version" ADD COLUMN IF NOT EXISTS "content" TEXT;
END IF;

-- ── x_process_organization ────────────────────────────────────
IF TO_REGCLASS('public.x_process_organization') IS NOT NULL THEN
    -- process_bam:122 `SELECT o.name` 并 GROUP BY o.name
    ALTER TABLE "x_process_organization" ADD COLUMN IF NOT EXISTS "name" TEXT;
END IF;

-- ── x_program_callback_registration ───────────────────────────
IF TO_REGCLASS('public.x_program_callback_registration') IS NOT NULL THEN
    -- program_center:4786 INSERT 字面量 'dingding'
    ALTER TABLE "x_program_callback_registration" ADD COLUMN IF NOT EXISTS "source" TEXT;
END IF;

-- ── x_program_deploy ──────────────────────────────────────────
IF TO_REGCLASS('public.x_program_deploy') IS NOT NULL THEN
    -- program_center:4214 `Option<String>`
    ALTER TABLE "x_program_deploy" ADD COLUMN IF NOT EXISTS "status" TEXT;
END IF;

-- ── x_program_message_log ─────────────────────────────────────
IF TO_REGCLASS('public.x_program_message_log') IS NOT NULL THEN
    -- program_center:7822 INSERT 字面量 'qiyeweixin_getprivateinfo'
    ALTER TABLE "x_program_message_log" ADD COLUMN IF NOT EXISTS "message_type" TEXT;
    -- program_center:7823 INSERT 绑定 `&user_id`（String）
    ALTER TABLE "x_program_message_log" ADD COLUMN IF NOT EXISTS "user_id" TEXT;
END IF;

-- ── x_program_module ──────────────────────────────────────────
IF TO_REGCLASS('public.x_program_module') IS NOT NULL THEN
    -- program_center:5697 `Value::Bool(row.get("vip"))` —— 全批唯一布尔列
    ALTER TABLE "x_program_module" ADD COLUMN IF NOT EXISTS "vip" BOOLEAN DEFAULT false;
END IF;

-- ── x_program_output ──────────────────────────────────────────
IF TO_REGCLASS('public.x_program_output') IS NOT NULL THEN
    -- program_center:7078 `Option<String>`
    ALTER TABLE "x_program_output" ADD COLUMN IF NOT EXISTS "select_file" TEXT;
END IF;

-- ── x_program_script ──────────────────────────────────────────
IF TO_REGCLASS('public.x_program_script') IS NOT NULL THEN
    -- program_center:8580 `Option<String>`
    ALTER TABLE "x_program_script" ADD COLUMN IF NOT EXISTS "imported_content" TEXT;
END IF;

-- ── x_program_sync_log ────────────────────────────────────────
IF TO_REGCLASS('public.x_program_sync_log') IS NOT NULL THEN
    -- program_center:657 INSERT 字面量 'pull'
    ALTER TABLE "x_program_sync_log" ADD COLUMN IF NOT EXISTS "action" TEXT;
    -- program_center:4756 INSERT 绑定 `&event_type`（String）
    ALTER TABLE "x_program_sync_log" ADD COLUMN IF NOT EXISTS "event_type" TEXT;
    -- program_center:4756 INSERT 绑定 `&org_id`（String）
    ALTER TABLE "x_program_sync_log" ADD COLUMN IF NOT EXISTS "org_id" TEXT;
END IF;
END $$;
