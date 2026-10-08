-- W10 IM rich media: store real file bytes for binary download.
-- Upload persists base64 content + mime; /im/msg/download/{id} serves the bytes.
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "mime" TEXT;
