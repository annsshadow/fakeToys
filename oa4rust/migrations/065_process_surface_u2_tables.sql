-- plan002 U2: processplatform_assemble_surface missing-endpoint tables.
-- Supports the Java-alignment endpoints added in this round:
--   PP_C_SNAP  -> /snap/{id}, /snap/{id}/restore, /snap/list/{id}/next|prev/{count},
--                 /snap/work/{workId}/type/{type}, /snap/workcompleted/{id}/type/{type}
--                 (Java entity: com.x.processplatform.core.entity.content.Snap)
--   PP_C_ATTACHMENT -> /attachment/list/job|work|workcompleted|workorworkcompleted/{flag},
--                 /attachment/{id}/available, /attachment/{id}/work/{workId}[/text],
--                 DELETE /attachment/{id}/work/{workId}
--                 (Java entity: com.x.processplatform.core.entity.content.Attachment)
-- Column names follow the 032_create_assemble_control_tables.sql convention:
-- Java JPA field name (x-prefixed, mixed case) as a quoted column + standard audit columns.
-- Idempotent: safe to run repeatedly. Rollback file: 065_process_surface_u2_tables_rollback.sql

CREATE TABLE IF NOT EXISTS "pp_c_snap" (
    "xid" TEXT,
    "xtitle" TEXT,
    "xjob" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xtype" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xunit" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xprocessAlias" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xactivity" TEXT,
    "xactivityName" TEXT,
    "xactivityToken" TEXT,
    "xdata" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_attachment" (
    "xid" TEXT,
    "xjob" TEXT,
    "xname" TEXT,
    "xextension" TEXT,
    "xlength" BIGINT,
    "xsite" TEXT,
    "xtype" TEXT,
    "xtext" TEXT,
    "xstorage" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xcompleted" BOOLEAN DEFAULT FALSE,
    "xperson" TEXT,
    "xactivity" TEXT,
    "xactivityName" TEXT,
    "xactivityToken" TEXT,
    "xlastUpdatePerson" TEXT,
    "xapplication" TEXT,
    "xprocess" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE INDEX IF NOT EXISTS idx_pp_c_snap_work ON "pp_c_snap"("xwork");
CREATE INDEX IF NOT EXISTS idx_pp_c_snap_job ON "pp_c_snap"("xjob");
CREATE INDEX IF NOT EXISTS idx_pp_c_snap_type ON "pp_c_snap"("xtype");
CREATE INDEX IF NOT EXISTS idx_pp_c_attachment_job ON "pp_c_attachment"("xjob");
CREATE INDEX IF NOT EXISTS idx_pp_c_attachment_work ON "pp_c_attachment"("xwork");
