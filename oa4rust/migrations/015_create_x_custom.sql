-- Migration 015: x_custom table for electronic signatures
-- Stores base64-encoded signature images as key-value pairs

CREATE TABLE IF NOT EXISTS x_custom (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    person VARCHAR(255) NOT NULL,
    value TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_custom_person ON x_custom(person);
CREATE INDEX IF NOT EXISTS idx_x_custom_name ON x_custom(name);
CREATE INDEX IF NOT EXISTS idx_x_custom_deleted ON x_custom(deleted_at);
