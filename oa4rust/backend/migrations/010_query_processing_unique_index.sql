-- 010: Add unique index on x_query_processing.model_flag
--
-- Required for ON CONFLICT (model_flag) in query_service processing_execute
-- to work correctly. Uses partial index to allow NULL model_flag values.
--
-- Pre-flight check: if constraint already exists, skip creation.
-- Also ensures the table exists (created here if missing).

CREATE TABLE IF NOT EXISTS x_query_processing (
    id VARCHAR(255) PRIMARY KEY,
    query TEXT NOT NULL,
    model_flag VARCHAR(255),
    params JSONB DEFAULT '{}',
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'uq_query_processing_model_flag'
    ) THEN
        CREATE UNIQUE INDEX IF NOT EXISTS uq_query_processing_model_flag
            ON x_query_processing (model_flag)
            WHERE model_flag IS NOT NULL;
    END IF;
END $$;
