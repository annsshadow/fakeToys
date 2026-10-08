-- 081_pp_u2_designer_bam.sql 的回滚：仅移除本次新增列（幂等）。

ALTER TABLE "pp_e_process" DROP COLUMN IF EXISTS "xstatus";
ALTER TABLE "pp_e_process" DROP COLUMN IF EXISTS "xedition";
ALTER TABLE "pp_e_process" DROP COLUMN IF EXISTS "xversion";
ALTER TABLE "pp_e_process" DROP COLUMN IF EXISTS "xformid";
ALTER TABLE "pp_e_process" DROP COLUMN IF EXISTS "xproperties";
