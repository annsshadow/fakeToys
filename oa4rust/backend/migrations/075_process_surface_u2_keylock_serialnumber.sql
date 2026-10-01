-- plan002 U2-c: processplatform_assemble_surface filter-family gap closure.
-- Columns required by the newly implemented Java-alignment endpoints:
--   pp_c_keylock       <- PUT /keylock/lock            (Java KeyLock: xkey / xperson)
--   pp_c_serialnumber  <- POST /serialnumber/generate/... (Java SerialNumber: serial counter)
--   pp_c_handover      <- POST /handover               (Java Handover: type/scheme/person/
--                                                         targetIdentity/targetPerson/status)
--   pp_c_draft         <- PUT /draft                   (Java Draft.properties: xdata)-- Column naming follows the 032/065 convention (JPA field name as quoted column).
-- Idempotent. Rollback: 075_process_surface_u2_keylock_serialnumber_rollback.sql

ALTER TABLE "pp_c_keylock" ADD COLUMN IF NOT EXISTS "xkey" TEXT;
ALTER TABLE "pp_c_keylock" ADD COLUMN IF NOT EXISTS "xperson" TEXT;

ALTER TABLE "pp_c_serialnumber" ADD COLUMN IF NOT EXISTS "xserial" INTEGER NOT NULL DEFAULT 0;

ALTER TABLE "pp_c_handover" ADD COLUMN IF NOT EXISTS "xtargetPerson" TEXT;
ALTER TABLE "pp_c_handover" ADD COLUMN IF NOT EXISTS "xtargetIdentity" TEXT;
ALTER TABLE "pp_c_handover" ADD COLUMN IF NOT EXISTS "xtype" TEXT;
ALTER TABLE "pp_c_handover" ADD COLUMN IF NOT EXISTS "xscheme" TEXT;
ALTER TABLE "pp_c_handover" ADD COLUMN IF NOT EXISTS "xstatus" TEXT;

ALTER TABLE "pp_c_draft" ADD COLUMN IF NOT EXISTS "xdata" TEXT;

CREATE INDEX IF NOT EXISTS idx_pp_c_keylock_xkey ON "pp_c_keylock"("xkey");
CREATE INDEX IF NOT EXISTS idx_pp_c_serialnumber_process ON "pp_c_serialnumber"("xprocess");
