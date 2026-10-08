-- File system tables
CREATE TABLE IF NOT EXISTS FILE_FOLDER (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    person VARCHAR(255) NOT NULL,
    superior VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FILE_FILE (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(500) NOT NULL,
    person VARCHAR(255) NOT NULL,
    reference_id VARCHAR(255) DEFAULT '',
    reference_type VARCHAR(100) DEFAULT 'file',
    extension VARCHAR(50),
    length BIGINT DEFAULT 0,
    mime_type VARCHAR(200),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS FILE_PERMISSION (
    id VARCHAR(255) PRIMARY KEY,
    target_type VARCHAR(100) NOT NULL,
    target_id VARCHAR(255) NOT NULL,
    permissions JSONB DEFAULT '{}',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (target_type, target_id)
);

CREATE INDEX IF NOT EXISTS idx_file_folder_superior ON FILE_FOLDER(superior);
CREATE INDEX IF NOT EXISTS idx_file_file_person ON FILE_FILE(person);
CREATE INDEX IF NOT EXISTS idx_file_file_reference ON FILE_FILE(reference_id, reference_type);
