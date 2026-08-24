-- plan002 U2 冲刺：processplatform_assemble_designer / processplatform_assemble_bam
-- 端点闭合所需的幂等补列。
--
-- 背景：032 建的 pp_e_process 未包含既有/新增端点 handler 实际读写的列：
--   - GET /process/{id}/disable|enable、POST /process/{id}/upgrade 等
--     （本冲刺注册到 Java 精确路径）读写 xstatus / xversion；
--   - POST /process/{id}/permission 写 xproperties；
--   - edition 族端点（GET /process/application/{id}/edition/{edition} 等）读写 xedition；
--   - GET /process/form/{formId} 按 xformid 查询。
-- BAM 监控端点全部基于既有表（x_task / x_work / x_org_person / x_bam_config），
-- 无需建表；x_bam_config 已由 034 创建。
--
-- 说明：计划原文提到的 migration 080 已被 080_add_ai_missing_columns 占用，
-- 故本迁移顺延编号 081。全部语句幂等，可重复执行。

ALTER TABLE "pp_e_process" ADD COLUMN IF NOT EXISTS "xstatus" TEXT;
ALTER TABLE "pp_e_process" ADD COLUMN IF NOT EXISTS "xedition" TEXT;
ALTER TABLE "pp_e_process" ADD COLUMN IF NOT EXISTS "xversion" BIGINT;
ALTER TABLE "pp_e_process" ADD COLUMN IF NOT EXISTS "xformid" TEXT;
ALTER TABLE "pp_e_process" ADD COLUMN IF NOT EXISTS "xproperties" TEXT;

-- 缺省启用态回填，保证 enable/disable/enabled 端点语义确定。
UPDATE "pp_e_process" SET "xstatus" = 'enabled' WHERE "xstatus" IS NULL;

-- 版本列缺省 0，供 upgrade 端点 COALESCE 自增。
UPDATE "pp_e_process" SET "xversion" = 0 WHERE "xversion" IS NULL;
