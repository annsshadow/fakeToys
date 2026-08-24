-- plan002 U2：calendar_assemble_control 与 mind_assemble_control 端点闭合
-- 所需的缺列/缺表补齐（对照 x_calendar_assemble_control / x_mind_assemble_control jaxrs 全集）。
--
-- 全部幂等（IF NOT EXISTS / ADD COLUMN IF NOT EXISTS / DROP COLUMN IF EXISTS），
-- 可重复执行；回滚见 085_calendar_mind_u2_columns_rollback.sql。
--
-- calendar 域新增：
--   cal_calendar_follow  关注关系（follow / follow/cancel）
--   cal_event.master_id  事件重复实例分组（update/after、update/all、delete/after、delete/all）
--   cal_event.rfc_text   RFC2445 文本（event/rfc/{id}）
--   cal_message          POST /message 留言
-- mind 域新增（x_mind 缺列）：
--   parent_id / folder_id / icon / description / shared / file_version / creator_unit
--   x_mind_share         共享记录（share / share/cancel / list/{id}/shareRecords）
--   x_mind_version_info  已存在，本迁移仅确保可用（不重建）

-- ── calendar：关注关系 ───────────────────────────────────────────────
ALTER TABLE "cal_calendar" ADD COLUMN IF NOT EXISTS "follow_enabled" BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS "cal_calendar_follow" (
    "id"          TEXT PRIMARY KEY,
    "calendar_id" TEXT NOT NULL,
    "person"      TEXT NOT NULL,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX IF NOT EXISTS "uq_cal_calendar_follow" ON "cal_calendar_follow" ("calendar_id", "person");

-- ── calendar：事件重复分组 + RFC 文本 ─────────────────────────────────
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "master_id" TEXT;
ALTER TABLE "cal_event" ADD COLUMN IF NOT EXISTS "rfc_text" TEXT;

-- ── calendar：留言 ───────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS "cal_message" (
    "id"          TEXT PRIMARY KEY,
    "calendar_id" TEXT,
    "title"       TEXT,
    "body"        TEXT,
    "person"      TEXT,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW()
);

-- ── mind：x_mind 缺列补齐 ────────────────────────────────────────────
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "parent_id"     TEXT;
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "folder_id"     TEXT;
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "icon"          TEXT;
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "description"   TEXT;
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "shared"       BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "file_version" BIGINT NOT NULL DEFAULT 0;
ALTER TABLE "x_mind" ADD COLUMN IF NOT EXISTS "creator_unit" TEXT;

-- ── mind：共享记录 ───────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS "x_mind_share" (
    "id"       TEXT PRIMARY KEY,
    "mind_id"  TEXT NOT NULL,
    "person"   TEXT NOT NULL,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX IF NOT EXISTS "uq_x_mind_share" ON "x_mind_share" ("mind_id", "person");

-- x_mind_version_info 已在 029 创建，这里仅保活（无操作占位，便于阅读）。
