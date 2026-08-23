-- plan002 U2 冲刺收尾：残余模块端点闭合共用 migration（organization_assemble_authentication /
-- personal / query_service_processing / correlation_service_processing）。
--
-- 1) x_org_bind_record         -> 认证 GET bind/list 扫码绑定记录（Java ORG_BIND 实体字段子集）
-- 2) x_org_definition          -> personal definition/{name} 用户自定义数据（Java Definition 实体）
-- 3) x_org_person_extend       -> personal exmail 被动读取（Java PersonExtend，type=exmail）
-- 4) x_org_empower_log 增列     -> personal empowerlog 族（Java EmpowerLog 字段补齐，072 已建基础表）
-- 5) x_correlation 增列/索引    -> correlation Java Correlation 实体字段对齐（from/target/site/view）
-- 6) x_query_index_state       -> query processing touch/reset/optimize/reload 任务状态
-- 7) x_query_index_extra       -> query processing index/update/extra/document 落库
-- 8) x_query_neural_job        -> query processing neural learn/generate/stop 任务记录
-- 9) x_query_table_data 增列   -> query processing table insert/updateWithBundle 按 bundle 定位行
--
-- 全部幂等（IF NOT EXISTS / ADD COLUMN IF NOT EXISTS），可重复执行；回滚见 077_u2_residual_modules_rollback.sql。

-- ── 1) 扫码绑定记录（bind/list）─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS "x_org_bind_record" (
    "id"          VARCHAR(36) PRIMARY KEY,
    "name"        TEXT NOT NULL,
    "message"     TEXT,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS "idx_x_org_bind_record_create" ON "x_org_bind_record" ("create_time");

-- ── 2) 用户自定义数据（Definition）──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS "x_org_definition" (
    "id"          VARCHAR(36) PRIMARY KEY,
    "name"        TEXT NOT NULL,
    "data"        TEXT,
    "creator"     TEXT,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time" TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 归一化查重：同一名称仅一行（新建表无历史脏数据，可直接唯一约束）
CREATE UNIQUE INDEX IF NOT EXISTS "uq_x_org_definition_name" ON "x_org_definition" ("name");

-- ── 3) 人员扩展属性（PersonExtend）──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS "x_org_person_extend" (
    "id"          VARCHAR(36) PRIMARY KEY,
    "person"      TEXT NOT NULL,
    "type"        TEXT NOT NULL,
    "extend"      JSONB NOT NULL DEFAULT '{}'::jsonb,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS "uq_x_org_person_extend_person_type"
    ON "x_org_person_extend" ("person", "type");

-- ── 4) 授权日志表补齐 Java EmpowerLog 字段（072 已建 x_org_empower_log）──────
ALTER TABLE "x_org_empower_log" ADD COLUMN IF NOT EXISTS "from_person" VARCHAR(255);
ALTER TABLE "x_org_empower_log" ADD COLUMN IF NOT EXISTS "to_person" VARCHAR(255);
ALTER TABLE "x_org_empower_log" ADD COLUMN IF NOT EXISTS "title" VARCHAR(255);
ALTER TABLE "x_org_empower_log" ADD COLUMN IF NOT EXISTS "empower_time" TIMESTAMP;
ALTER TABLE "x_org_empower_log" ADD COLUMN IF NOT EXISTS "activity" VARCHAR(255);
ALTER TABLE "x_org_empower_log" ADD COLUMN IF NOT EXISTS "activity_name" VARCHAR(255);
CREATE INDEX IF NOT EXISTS "idx_x_org_empower_log_from_person" ON "x_org_empower_log" ("from_person");
CREATE INDEX IF NOT EXISTS "idx_x_org_empower_log_to_person" ON "x_org_empower_log" ("to_person");
CREATE INDEX IF NOT EXISTS "idx_x_org_empower_log_title" ON "x_org_empower_log" ("title");

-- ── 5) 关联内容表对齐 Java Correlation 字段 ─────────────────────────────────
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "from_type" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "from_bundle" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "target_type" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "target_bundle" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "target_title" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "target_category" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "target_start_time" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "target_creator_person" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "site" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "view" TEXT;
ALTER TABLE "x_correlation" ADD COLUMN IF NOT EXISTS "person" TEXT;
CREATE INDEX IF NOT EXISTS "idx_x_correlation_from" ON "x_correlation" ("from_type", "from_bundle");
CREATE INDEX IF NOT EXISTS "idx_x_correlation_target" ON "x_correlation" ("target_type", "target_bundle");
CREATE INDEX IF NOT EXISTS "idx_x_correlation_from_site" ON "x_correlation" ("from_type", "from_bundle", "site");

-- ── 6) 索引任务状态（touch/reset/optimize/reload）───────────────────────────
CREATE TABLE IF NOT EXISTS "x_query_index_state" (
    "id"              VARCHAR(64) PRIMARY KEY,
    "entity_type"     TEXT NOT NULL,
    "freq"            TEXT,
    "node"            TEXT NOT NULL DEFAULT '0',
    "status"          TEXT NOT NULL DEFAULT 'idle',
    "error_message"   TEXT,
    "last_touch_time" TIMESTAMP,
    "last_reset_time" TIMESTAMP,
    "update_time"     TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS "uq_x_query_index_state"
    ON "x_query_index_state" ("entity_type", "freq", "node");

-- ── 7) 索引附加文档（index/update/extra/document）───────────────────────────
CREATE TABLE IF NOT EXISTS "x_query_index_extra" (
    "id"          VARCHAR(64) PRIMARY KEY,
    "type"        TEXT NOT NULL,
    "key"         TEXT NOT NULL,
    "doc_id"      TEXT NOT NULL,
    "data"        JSONB NOT NULL DEFAULT '{}'::jsonb,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS "uq_x_query_index_extra_doc"
    ON "x_query_index_extra" ("type", "key", "doc_id");

-- ── 8) 神经网络任务（neural learn/generate/stop）────────────────────────────
CREATE TABLE IF NOT EXISTS "x_query_neural_job" (
    "id"          VARCHAR(36) PRIMARY KEY,
    "model_flag"  TEXT NOT NULL,
    "action"      TEXT NOT NULL CHECK ("action" IN ('learn', 'generate')),
    "status"      TEXT NOT NULL DEFAULT 'running'
                  CHECK ("status" IN ('running', 'completed', 'stopped', 'failed')),
    "message"     TEXT,
    "create_time" TIMESTAMP NOT NULL DEFAULT NOW(),
    "update_time" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS "idx_x_query_neural_job_model"
    ON "x_query_neural_job" ("model_flag", "action", "status");

-- ── 9) 动态表数据行按 bundle 定位 ──────────────────────────────────────────
ALTER TABLE "x_query_table_data" ADD COLUMN IF NOT EXISTS "bundle" TEXT;
CREATE INDEX IF NOT EXISTS "idx_x_query_table_data_flag_bundle"
    ON "x_query_table_data" ("table_flag", "bundle");
