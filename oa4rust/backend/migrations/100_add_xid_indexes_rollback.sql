-- 100_add_xid_indexes_rollback.sql
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
        EXECUTE format('DROP INDEX IF EXISTS %I', idx_name);
    END LOOP;
END $$;
