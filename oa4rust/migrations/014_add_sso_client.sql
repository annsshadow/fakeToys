-- Migration 014: Add sso_client table for SSO client key management
-- Stores client_name -> 3DES key mapping for SSO GET endpoint.

BEGIN;

CREATE TABLE IF NOT EXISTS sso_client (
    id VARCHAR(255) PRIMARY KEY,
    client_name VARCHAR(255) UNIQUE NOT NULL,
    key VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

COMMIT;
