-- Seed data for jpush tables (test environment)
INSERT INTO x_jpush_device (id, user_id, platform, token)
VALUES
    ('test-id', 'test-id', 'test', 'test-token')
ON CONFLICT (id) DO NOTHING;

INSERT INTO x_jpush_template (id, name, title, content)
VALUES
    ('test-id', 'test', 'Test Template', 'test content')
ON CONFLICT (id) DO NOTHING;
