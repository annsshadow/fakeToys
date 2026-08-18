-- 009_person_group_tables.sql
-- 用户组成员关联表：auth_person_group
-- 用于权限系统的 Group 级别检查，支持按用户组授权访问控制。
CREATE TABLE IF NOT EXISTS auth_person_group (
    person_id VARCHAR(255) NOT NULL,
    group_id  VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (person_id, group_id),
    CONSTRAINT fk_person_group_person FOREIGN KEY (person_id) REFERENCES auth_person(id) ON DELETE CASCADE,
    CONSTRAINT fk_person_group_group FOREIGN KEY (group_id) REFERENCES auth_group(id) ON DELETE CASCADE
);

-- Composite PK already creates implicit indexes; explicit single-column indexes are redundant
-- but kept for query performance on person_id/group_id lookups

