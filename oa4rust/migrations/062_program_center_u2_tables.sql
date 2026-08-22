-- plan002 U2: program_center missing-endpoint tables.
-- x_program_warn_log  -> jaxrs/warnlog family (POST /warnlog, list/next/prev, view/system/log/tag)
-- x_program_app_pack  -> jaxrs/apppack family (pack info / publish / download / android build)
-- Additive columns fix latent schema gaps hit by existing routed reads:
--   x_program_script.content (script_flag / script_id SELECT it)
--   x_program_dict.flag      (dict_dictFlag_path_data filters on it; dict_data_write updates it)
-- Idempotent: safe to run repeatedly.

CREATE TABLE IF NOT EXISTS "x_program_warn_log" (
    "id" TEXT PRIMARY KEY,
    "level" TEXT,
    "tag" TEXT,
    "logger_name" TEXT,
    "message" TEXT,
    "detail" TEXT,
    "host" TEXT,
    "port" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT
);

CREATE INDEX IF NOT EXISTS "idx_x_program_warn_log_tag"
    ON "x_program_warn_log" ("tag");

CREATE TABLE IF NOT EXISTS "x_program_app_pack" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT,
    "version" TEXT,
    "status" TEXT DEFAULT 'draft',
    "file_name" TEXT,
    "file_path" TEXT,
    "description" TEXT,
    "config_json" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);

CREATE INDEX IF NOT EXISTS "idx_x_program_app_pack_status"
    ON "x_program_app_pack" ("status");

ALTER TABLE IF EXISTS "x_program_script"
    ADD COLUMN IF NOT EXISTS "content" TEXT;

ALTER TABLE IF EXISTS "x_program_dict"
    ADD COLUMN IF NOT EXISTS "flag" TEXT;
