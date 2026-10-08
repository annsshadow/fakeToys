-- W7 script designer: keep version snapshots restorable.
ALTER TABLE IF EXISTS "x_portal_script_version"
    ADD COLUMN IF NOT EXISTS "content" TEXT;
