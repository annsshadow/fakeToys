-- Manual rollback for 075_process_surface_u2_keylock_serialnumber.sql.
ALTER TABLE "pp_c_keylock" DROP COLUMN IF EXISTS "xkey";
ALTER TABLE "pp_c_keylock" DROP COLUMN IF EXISTS "xperson";
ALTER TABLE "pp_c_serialnumber" DROP COLUMN IF EXISTS "xserial";
ALTER TABLE "pp_c_handover" DROP COLUMN IF EXISTS "xtargetPerson";
ALTER TABLE "pp_c_handover" DROP COLUMN IF EXISTS "xtargetIdentity";
ALTER TABLE "pp_c_handover" DROP COLUMN IF EXISTS "xtype";
ALTER TABLE "pp_c_handover" DROP COLUMN IF EXISTS "xscheme";
ALTER TABLE "pp_c_handover" DROP COLUMN IF EXISTS "xstatus";
ALTER TABLE "pp_c_draft" DROP COLUMN IF EXISTS "xdata";
DROP INDEX IF EXISTS idx_pp_c_keylock_xkey;
DROP INDEX IF EXISTS idx_pp_c_serialnumber_process;
