-- Migration 017: Create auth_token_threshold table for safe_logout broadcast
-- This table tracks the latest logout time per user across multiple instances.
-- When safe_logout is called, it writes the current timestamp here, and all
-- instances check this table during session validation to invalidate old tokens.

CREATE TABLE IF NOT EXISTS auth_token_threshold (
    person_unique VARCHAR(255) PRIMARY KEY,
    threshold_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_auth_token_threshold_person ON auth_token_threshold(person_unique);
