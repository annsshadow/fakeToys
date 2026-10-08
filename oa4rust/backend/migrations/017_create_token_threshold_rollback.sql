-- Rollback for migration 017
DROP INDEX IF EXISTS idx_auth_token_threshold_person;
DROP TABLE IF EXISTS auth_token_threshold;
