-- Migration 012: Add creator_person to application and script tables
-- This enables ownership-based access control (IDOR protection).

BEGIN;

ALTER TABLE x_application ADD COLUMN IF NOT EXISTS creator_person TEXT NOT NULL DEFAULT '';
ALTER TABLE x_script ADD COLUMN IF NOT EXISTS creator_person TEXT NOT NULL DEFAULT '';

COMMIT;
