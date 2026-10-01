-- Rollback of 063_message_communicate_u2_tables.sql.
-- Only removes objects introduced by 063 (idempotent: safe to run repeatedly).

DROP TABLE IF EXISTS "x_message_conversation_ext";
DROP TABLE IF EXISTS "x_message_ws_session";

ALTER TABLE IF EXISTS "x_message_conversation" DROP COLUMN IF EXISTS "read_status";
ALTER TABLE IF EXISTS "x_message_conversation" DROP COLUMN IF EXISTS "read_time";
ALTER TABLE IF EXISTS "x_message_conversation" DROP COLUMN IF EXISTS "last_message_time";

ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "enabled";
ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "type";
ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "title";
ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "content";
ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "sender";
ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "body";
ALTER TABLE IF EXISTS "x_message_mass" DROP COLUMN IF EXISTS "send_person_list";

ALTER TABLE IF EXISTS "x_message_instant" DROP COLUMN IF EXISTS "consumed";
ALTER TABLE IF EXISTS "x_message_instant" DROP COLUMN IF EXISTS "body";
ALTER TABLE IF EXISTS "x_message_instant" DROP COLUMN IF EXISTS "type";
ALTER TABLE IF EXISTS "x_message_instant" DROP COLUMN IF EXISTS "person";
ALTER TABLE IF EXISTS "x_message_instant" DROP COLUMN IF EXISTS "title";

ALTER TABLE IF EXISTS "x_message" DROP COLUMN IF EXISTS "revoked";
ALTER TABLE IF EXISTS "x_message" DROP COLUMN IF EXISTS "revoke_time";
ALTER TABLE IF EXISTS "x_message_file" DROP COLUMN IF EXISTS "message_id";
ALTER TABLE IF EXISTS "x_message_file" DROP COLUMN IF EXISTS "conversation_id";
ALTER TABLE IF EXISTS "x_message_file" DROP COLUMN IF EXISTS "file_url";
ALTER TABLE IF EXISTS "x_message_file" DROP COLUMN IF EXISTS "file_name";
ALTER TABLE IF EXISTS "x_message_file" DROP COLUMN IF EXISTS "file_size";
ALTER TABLE IF EXISTS "x_message_file" DROP COLUMN IF EXISTS "type";
