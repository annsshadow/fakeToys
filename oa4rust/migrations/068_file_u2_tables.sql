-- plan002 U2: file_assemble_control 端点全量闭合
-- FILE_SHARE：share 族端点（create/get/delete/shield/password/saveToFolder）的真实存储。
-- 字段对齐 Java com.x.file.core.entity.personal.Share（validTime=失效时间，shield 复用为已失效）。

CREATE TABLE IF NOT EXISTS FILE_SHARE (
    id VARCHAR(255) PRIMARY KEY,
    person VARCHAR(255) NOT NULL,
    name VARCHAR(500) DEFAULT '',
    file_id VARCHAR(255) NOT NULL,
    file_type VARCHAR(100) DEFAULT '',
    extension VARCHAR(50),
    length BIGINT DEFAULT 0,
    share_type VARCHAR(50) DEFAULT 'password',
    password VARCHAR(255) DEFAULT '',
    valid_time TIMESTAMP,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_file_share_person ON FILE_SHARE(person);
CREATE INDEX IF NOT EXISTS idx_file_share_file ON FILE_SHARE(file_id);

-- BlobStorage 接入：FS 后端落盘后的 blob key（db 行内 content 模式下为 NULL）。
ALTER TABLE FILE_FILE ADD COLUMN IF NOT EXISTS storage_key VARCHAR(500);
