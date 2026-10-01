-- Process platform tables for Rust side
-- Core workflow engine tables referenced by processplatform_service_processing and processplatform_assemble_designer

CREATE TABLE IF NOT EXISTS x_work (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    process VARCHAR(255) NOT NULL,
    application VARCHAR(255),
    work_status VARCHAR(50) DEFAULT 'pending',
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_task (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(500),
    work VARCHAR(255) NOT NULL,
    activity VARCHAR(255),
    activity_token VARCHAR(255),
    person VARCHAR(255),
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    task_status VARCHAR(50) DEFAULT 'pending',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_review (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    reviewer VARCHAR(255),
    comment TEXT,
    status VARCHAR(50) DEFAULT 'pending',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_snap (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    snap_type VARCHAR(50) NOT NULL,
    snap_data JSONB,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_record (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255),
    task_id VARCHAR(255),
    record_type VARCHAR(50),
    content TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_workcompleted (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    completed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_draft (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    content JSONB,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_read (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    person VARCHAR(255) NOT NULL,
    read_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_readcompleted (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    person VARCHAR(255) NOT NULL,
    completed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_attachment (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255),
    workcompleted_id VARCHAR(255),
    name VARCHAR(255),
    content TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_document_version (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    version INTEGER DEFAULT 1,
    content JSONB,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_job (
    id VARCHAR(255) PRIMARY KEY,
    work_id VARCHAR(255) NOT NULL,
    person VARCHAR(255) NOT NULL,
    activity_token VARCHAR(255),
    job_status VARCHAR(50) DEFAULT 'pending',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    start_time TIMESTAMP,
    end_time TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_process_definition (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    category VARCHAR(255),
    process_definition TEXT,
    version INTEGER DEFAULT 1,
    creator VARCHAR(255) DEFAULT 'system',
    status VARCHAR(50) DEFAULT 'disabled',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_work_status ON x_work(work_status);
CREATE INDEX IF NOT EXISTS idx_x_work_application ON x_work(application);
CREATE INDEX IF NOT EXISTS idx_x_task_work ON x_task(work);
CREATE INDEX IF NOT EXISTS idx_x_task_person ON x_task(person);
CREATE INDEX IF NOT EXISTS idx_x_review_work ON x_review(work_id);
CREATE INDEX IF NOT EXISTS idx_x_snap_work ON x_snap(work_id);
CREATE INDEX IF NOT EXISTS idx_x_record_work ON x_record(work_id);
CREATE INDEX IF NOT EXISTS idx_x_workcompleted_work ON x_workcompleted(work_id);
CREATE INDEX IF NOT EXISTS idx_x_draft_work ON x_draft(work_id);
CREATE INDEX IF NOT EXISTS idx_x_read_work ON x_read(work_id);
CREATE INDEX IF NOT EXISTS idx_x_job_work ON x_job(work_id);
CREATE INDEX IF NOT EXISTS idx_x_job_person ON x_job(person);
CREATE INDEX IF NOT EXISTS idx_x_process_def_category ON x_process_definition(category);
CREATE INDEX IF NOT EXISTS idx_x_process_def_status ON x_process_definition(status);
