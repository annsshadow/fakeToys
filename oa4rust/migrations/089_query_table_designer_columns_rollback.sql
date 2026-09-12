-- Roll back W9 QueryTable typed column metadata.
ALTER TABLE "x_query_table" DROP COLUMN IF EXISTS "columns";
