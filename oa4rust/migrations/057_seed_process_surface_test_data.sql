-- Seed data for x_process_surface table (test environment)
INSERT INTO x_process_surface (id, name, category, content, version, creator, create_time, update_time)
VALUES
    ('test-id', 'Test Surface', 'test', '{}', '1.0', 'system', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
