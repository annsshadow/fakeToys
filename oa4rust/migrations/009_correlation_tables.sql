-- Correlation tables
CREATE TABLE IF NOT EXISTS x_correlation (
    id VARCHAR(255) PRIMARY KEY,
    source_type VARCHAR(100) NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    target_type VARCHAR(100) NOT NULL,
    target_id VARCHAR(255) NOT NULL,
    weight INTEGER DEFAULT 0,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS CORR_C_CORRELATION (
    id VARCHAR(255) PRIMARY KEY,
    from_bundle VARCHAR(255) NOT NULL,
    target_bundle VARCHAR(255) NOT NULL,
    from_type VARCHAR(100) NOT NULL,
    person VARCHAR(255),
    site TEXT,
    order_number INTEGER DEFAULT 0,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_correlation_source ON x_correlation(source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_correlation_target ON x_correlation(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_corr_c_from_type ON CORR_C_CORRELATION(from_type);
