-- plan002 U2: message_assemble_communicate missing-endpoint tables & columns.
-- x_message_ws_session -> jaxrs/ws family (POST /ws, count/person, list/person[/current/node])
-- x_message_conversation_ext -> im virtual delete of single conversation (per-person soft delete)
-- Additive columns fix latent schema gaps hit by existing routed reads/writes:
--   x_message_conversation.read_status / read_time / last_message_time
--     (send_message updates last_message_time; read handlers update read_status)
--   x_message_mass.enabled / type / title / content / sender / body / send_person_list
--     (mass_enable_type, mass_id, POST /mass)
--   x_message_instant.consumed / body / type / person / title
--     (instant consumed mockputtopost; POST /connector instant persistence)
-- Idempotent: safe to run repeatedly.

CREATE TABLE IF NOT EXISTS "x_message_ws_session" (
    "id" TEXT PRIMARY KEY,
    "person" TEXT NOT NULL,
    "node" TEXT,
    "connected_at" TEXT,
    "disconnected_at" TEXT
);

CREATE INDEX IF NOT EXISTS "idx_x_message_ws_session_person_open"
    ON "x_message_ws_session" ("person")
    WHERE "disconnected_at" IS NULL;

CREATE TABLE IF NOT EXISTS "x_message_conversation_ext" (
    "id" TEXT PRIMARY KEY,
    "conversation_id" TEXT,
    "person" TEXT,
    "is_deleted" BOOLEAN NOT NULL DEFAULT false,
    "last_delete_time" TEXT,
    "last_read_time" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);

CREATE INDEX IF NOT EXISTS "idx_x_message_conversation_ext_conv_person"
    ON "x_message_conversation_ext" ("conversation_id", "person");

ALTER TABLE IF EXISTS "x_message_conversation"
    ADD COLUMN IF NOT EXISTS "read_status" TEXT;
ALTER TABLE IF EXISTS "x_message_conversation"
    ADD COLUMN IF NOT EXISTS "read_time" TEXT;
ALTER TABLE IF EXISTS "x_message_conversation"
    ADD COLUMN IF NOT EXISTS "last_message_time" TEXT;

ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "enabled" BOOLEAN DEFAULT true;
ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "type" TEXT;
ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "title" TEXT;
ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "sender" TEXT;
ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "body" TEXT;
ALTER TABLE IF EXISTS "x_message_mass"
    ADD COLUMN IF NOT EXISTS "send_person_list" TEXT;

ALTER TABLE IF EXISTS "x_message_instant"
    ADD COLUMN IF NOT EXISTS "consumed" BOOLEAN DEFAULT false;
ALTER TABLE IF EXISTS "x_message_instant"
    ADD COLUMN IF NOT EXISTS "body" TEXT;
ALTER TABLE IF EXISTS "x_message_instant"
    ADD COLUMN IF NOT EXISTS "type" TEXT;
ALTER TABLE IF EXISTS "x_message_instant"
    ADD COLUMN IF NOT EXISTS "person" TEXT;
ALTER TABLE IF EXISTS "x_message_instant"
    ADD COLUMN IF NOT EXISTS "title" TEXT;

-- Latent gaps hit by existing routed handlers (im_msg_revoke_id / im_msg_upload):
ALTER TABLE IF EXISTS "x_message"
    ADD COLUMN IF NOT EXISTS "revoked" BOOLEAN DEFAULT false;
ALTER TABLE IF EXISTS "x_message"
    ADD COLUMN IF NOT EXISTS "revoke_time" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "message_id" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "conversation_id" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "file_url" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "file_name" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "file_size" TEXT;
ALTER TABLE IF EXISTS "x_message_file"
    ADD COLUMN IF NOT EXISTS "type" TEXT;
