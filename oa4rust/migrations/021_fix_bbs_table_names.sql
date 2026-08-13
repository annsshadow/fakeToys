-- Fix BBS table names to match code queries
-- Migration 007 created bbs_forum_info, bbs_section_info, bbs_subject_info, bbs_comment_info
-- Code queries x_bbs_forum, x_bbs_section, x_bbs_topic, x_bbs_reply
-- Also create missing config/section control tables

DO $$
BEGIN
    -- Rename existing tables if they exist
    IF EXISTS (SELECT FROM pg_tables WHERE tablename = 'bbs_forum_info') THEN
        ALTER TABLE bbs_forum_info RENAME TO x_bbs_forum;
    END IF;
    IF EXISTS (SELECT FROM pg_tables WHERE tablename = 'bbs_section_info') THEN
        ALTER TABLE bbs_section_info RENAME TO x_bbs_section;
    END IF;
    IF EXISTS (SELECT FROM pg_tables WHERE tablename = 'bbs_subject_info') THEN
        ALTER TABLE bbs_subject_info RENAME TO x_bbs_topic;
    END IF;
    IF EXISTS (SELECT FROM pg_tables WHERE tablename = 'bbs_comment_info') THEN
        ALTER TABLE bbs_comment_info RENAME TO x_bbs_reply;
    END IF;
END $$;

-- Create control config table if not exists
CREATE TABLE IF NOT EXISTS x_bbs_assemble_control_config (
    id VARCHAR(255) PRIMARY KEY DEFAULT gen_random_uuid(),
    enabled BOOLEAN DEFAULT TRUE,
    max_forum_count INTEGER DEFAULT 1000,
    allow_anonymous BOOLEAN DEFAULT FALSE,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create control section table if not exists
CREATE TABLE IF NOT EXISTS x_bbs_assemble_control_section (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    sort INTEGER DEFAULT 0,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Add deleted_at to x_bbs_topic if not exists
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM information_schema.columns WHERE table_name = 'x_bbs_topic' AND column_name = 'deleted_at') THEN
        ALTER TABLE x_bbs_topic ADD COLUMN deleted_at TIMESTAMP;
    END IF;
    IF NOT EXISTS (SELECT FROM information_schema.columns WHERE table_name = 'x_bbs_forum' AND column_name = 'sort') THEN
        ALTER TABLE x_bbs_forum ADD COLUMN sort INTEGER DEFAULT 0;
    END IF;
    IF NOT EXISTS (SELECT FROM information_schema.columns WHERE table_name = 'x_bbs_section' AND column_name = 'sort') THEN
        ALTER TABLE x_bbs_section ADD COLUMN sort INTEGER DEFAULT 0;
    END IF;
END $$;

-- Ensure indexes exist
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_attribute a JOIN pg_class c ON a.attrelid = c.oid WHERE c.relname = 'x_bbs_topic' AND a.attname = 'forum_id') THEN
    CREATE INDEX IF NOT EXISTS idx_x_bbs_topic_forum ON x_bbs_topic(forum_id);
  END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_x_bbs_topic_author ON x_bbs_topic(author_id);
CREATE INDEX IF NOT EXISTS idx_x_bbs_reply_topic ON x_bbs_reply(subject_id);
DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_attribute a JOIN pg_class c ON a.attrelid = c.oid WHERE c.relname = 'x_bbs_section' AND a.attname = 'forum_id') THEN
    CREATE INDEX IF NOT EXISTS idx_x_bbs_section_forum ON x_bbs_section(forum_id);
  END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_x_bbs_topic_deleted ON x_bbs_topic(deleted_at);
