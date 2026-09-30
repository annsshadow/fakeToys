-- 099_create_blob_storage_table.sql
-- BlobStorage 抽象的 PostgreSQL 后端存储表（shared::storage::PgBlobStorage）。
-- 取代 DbBlobStorage 占位：附件/图片等二进制内容以原始字节按不透明 storage_key 独立存取，
-- 使 STORAGE_BACKEND=db（默认）下的上传/下载真实落盘、可回读校验，而非 501 占位。

CREATE TABLE IF NOT EXISTS "x_blob_storage" (
    "storage_key" TEXT PRIMARY KEY,
    "content" BYTEA NOT NULL,
    "create_time" TIMESTAMP DEFAULT NOW()
);
