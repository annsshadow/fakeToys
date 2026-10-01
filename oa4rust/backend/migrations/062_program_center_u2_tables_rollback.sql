-- Rollback of 062_program_center_u2_tables.sql.
-- Only drops objects created by 062; leaves pre-existing data intact.

DROP INDEX IF EXISTS "idx_x_program_warn_log_tag";
DROP TABLE IF EXISTS "x_program_warn_log";

DROP INDEX IF EXISTS "idx_x_program_app_pack_status";
DROP TABLE IF EXISTS "x_program_app_pack";

ALTER TABLE IF EXISTS "x_program_script"
    DROP COLUMN IF EXISTS "content";

ALTER TABLE IF EXISTS "x_program_dict"
    DROP COLUMN IF EXISTS "flag";
