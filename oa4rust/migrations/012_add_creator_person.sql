-- Migration 012: Add creator_person to application and script tables
-- This enables ownership-based access control (IDOR protection).

BEGIN;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_application') THEN
    ALTER TABLE x_application ADD COLUMN IF NOT EXISTS creator_person TEXT NOT NULL DEFAULT '';
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_script') THEN
    ALTER TABLE x_script ADD COLUMN IF NOT EXISTS creator_person TEXT NOT NULL DEFAULT '';
  END IF;
END $$;

COMMIT;
