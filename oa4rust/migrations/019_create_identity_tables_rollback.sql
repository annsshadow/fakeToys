-- Rollback for migration 019
DROP INDEX IF EXISTS idx_auth_person_identity_identity;
DROP INDEX IF EXISTS idx_auth_person_identity_person;
DROP INDEX IF EXISTS idx_auth_identity_name;
DROP TABLE IF EXISTS auth_person_identity;
DROP TABLE IF EXISTS auth_identity;
