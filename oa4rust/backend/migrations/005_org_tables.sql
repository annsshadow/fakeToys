-- Organization tables and soft-delete columns
-- Required by control module endpoints (person/unit/role/group CRUD)
-- auth_group table + deleted_at/avatar/icon columns for auth_person/auth_unit/auth_role

CREATE TABLE IF NOT EXISTS auth_group (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    disable BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE auth_person ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP;
ALTER TABLE auth_person ADD COLUMN IF NOT EXISTS avatar TEXT;
ALTER TABLE auth_person ADD COLUMN IF NOT EXISTS icon TEXT;

ALTER TABLE auth_unit ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP;
ALTER TABLE auth_role ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP;
ALTER TABLE auth_role ADD COLUMN IF NOT EXISTS disable BOOLEAN DEFAULT FALSE;

CREATE INDEX IF NOT EXISTS idx_auth_group_name ON auth_group(name);
CREATE INDEX IF NOT EXISTS idx_auth_person_deleted ON auth_person(deleted_at);
CREATE INDEX IF NOT EXISTS idx_auth_unit_deleted ON auth_unit(deleted_at);
CREATE INDEX IF NOT EXISTS idx_auth_role_deleted ON auth_role(deleted_at);
