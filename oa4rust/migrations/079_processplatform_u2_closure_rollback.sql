-- plan002 U2 rollback for 079_processplatform_u2_closure.sql
-- 仅回滚 079 新增对象；不触碰既有表数据。

DROP TABLE IF EXISTS "x_data";
DROP TABLE IF EXISTS "x_application_dict";

ALTER TABLE "x_read" DROP COLUMN IF EXISTS "scope";
ALTER TABLE "x_task" DROP COLUMN IF EXISTS "next_task_identity";
