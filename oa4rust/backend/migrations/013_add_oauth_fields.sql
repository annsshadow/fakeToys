-- Migration 013: Add mpwxopenId column to auth_person
-- Stores WeChat Mini Program openid for binding oauth accounts.

BEGIN;

ALTER TABLE auth_person ADD COLUMN IF NOT EXISTS mpwxopenId VARCHAR(255);

CREATE INDEX IF NOT EXISTS idx_auth_person_mpwxopenId ON auth_person(mpwxopenId) WHERE mpwxopenId IS NOT NULL;

COMMIT;
