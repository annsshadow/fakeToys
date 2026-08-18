-- Migration 016: x_empower table for authorization management
-- Stores person-to-person empowerment (role delegation) records

CREATE TABLE IF NOT EXISTS x_empower (
    id VARCHAR(255) PRIMARY KEY,
    from_person VARCHAR(255) NOT NULL,
    to_person VARCHAR(255) NOT NULL,
    role_id VARCHAR(255),
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_empower_from_person ON x_empower(from_person);
CREATE INDEX IF NOT EXISTS idx_x_empower_to_person ON x_empower(to_person);
CREATE INDEX IF NOT EXISTS idx_x_empower_role ON x_empower(role_id);
CREATE INDEX IF NOT EXISTS idx_x_empower_deleted ON x_empower(deleted_at);
