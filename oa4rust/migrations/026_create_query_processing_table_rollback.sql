-- Rollback for migration 026: Drop x_query_processing table and its index

DROP INDEX IF EXISTS uq_query_processing_model_flag;
DROP TABLE IF EXISTS x_query_processing;
