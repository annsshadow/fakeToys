-- Seed data for personal module
INSERT INTO auth_person (id, unique_id, name, mobile, email, password_hash, locked, failed_attempts)
VALUES
    ('person-admin', 'admin', 'admin', NULL, NULL, '$2b$12$dummy.hash.for.testing', FALSE, 0)
ON CONFLICT (id) DO NOTHING;

INSERT INTO auth_role (id, name, description)
VALUES
    ('role-admin', 'admin', 'system administrator'),
    ('role-user', 'user', 'regular user')
ON CONFLICT (id) DO NOTHING;

INSERT INTO auth_unit (id, name, parent_id, level)
VALUES
    ('unit-root', 'root', NULL, 0),
    ('unit-dept1', 'department1', 'unit-root', 1)
ON CONFLICT (id) DO NOTHING;
