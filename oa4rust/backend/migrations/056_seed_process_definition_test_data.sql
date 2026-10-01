-- Seed data for x_process_definition table (test environment)
INSERT INTO x_process_definition (id, name, category, process_definition, version, creator)
VALUES
    ('test-id', 'Test Flow', 'test', '{}', 1, 'system')
ON CONFLICT (id) DO NOTHING;
