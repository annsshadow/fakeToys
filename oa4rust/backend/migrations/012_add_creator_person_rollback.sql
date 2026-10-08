-- Rollback for Migration 012: Remove creator_person column from application and script tables
-- Reverses: 012_add_creator_person.sql

BEGIN;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_application') THEN
    ALTER TABLE x_application DROP COLUMN IF EXISTS creator_person;
  END IF;
END $$;

DO $$ BEGIN
  IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'x_script') THEN
    ALTER TABLE x_script DROP COLUMN IF EXISTS creator_person;
  END IF;
END $$;

COMMIT;
