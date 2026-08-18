-- Rollback for migration 009_person_group_tables.sql
-- Drop auth_person_group table and associated constraints

BEGIN;

DROP TABLE IF EXISTS auth_person_group;

COMMIT;
