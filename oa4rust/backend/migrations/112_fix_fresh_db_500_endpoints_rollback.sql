-- 112_fix_fresh_db_500_endpoints_rollback.sql
-- 回滚 112：只删本迁移新建的列/视图/表。注意这些对象本就被 handler 引用，
-- 删掉后对应端点回到「undefined column / relation does not exist」500 ——
-- 只在确认要退回旧状态时执行。
--
-- ⚠️ 对 live 库（Java 时代或 089 手工应用过）本迁移是 no-op，所以本回滚
-- 在那些库上同样 mostly no-op；会真正生效的是 fresh DB（112 建的空列/空表/视图）。

DO $$
BEGIN

-- F-1 / F-5：删 112 补的列
IF TO_REGCLASS('public.x_process_bam') IS NOT NULL THEN
    ALTER TABLE "x_process_bam" DROP COLUMN IF EXISTS "status";
END IF;
IF TO_REGCLASS('public.x_program_field') IS NOT NULL THEN
    ALTER TABLE "x_program_field" DROP COLUMN IF EXISTS "sort_order";
END IF;

-- F-3：删 112 建的视图（只在确是本迁移建的场景才有意义；
-- live 库若早已手工建过同名视图，此删除会移除它——执行前先确认归属）。
DROP VIEW IF EXISTS "PP_E_APPLICATION";

-- P0-4：删 112 建的空遗留表（fresh DB 专属形状；live 库的 Java 表
-- 不是本迁移建的，但 DROP IF EXISTS 不区分来源——执行前确认这些表
-- 在目标库里没有需要保留的数据）。
DROP TABLE IF EXISTS "x_process_transition";
DROP TABLE IF EXISTS "pp_c_touch_detained";
DROP TABLE IF EXISTS "pp_c_touch_passexpired";
DROP TABLE IF EXISTS "pp_c_touch_expire";

END $$;
