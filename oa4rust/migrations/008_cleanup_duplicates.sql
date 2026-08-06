-- 008: Archive duplicate migrations 003 and 004
--
-- 003_personal_tables.sql and 004_seed_personal_data.sql were duplicates of
-- 001_create_auth_tables.sql and 002_seed_auth_data.sql with minor column additions
-- (icon, avatar, deleted_at). Those columns are already covered by 005_org_tables.sql.
--
-- To preserve migration history for existing environments, 003 and 004 were moved
-- to migrations/archive/ rather than deleted. New environments should apply:
--   001_create_auth_tables.sql
--   002_seed_auth_data.sql
--   005_org_tables.sql
--   006_org_updated_at.sql
--   007_admin_seed.sql
--   007_secret_config.sql
--
-- This migration is a no-op for database schema; it documents the cleanup decision.

SELECT 1;
