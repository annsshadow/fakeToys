-- plan002 U2: meeting_assemble_control 端点全量闭合
-- 1) x_meeting_attachment: Java Attachment 实体对应表（上传/下载/元数据管理）
-- 2) x_meeting.completed_time: ActionEditCompleteTime 落点列
-- 3) x_meeting_room_photo.room_id: room setPhoto 关联列（parity 表缺业务键）

CREATE TABLE IF NOT EXISTS "x_meeting_attachment" (
    id VARCHAR(255) PRIMARY KEY,
    meeting_id VARCHAR(255) NOT NULL,
    person VARCHAR(255),
    file_name VARCHAR(500),
    extension VARCHAR(50),
    mime_type VARCHAR(255),
    length BIGINT DEFAULT 0,
    summary BOOLEAN DEFAULT FALSE,
    content TEXT,
    storage_key TEXT,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_meeting_attachment_meeting ON x_meeting_attachment(meeting_id);

ALTER TABLE "x_meeting" ADD COLUMN IF NOT EXISTS "completed_time" TIMESTAMP;
ALTER TABLE "x_meeting_room_photo" ADD COLUMN IF NOT EXISTS "room_id" VARCHAR(255);
CREATE INDEX IF NOT EXISTS idx_meeting_room_photo_room ON x_meeting_room_photo(room_id);
