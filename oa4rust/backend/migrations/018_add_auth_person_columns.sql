-- Migration 018: Add missing columns to auth_person
-- Required by: R1 (passwordExpired check), R2 (full Person fields), R8 (icon field)

ALTER TABLE auth_person
    ADD COLUMN IF NOT EXISTS change_password_time TIMESTAMP,
    ADD COLUMN IF NOT EXISTS password_expired_time TIMESTAMP,
    ADD COLUMN IF NOT EXISTS icon TEXT DEFAULT '',
    ADD COLUMN IF NOT EXISTS job VARCHAR(255),
    ADD COLUMN IF NOT EXISTS department VARCHAR(255),
    ADD COLUMN IF NOT EXISTS unit VARCHAR(255),
    ADD COLUMN IF NOT EXISTS position VARCHAR(255);

-- Add index for password expiration checks
CREATE INDEX IF NOT EXISTS idx_auth_person_change_pwd ON auth_person(change_password_time);
