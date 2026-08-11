-- Rollback for Migration 012: Remove creator_person column from application and script tables
-- Reverses: 012_add_creator_person.sql

BEGIN;

ALTER TABLE x_application DROP COLUMN IF EXISTS creator_person;
ALTER TABLE x_script DROP COLUMN IF EXISTS creator_person;

COMMIT;
