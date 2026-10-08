-- 074_u2_small_modules.sql 回滚：删除本迁移创建的表与索引。
-- 幂等：重复执行不报错。

DROP INDEX IF EXISTS "uq_cal_setting_code";
DROP TABLE IF EXISTS "cal_setting";

DROP INDEX IF EXISTS "idx_program_invoke_category";
DROP INDEX IF EXISTS "uq_program_invoke_name";
DROP TABLE IF EXISTS "x_program_invoke";

DROP INDEX IF EXISTS "idx_init_restore_upload_status";
DROP TABLE IF EXISTS "init_restore_upload";

DROP INDEX IF EXISTS "idx_init_server_command_created";
DROP TABLE IF EXISTS "init_server_command";

DROP INDEX IF EXISTS "uq_init_ext_ds_name";
DROP INDEX IF EXISTS "idx_init_ext_ds_applied";
DROP TABLE IF EXISTS "init_external_datasource";
