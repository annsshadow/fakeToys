-- Rollback for migration 010_query_processing_unique_index.sql
-- Drop the unique index on x_query_processing.model_flag

BEGIN;

DROP INDEX IF EXISTS uq_query_processing_model_flag;

COMMIT;
