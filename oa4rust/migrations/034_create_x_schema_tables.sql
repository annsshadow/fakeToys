-- migration 034: create missing x_* tables referenced as X.<TABLE>
-- (schema 'X' does not exist; tables live in public as x_*).

CREATE TABLE IF NOT EXISTS "x_ai_clue" (
    "id" VARCHAR PRIMARY KEY,
    "clueId" VARCHAR,
    "clue_id" VARCHAR,
    "content" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "generateType" VARCHAR,
    "input" VARCHAR,
    "person" VARCHAR,
    "size" BIGINT,
    "title" VARCHAR,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_ai_clue" IS 'auto-created for parity (was X.AI_CLUE)';

CREATE TABLE IF NOT EXISTS "x_ai_completion" (
    "id" VARCHAR PRIMARY KEY,
    "create_time" TIMESTAMPTZ,
    "enabled" BOOLEAN,
    "name" VARCHAR,
    "provider" VARCHAR,
    "status" VARCHAR,
    "title" VARCHAR,
    "user_id" VARCHAR,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_ai_completion" IS 'auto-created for parity (was X.AI_COMPLETION)';

CREATE TABLE IF NOT EXISTS "x_ai_file" (
    "id" VARCHAR PRIMARY KEY,
    "xcreateTime" TIMESTAMPTZ,
    "xcreator" VARCHAR,
    "xid" VARCHAR,
    "xlength" VARCHAR,
    "xname" VARCHAR,
    "xstorage" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_ai_file" IS 'auto-created for parity (was X.AI_FILE)';

CREATE TABLE IF NOT EXISTS "x_ai_model" (
    "id" VARCHAR PRIMARY KEY,
    "apiKey" VARCHAR,
    "asDefault" VARCHAR,
    "completionUrl" VARCHAR,
    "desc" VARCHAR,
    "enable" BOOLEAN,
    "model" VARCHAR,
    "name" VARCHAR,
    "type" VARCHAR,
    "xenable" BOOLEAN,
    "xmodel" VARCHAR,
    "xname" VARCHAR,
    "xtype" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_ai_model" IS 'auto-created for parity (was X.AI_MODEL)';

CREATE TABLE IF NOT EXISTS "x_bam_config" (
    "id" VARCHAR PRIMARY KEY,
    "activity" VARCHAR,
    "application" VARCHAR,
    "category" VARCHAR,
    "completed" BOOLEAN,
    "end_time" TIMESTAMPTZ,
    "expired" BOOLEAN,
    "pending" BOOLEAN,
    "person" VARCHAR,
    "process" VARCHAR,
    "processing" BOOLEAN,
    "start_time" TIMESTAMPTZ,
    "task_status" VARCHAR,
    "title" VARCHAR,
    "total" BIGINT,
    "work_status" VARCHAR,
    "xcategory" VARCHAR,
    "xdefinition" VARCHAR,
    "xenabled" VARCHAR,
    "xid" VARCHAR,
    "xname" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_bam_config" IS 'auto-created for parity (was X.BAM_CONFIG)';

CREATE TABLE IF NOT EXISTS "x_cms_document" (
    "id" VARCHAR PRIMARY KEY,
    "xid" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_cms_document" IS 'auto-created for parity (was X.CMS_DOCUMENT)';

CREATE TABLE IF NOT EXISTS "x_console_cache" (
    "id" VARCHAR PRIMARY KEY,
    "token" VARCHAR,
    "xid" VARCHAR,
    "xlevel" VARCHAR,
    "xmessage" VARCHAR,
    "xstatus" VARCHAR,
    "xtimestamp" TIMESTAMPTZ,
    "xtoken" VARCHAR,
    "xunit" VARCHAR,
    "xuptime" VARCHAR,
    "xvalue" VARCHAR,
    "xversion" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_console_cache" IS 'auto-created for parity (was X.CONSOLE_CACHE)';

CREATE TABLE IF NOT EXISTS "x_console_log" (
    "id" VARCHAR PRIMARY KEY,
    "xlevel" VARCHAR,
    "xmessage" VARCHAR,
    "xstatus" VARCHAR,
    "xtimestamp" TIMESTAMPTZ,
    "xunit" VARCHAR,
    "xuptime" VARCHAR,
    "xvalue" VARCHAR,
    "xversion" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_console_log" IS 'auto-created for parity (was X.CONSOLE_LOG)';

CREATE TABLE IF NOT EXISTS "x_console_message" (
    "id" VARCHAR PRIMARY KEY,
    "xlevel" VARCHAR,
    "xmessage" VARCHAR,
    "xstatus" VARCHAR,
    "xtimestamp" TIMESTAMPTZ,
    "xunit" VARCHAR,
    "xuptime" VARCHAR,
    "xvalue" VARCHAR,
    "xversion" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_console_message" IS 'auto-created for parity (was X.CONSOLE_MESSAGE)';

CREATE TABLE IF NOT EXISTS "x_console_metric" (
    "id" VARCHAR PRIMARY KEY,
    "token" VARCHAR,
    "xid" VARCHAR,
    "xlevel" VARCHAR,
    "xmessage" VARCHAR,
    "xstatus" VARCHAR,
    "xtimestamp" TIMESTAMPTZ,
    "xtoken" VARCHAR,
    "xunit" VARCHAR,
    "xuptime" VARCHAR,
    "xvalue" VARCHAR,
    "xversion" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_console_metric" IS 'auto-created for parity (was X.CONSOLE_METRIC)';

CREATE TABLE IF NOT EXISTS "x_console_status" (
    "id" VARCHAR PRIMARY KEY,
    "xlevel" VARCHAR,
    "xmessage" VARCHAR,
    "xstatus" VARCHAR,
    "xtimestamp" TIMESTAMPTZ,
    "xunit" VARCHAR,
    "xuptime" VARCHAR,
    "xvalue" VARCHAR,
    "xversion" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_console_status" IS 'auto-created for parity (was X.CONSOLE_STATUS)';

CREATE TABLE IF NOT EXISTS "x_express_company" (
    "id" VARCHAR PRIMARY KEY,
    "code" VARCHAR,
    "xcode" VARCHAR,
    "xname" VARCHAR,
    "xstatus" VARCHAR,
    "xtrackingNumber" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_express_company" IS 'auto-created for parity (was X.EXPRESS_COMPANY)';

CREATE TABLE IF NOT EXISTS "x_express_info" (
    "id" VARCHAR PRIMARY KEY,
    "xcode" VARCHAR,
    "xname" VARCHAR,
    "xstatus" VARCHAR,
    "xtrackingNumber" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_express_info" IS 'auto-created for parity (was X.EXPRESS_INFO)';

CREATE TABLE IF NOT EXISTS "x_express_subscribe" (
    "id" VARCHAR PRIMARY KEY,
    "xcode" VARCHAR,
    "xname" VARCHAR,
    "xstatus" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xid" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_express_subscribe" IS 'auto-created for parity (was X.EXPRESS_SUBSCRIBE)';

CREATE TABLE IF NOT EXISTS "x_msg_message" (
    "id" VARCHAR PRIMARY KEY,
    "xbody" VARCHAR,
    "xconsumer" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xid" VARCHAR,
    "xperson" VARCHAR,
    "xtitle" VARCHAR,
    "xtype" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_msg_message" IS 'auto-created for parity (was X.MSG_MESSAGE)';

CREATE TABLE IF NOT EXISTS "x_pp_c_task" (
    "id" VARCHAR PRIMARY KEY,
    "activity" VARCHAR,
    "application" VARCHAR,
    "category" VARCHAR,
    "completed" BOOLEAN,
    "end_time" TIMESTAMPTZ,
    "expired" BOOLEAN,
    "name" VARCHAR,
    "pending" BOOLEAN,
    "person" VARCHAR,
    "process" VARCHAR,
    "processing" BOOLEAN,
    "start_time" TIMESTAMPTZ,
    "task_status" VARCHAR,
    "title" VARCHAR,
    "total" BIGINT,
    "work_status" VARCHAR,
    "xcategory" VARCHAR,
    "xdefinition" VARCHAR,
    "xenabled" VARCHAR,
    "xid" VARCHAR,
    "xname" VARCHAR,
    "create_time" TIMESTAMPTZ,
    "update_time" TIMESTAMPTZ,
    "creator_person" VARCHAR,
    "creator_unit" VARCHAR,
    "last_update_person" VARCHAR,
    "sequence" VARCHAR,
    "xcreateTime" TIMESTAMPTZ,
    "xupdateTime" TIMESTAMPTZ,
    "xowner" VARCHAR
);

COMMENT ON TABLE "x_pp_c_task" IS 'auto-created for parity (was X.PP_C_TASK)';
