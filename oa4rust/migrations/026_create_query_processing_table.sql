-- Migration 026: Create x_query_processing table (referenced by query_service but missing from prior migrations)
-- Also creates the partial unique index that migration 010 expected.

CREATE TABLE IF NOT EXISTS x_query_processing (
    id VARCHAR(255) PRIMARY KEY,
    query TEXT NOT NULL,
    model_flag VARCHAR(255),
    params JSONB DEFAULT '{}',
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_query_processing_model_flag
    ON x_query_processing (model_flag)
    WHERE model_flag IS NOT NULL;
