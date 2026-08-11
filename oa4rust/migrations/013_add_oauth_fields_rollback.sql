-- Rollback for Migration 013: Remove mpwxopenId column from auth_person

BEGIN;

DROP INDEX IF EXISTS idx_auth_person_mpwxopenId;
ALTER TABLE auth_person DROP COLUMN IF EXISTS mpwxopenId;

COMMIT;
