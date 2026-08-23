-- plan002 U2: 四小模块端点闭合共用 migration（program_init / base / calendar_assemble_control / program_center）。
--
-- 1) init_external_datasource  -> program_init jaxrs/externaldatasources（set/list/check/validate/set-cancel）
-- 2) init_server_command       -> program_init jaxrs/server（execute / execute/status / stop）
-- 3) init_restore_upload       -> program_init jaxrs/restore（upload / upload/cancel）
-- 4) x_program_invoke          -> program_center invoke CRUD（POST/GET/PUT/DELETE /invoke）
-- 5) cal_setting               -> calendar_assemble_control setting 域（list/all、{id}）
--
-- 全部幂等（IF NOT EXISTS），可重复执行；回滚见 074_u2_small_modules_rollback.sql。

-- ── 1) 外部数据源配置（对齐 Java ExternalDataSource 字段子集）──────────────
CREATE TABLE IF NOT EXISTS "init_external_datasource" (
    "id"         VARCHAR(36) PRIMARY KEY,
    "name"       TEXT NOT NULL,
    "unique_name" TEXT,
    "enable"     BOOLEAN NOT NULL DEFAULT TRUE,
    "host"       TEXT,
    "port"       INTEGER,
    "database_name" TEXT,
    "user_name"  TEXT,
    "password"   TEXT,
    "url"        TEXT,
    "applied"    BOOLEAN NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS "idx_init_ext_ds_applied" ON "init_external_datasource" ("applied");
CREATE UNIQUE INDEX IF NOT EXISTS "uq_init_ext_ds_name" ON "init_external_datasource" ("name");

-- ── 2) 初始化服务命令记录（execute/stop + 状态查询）───────────────────────
CREATE TABLE IF NOT EXISTS "init_server_command" (
    "id"              VARCHAR(36) PRIMARY KEY,
    "command"         TEXT NOT NULL CHECK ("command" IN ('execute', 'stop')),
    "status"          TEXT NOT NULL DEFAULT 'pending'
                      CHECK ("status" IN ('pending', 'running', 'completed', 'failed')),
    "messages"        JSONB NOT NULL DEFAULT '[]'::jsonb,
    "failure_message" TEXT,
    "creator"         TEXT,
    "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at"      TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS "idx_init_server_command_created" ON "init_server_command" ("created_at" DESC);

-- ── 3) 恢复上传（stamp 对齐 Java DateTools compact 时间戳命名）────────────
CREATE TABLE IF NOT EXISTS "init_restore_upload" (
    "id"         VARCHAR(36) PRIMARY KEY,
    "stamp"      TEXT NOT NULL UNIQUE,
    "file_path"  TEXT,
    "size_bytes" BIGINT NOT NULL DEFAULT 0,
    "status"     TEXT NOT NULL DEFAULT 'uploaded'
                 CHECK ("status" IN ('uploaded', 'cancelled', 'applied')),
    "creator"    TEXT,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS "idx_init_restore_upload_status" ON "init_restore_upload" ("status");

-- ── 4) 服务调用（Invoke 实体，字段对齐 x_program_center_core_entity.Invoke）──
CREATE TABLE IF NOT EXISTS "x_program_invoke" (
    "id"                VARCHAR(255) PRIMARY KEY,
    "name"              TEXT NOT NULL,
    "alias"             TEXT,
    "category"          TEXT,
    "description"       TEXT,
    "enable"            BOOLEAN NOT NULL DEFAULT FALSE,
    "enable_token"      BOOLEAN NOT NULL DEFAULT FALSE,
    "enable_anonymous"  BOOLEAN NOT NULL DEFAULT TRUE,
    "validated"         BOOLEAN NOT NULL DEFAULT FALSE,
    "text"              TEXT,
    "remote_addr_regex" TEXT,
    "data"              TEXT,
    "executor_list"     JSONB NOT NULL DEFAULT '[]'::jsonb,
    "last_start_time"   TIMESTAMP,
    "last_end_time"     TIMESTAMP,
    "creator"           TEXT,
    "create_time"       TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time"       TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS "uq_program_invoke_name" ON "x_program_invoke" ("name");
CREATE INDEX IF NOT EXISTS "idx_program_invoke_category" ON "x_program_invoke" ("category");

-- ── 5) 日历设置（CalendarSetting 实体）────────────────────────────────────
CREATE TABLE IF NOT EXISTS "cal_setting" (
    "id"          VARCHAR(255) PRIMARY KEY,
    "code"        TEXT NOT NULL,
    "name"        TEXT,
    "description" TEXT,
    "value"       TEXT,
    "order_no"    INTEGER NOT NULL DEFAULT 0,
    "person"      TEXT,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time" TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS "uq_cal_setting_code" ON "cal_setting" ("code");
