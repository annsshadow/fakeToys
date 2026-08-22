-- plan002 U2 rollback for 065_process_surface_u2_tables.sql.
-- Only drops objects created by 065; both tables are new in this round.

DROP INDEX IF EXISTS idx_pp_c_attachment_work;
DROP INDEX IF EXISTS idx_pp_c_attachment_job;
DROP INDEX IF EXISTS idx_pp_c_snap_type;
DROP INDEX IF EXISTS idx_pp_c_snap_job;
DROP INDEX IF EXISTS idx_pp_c_snap_work;

DROP TABLE IF EXISTS "pp_c_attachment";
DROP TABLE IF EXISTS "pp_c_snap";
