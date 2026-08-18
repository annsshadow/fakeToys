-- Rollback for Migration 014: Remove sso_client table

BEGIN;

DROP TABLE IF EXISTS sso_client;

COMMIT;
