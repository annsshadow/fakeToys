-- Seed data for x_hotpic table (test environment)
INSERT INTO x_hotpic (id, application, info_id, title, image_url, creator, deleted_at)
VALUES
    ('test-id', 'test', 'test-id', 'Test Hotpic', '', 'system', NULL)
ON CONFLICT (id) DO NOTHING;
