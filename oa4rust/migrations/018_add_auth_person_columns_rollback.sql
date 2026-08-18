-- Rollback for migration 018
ALTER TABLE auth_person
    DROP COLUMN IF EXISTS change_password_time,
    DROP COLUMN IF EXISTS password_expired_time,
    DROP COLUMN IF EXISTS icon,
    DROP COLUMN IF EXISTS job,
    DROP COLUMN IF EXISTS department,
    DROP COLUMN IF EXISTS unit,
    DROP COLUMN IF EXISTS position;

DROP INDEX IF EXISTS idx_auth_person_change_pwd;
