-- Migration 023: CMS Assemble Control tables
-- Required by cms_assemble_control crate for real DB operations
-- Tables follow the naming convention: x_cms_<entity>

CREATE TABLE IF NOT EXISTS x_cms_appinfo (
    id VARCHAR(255) PRIMARY KEY,
    app_type VARCHAR(50) NOT NULL,
    alias VARCHAR(255),
    icon TEXT,
    enabled BOOLEAN DEFAULT true,
    manager VARCHAR(255),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_categoryinfo (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id VARCHAR(255),
    app_id VARCHAR(255),
    sort_order INTEGER DEFAULT 0,
    status VARCHAR(50) DEFAULT 'enabled',
    ext_content TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_comment (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    person_id VARCHAR(255),
    content TEXT,
    parent_id VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_commend (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    person_id VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_correlation (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    related_doc_id VARCHAR(255) NOT NULL,
    correlation_type VARCHAR(50),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_file (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255),
    name VARCHAR(255),
    size BIGINT DEFAULT 0,
    content_type VARCHAR(255),
    content_base64 TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_fileinfo (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255),
    file_id VARCHAR(255),
    original_name VARCHAR(255),
    size BIGINT DEFAULT 0,
    content_type VARCHAR(255),
    upload_person VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_form (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    definition TEXT,
    status VARCHAR(50) DEFAULT 'draft',
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_form_field (
    id VARCHAR(255) PRIMARY KEY,
    form_id VARCHAR(255) NOT NULL,
    field_name VARCHAR(255) NOT NULL,
    field_type VARCHAR(50),
    config TEXT,
    sort_order INTEGER DEFAULT 0,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_log (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255),
    category_id VARCHAR(255),
    doc_id VARCHAR(255),
    person_id VARCHAR(255),
    operation_level VARCHAR(50),
    operation_type VARCHAR(255),
    operation_detail TEXT,
    ip_address VARCHAR(50),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_output (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255),
    name VARCHAR(255),
    config TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_permission (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255),
    category_id VARCHAR(255),
    person_id VARCHAR(255),
    role_type VARCHAR(50),
    permission_level VARCHAR(50),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_script (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    unique_name VARCHAR(255),
    script_content TEXT,
    imported BOOLEAN DEFAULT false,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_searchfilter (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255) NOT NULL,
    category_id VARCHAR(255),
    filter_type VARCHAR(50),
    filter_config TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_surface_appdict (
    id VARCHAR(255) PRIMARY KEY,
    app_info_flag VARCHAR(255),
    app_dict_flag VARCHAR(255),
    path_levels TEXT[],
    data_value TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_view (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255),
    category_id VARCHAR(255),
    name VARCHAR(255) NOT NULL,
    view_config TEXT,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_viewcategory (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id VARCHAR(255),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_viewfieldconfig (
    id VARCHAR(255) PRIMARY KEY,
    view_id VARCHAR(255) NOT NULL,
    field_name VARCHAR(255) NOT NULL,
    field_config TEXT,
    sort_order INTEGER DEFAULT 0,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_viewrecord (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    view_id VARCHAR(255),
    record_data TEXT,
    person_id VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_data_document (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255),
    category_id VARCHAR(255),
    title VARCHAR(255),
    content TEXT,
    author_id VARCHAR(255),
    status VARCHAR(50) DEFAULT 'draft',
    publish_time TIMESTAMP,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_data_document_field (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    field_name VARCHAR(255) NOT NULL,
    field_value TEXT,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_document_cipher (
    id VARCHAR(255) PRIMARY KEY,
    doc_id VARCHAR(255) NOT NULL,
    cipher_text TEXT,
    person_id VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_cms_form_v2 (
    id VARCHAR(255) PRIMARY KEY,
    app_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    definition TEXT,
    status VARCHAR(50) DEFAULT 'draft',
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_x_cms_appinfo_type ON x_cms_appinfo(app_type);
CREATE INDEX IF NOT EXISTS idx_x_cms_appinfo_alias ON x_cms_appinfo(alias);
CREATE INDEX IF NOT EXISTS idx_x_cms_categoryinfo_app ON x_cms_categoryinfo(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_categoryinfo_parent ON x_cms_categoryinfo(parent_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_comment_doc ON x_cms_comment(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_comment_person ON x_cms_comment(person_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_commend_doc ON x_cms_commend(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_correlation_doc ON x_cms_correlation(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_file_app ON x_cms_file(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_fileinfo_doc ON x_cms_fileinfo(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_form_app ON x_cms_form(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_form_field_form ON x_cms_form_field(form_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_log_app ON x_cms_log(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_log_doc ON x_cms_log(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_permission_person ON x_cms_permission(person_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_script_app ON x_cms_script(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_searchfilter_app ON x_cms_searchfilter(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_view_app ON x_cms_view(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_view_category ON x_cms_view(category_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_viewfieldconfig_view ON x_cms_viewfieldconfig(view_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_viewrecord_doc ON x_cms_viewrecord(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_data_doc_app ON x_cms_data_document(app_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_data_doc_cat ON x_cms_data_document(category_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_data_field_doc ON x_cms_data_document_field(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_cipher_doc ON x_cms_document_cipher(doc_id);
CREATE INDEX IF NOT EXISTS idx_x_cms_form_v2_app ON x_cms_form_v2(app_id);
