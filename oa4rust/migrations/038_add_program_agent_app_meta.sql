-- Real-ize routed program_center write handlers.
--
-- The routed handlers application_create / application_save persist a
-- `description` column on x_applications, and agent_create / agent_save persist
-- `name`, `flag` and `description` on x_program_agent. Those columns were
-- missing from the generated schema (032), so the writes failed at runtime.
-- Add them (nullable, additive) so the handlers perform genuine DB writes.

ALTER TABLE IF EXISTS "x_applications"
    ADD COLUMN IF NOT EXISTS "description" TEXT;

ALTER TABLE IF EXISTS "x_program_agent"
    ADD COLUMN IF NOT EXISTS "name" TEXT,
    ADD COLUMN IF NOT EXISTS "flag" TEXT,
    ADD COLUMN IF NOT EXISTS "description" TEXT;

-- config_save relies on INSERT ... ON CONFLICT (key); a unique key is required.
CREATE UNIQUE INDEX IF NOT EXISTS "uq_x_program_config_key"
    ON "x_program_config" ("key");
