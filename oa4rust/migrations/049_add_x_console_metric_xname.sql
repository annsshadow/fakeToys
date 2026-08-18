-- x_console_metric.xname: metric name used by the console get_metric handler (WHERE xname = $1).
-- The auto-created parity table (034) omitted this column; o2server's X.CONSOLE_METRIC
-- uses it as the lookup key, so add it to keep the handler semantically correct.
ALTER TABLE "x_console_metric" ADD COLUMN IF NOT EXISTS "xname" VARCHAR;

-- Rollback: ALTER TABLE "x_console_metric" DROP COLUMN IF EXISTS "xname";
