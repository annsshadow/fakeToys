-- 095_create_person_signature_face_tables.sql
-- 补齐 /jaxrs/person/signature/save 与 /jaxrs/personal/face/list 斜杠路径家族所需的表。
-- 签名（person_signature）与人脸特征（person_face）均为个人扩展数据。

CREATE TABLE IF NOT EXISTS "x_person_signature" (
    "id" TEXT PRIMARY KEY,
    "person_id" TEXT NOT NULL,
    "signature" TEXT,
    "mime_type" TEXT,
    "size" BIGINT,
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "x_person_face" (
    "id" TEXT PRIMARY KEY,
    "person_id" TEXT NOT NULL,
    "face_name" TEXT,
    "face_type" TEXT,
    "feature" TEXT,
    "status" TEXT DEFAULT 'active',
    "creator" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP
);
