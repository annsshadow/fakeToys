-- plan002 U2: bbs_assemble_control endpoint full closure (Java x_bbs_assemble_control 对齐).
-- Additive columns for flag/toggle endpoints on existing tables, plus new tables
-- for domains that had no Rust storage yet (attachment / config setting / role /
-- permission / vote record / user info).
-- Idempotent: safe to run repeatedly (follows 064_cms_u2_columns.sql precedent).
-- Rollback file: 067_bbs_u2_tables_rollback.sql

-- ── 既有表补列 ──────────────────────────────────────────────────────────
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "is_cream" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "is_original" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "is_recommend" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "top_to_bbs" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "top_to_forum" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "top_to_main_section" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "top_to_section" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "locked" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "completed" BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "accept_reply_id" VARCHAR(255);
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "vote_count" INTEGER NOT NULL DEFAULT 0;
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "section_name" VARCHAR(255);
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "subject_type" VARCHAR(255);
ALTER TABLE "x_bbs_topic" ADD COLUMN IF NOT EXISTS "subject_grade" VARCHAR(255);

ALTER TABLE "x_bbs_reply" ADD COLUMN IF NOT EXISTS "accepted" BOOLEAN NOT NULL DEFAULT false;

ALTER TABLE "x_bbs_shutup" ADD COLUMN IF NOT EXISTS "reason" TEXT;

ALTER TABLE "x_bbs_section" ADD COLUMN IF NOT EXISTS "parent_id" VARCHAR(255);

-- ── 新表 ────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS "x_bbs_attachment" (
    "id" VARCHAR(255) PRIMARY KEY,
    "subject_id" VARCHAR(255),
    "name" TEXT,
    "extension" TEXT,
    "url" TEXT,
    "description" TEXT,
    "content" BYTEA,
    "length" BIGINT DEFAULT 0,
    "creator" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP WITHOUT TIME ZONE,
    "deleted_at" TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS "x_bbs_subject_attachment" (
    "id" VARCHAR(255) PRIMARY KEY,
    "subject_id" VARCHAR(255),
    "name" TEXT,
    "description" TEXT,
    "url" TEXT,
    "content" BYTEA,
    "length" BIGINT DEFAULT 0,
    "creator" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP WITHOUT TIME ZONE,
    "deleted_at" TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS "x_bbs_config_setting" (
    "id" VARCHAR(255) PRIMARY KEY,
    "name" TEXT,
    "code" VARCHAR(255),
    "value" TEXT,
    "description" TEXT,
    "creator" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP WITHOUT TIME ZONE,
    "deleted_at" TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS "x_bbs_role" (
    "id" VARCHAR(255) PRIMARY KEY,
    "name" TEXT,
    "code" VARCHAR(255),
    "description" TEXT,
    "forum_id" VARCHAR(255),
    "section_id" VARCHAR(255),
    "creator" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP WITHOUT TIME ZONE,
    "deleted_at" TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS "x_bbs_role_bind" (
    "id" VARCHAR(255) PRIMARY KEY,
    "role_id" VARCHAR(255),
    "object_type" VARCHAR(64),
    "object_code" VARCHAR(255),
    "object_name" TEXT,
    "creator" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_bbs_permission" (
    "id" VARCHAR(255) PRIMARY KEY,
    "code" VARCHAR(255),
    "name" TEXT,
    "forum_id" VARCHAR(255),
    "section_id" VARCHAR(255),
    "role_code" VARCHAR(255),
    "max_reply" BIGINT DEFAULT -1,
    "publish" BOOLEAN NOT NULL DEFAULT true,
    "reply" BOOLEAN NOT NULL DEFAULT true,
    "visible" BOOLEAN NOT NULL DEFAULT true,
    "creator" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS "x_bbs_vote_record" (
    "id" VARCHAR(255) PRIMARY KEY,
    "topic_id" VARCHAR(255),
    "person" VARCHAR(255),
    "option_id" VARCHAR(255),
    "option_name" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_bbs_user_info" (
    "id" VARCHAR(255) PRIMARY KEY,
    "person" VARCHAR(255),
    "nick_name" TEXT,
    "icon" TEXT,
    "signature" TEXT,
    "create_time" TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP WITHOUT TIME ZONE,
    "deleted_at" TIMESTAMP WITHOUT TIME ZONE
);

-- BBS_NAME 种子行（getBBSName 端点数据源）
INSERT INTO "x_bbs_config_setting" ("id", "name", "code", "value")
VALUES ('bbs-setting-bbs-name', 'BBS名称', 'BBS_NAME', 'O2社区')
ON CONFLICT ("id") DO NOTHING;
