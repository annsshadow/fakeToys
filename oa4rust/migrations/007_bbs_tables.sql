-- BBS (Bulletin Board System) tables
CREATE TABLE IF NOT EXISTS bbs_forum_info (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    order_number INTEGER DEFAULT 0,
    disable BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bbs_section_info (
    id VARCHAR(255) PRIMARY KEY,
    forum_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    order_number INTEGER DEFAULT 0,
    disable BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bbs_subject_info (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    author_id VARCHAR(255) NOT NULL,
    section_id VARCHAR(255) NOT NULL,
    content TEXT,
    reply_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    is_top BOOLEAN DEFAULT FALSE,
    disable BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bbs_comment_info (
    id VARCHAR(255) PRIMARY KEY,
    subject_id VARCHAR(255) NOT NULL,
    author_id VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_bbs_subject_section ON bbs_subject_info(section_id);
CREATE INDEX IF NOT EXISTS idx_bbs_subject_author ON bbs_subject_info(author_id);
CREATE INDEX IF NOT EXISTS idx_bbs_comment_subject ON bbs_comment_info(subject_id);
CREATE INDEX IF NOT EXISTS idx_bbs_section_forum ON bbs_section_info(forum_id);
