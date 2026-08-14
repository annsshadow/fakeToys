-- Organization assemble control tables for Rust side
-- Referenced by organization_assemble_control crate

CREATE TABLE IF NOT EXISTS x_org_role (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_unit (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id VARCHAR(255),
    level INTEGER DEFAULT 0,
    sort INTEGER DEFAULT 0,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_person (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    mobile VARCHAR(50),
    email VARCHAR(255),
    unit_id VARCHAR(255),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_group (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    unit_id VARCHAR(255),
    type VARCHAR(50),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_identity (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    unit_id VARCHAR(255),
    identity_id VARCHAR(255),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_duty (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    unit_id VARCHAR(255),
    identity_id VARCHAR(255),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_permission_setting (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    unit_id VARCHAR(255),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_unit_attribute (
    id VARCHAR(255) PRIMARY KEY,
    unit_id VARCHAR(255) NOT NULL,
    attribute_key VARCHAR(255) NOT NULL,
    attribute_value TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_person_attribute (
    id VARCHAR(255) PRIMARY KEY,
    person_id VARCHAR(255) NOT NULL,
    attribute_key VARCHAR(255) NOT NULL,
    attribute_value TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_group_member (
    group_id VARCHAR(255) NOT NULL,
    person_id VARCHAR(255) NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (group_id, person_id)
);

CREATE TABLE IF NOT EXISTS x_org_group_role (
    group_id VARCHAR(255) NOT NULL,
    role_id VARCHAR(255) NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (group_id, role_id)
);

CREATE TABLE IF NOT EXISTS x_org_export (
    id VARCHAR(255) PRIMARY KEY,
    type VARCHAR(50),
    status VARCHAR(50),
    file_url TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_import_result (
    id VARCHAR(255) PRIMARY KEY,
    person_id VARCHAR(255),
    status VARCHAR(50),
    message TEXT,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_org_login_record (
    id VARCHAR(255) PRIMARY KEY,
    person_id VARCHAR(255),
    stream VARCHAR(255),
    login_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ip VARCHAR(50),
    device VARCHAR(255)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_x_org_role_name ON x_org_role(name);
CREATE INDEX IF NOT EXISTS idx_x_org_unit_parent ON x_org_unit(parent_id);
CREATE INDEX IF NOT EXISTS idx_x_org_person_unit ON x_org_person(unit_id);
CREATE INDEX IF NOT EXISTS idx_x_org_group_unit ON x_org_group(unit_id);
CREATE INDEX IF NOT EXISTS idx_x_org_identity_unit ON x_org_identity(unit_id);
CREATE INDEX IF NOT EXISTS idx_x_org_duty_unit ON x_org_duty(unit_id);
CREATE INDEX IF NOT EXISTS idx_x_org_unit_attr_unit ON x_org_unit_attribute(unit_id);
CREATE INDEX IF NOT EXISTS idx_x_org_person_attr_person ON x_org_person_attribute(person_id);
CREATE INDEX IF NOT EXISTS idx_x_org_export_type ON x_org_export(type);
CREATE INDEX IF NOT EXISTS idx_x_org_login_record_person ON x_org_login_record(person_id);
