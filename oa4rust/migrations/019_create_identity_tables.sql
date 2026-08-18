-- Migration 019: Create auth_identity and auth_person_identity tables
-- Required by: R2 (identityList in login response), R19 (batch query identity)

CREATE TABLE IF NOT EXISTS auth_identity (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    identity_type VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS auth_person_identity (
    person_unique VARCHAR(255) NOT NULL,
    identity_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (person_unique, identity_id)
);

CREATE INDEX IF NOT EXISTS idx_auth_person_identity_person ON auth_person_identity(person_unique);
CREATE INDEX IF NOT EXISTS idx_auth_person_identity_identity ON auth_person_identity(identity_id);
CREATE INDEX IF NOT EXISTS idx_auth_identity_name ON auth_identity(name);
