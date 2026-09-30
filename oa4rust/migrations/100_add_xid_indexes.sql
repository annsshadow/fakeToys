-- 100_add_xid_indexes.sql
-- 为所有含 "xid" 列的表补 (xid) b-tree 索引。
-- 业务代码里大量 handler 以 WHERE xid = $1 按唯一标识取行（processplatform_assemble_surface
-- 单 crate 即 500+ 处），此前 63 张含 xid 的表没有任何一张有 xid 索引，
-- 这些查询全部退化为顺序扫描；随流程/工作数据量增长线性恶化。
-- 动态枚举而非硬编码表清单：对任何含 xid 列的表幂等建索引，避免清单与 schema 漂移。
-- 用普通索引而非 UNIQUE：不假设存量数据已满足唯一性，避免历史脏数据导致迁移失败。

DO $$
DECLARE
    r RECORD;
    idx_name TEXT;
BEGIN
    FOR r IN
        SELECT t.tablename
        FROM pg_tables t
        JOIN information_schema.columns c
          ON c.table_schema = t.schemaname AND c.table_name = t.tablename AND c.column_name = 'xid'
        WHERE t.schemaname = 'public'
    LOOP
        idx_name := 'idx_' || r.tablename || '_xid';
        IF length(idx_name) > 63 THEN
            idx_name := substring(idx_name FROM 1 FOR 63);
        END IF;
        EXECUTE format(
            'CREATE INDEX IF NOT EXISTS %I ON %I ("xid")',
            idx_name, r.tablename
        );
    END LOOP;
END $$;
