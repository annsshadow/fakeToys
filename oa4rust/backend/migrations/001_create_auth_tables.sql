-- Auth tables for Rust side
-- This schema is designed to be compatible with Java's JPA entities

CREATE TABLE IF NOT EXISTS auth_person (
    id VARCHAR(255) PRIMARY KEY,
    unique_id VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    mobile VARCHAR(50),
    email VARCHAR(255),
    password_hash VARCHAR(255) NOT NULL,
    salt VARCHAR(255),
    locked BOOLEAN DEFAULT FALSE,
    locked_at TIMESTAMP,
    failed_attempts INTEGER DEFAULT 0,
    last_login_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS auth_role (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS auth_unit (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id VARCHAR(255),
    level INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS auth_person_role (
    person_id VARCHAR(255) NOT NULL,
    role_id VARCHAR(255) NOT NULL,
    unit_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (person_id, role_id, unit_id)
);

CREATE TABLE IF NOT EXISTS auth_session (
    token VARCHAR(255) PRIMARY KEY,
    person_id VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_auth_person_unique ON auth_person(unique_id);
CREATE INDEX IF NOT EXISTS idx_auth_session_person ON auth_session(person_id);
CREATE INDEX IF NOT EXISTS idx_auth_person_role ON auth_person_role(person_id);
