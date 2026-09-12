-- W9: persist typed QueryTable columns used by the safe DDL executor.
ALTER TABLE "x_query_table"
    ADD COLUMN IF NOT EXISTS "columns" TEXT NOT NULL DEFAULT '[]';
