-- Seed data for x_component table (test environment)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM x_component WHERE id = 'test-id') THEN
        INSERT INTO x_component (id, name, title, type, visible, order_number, path, icon_path, deleted_at)
        VALUES ('test-id', 'test-id', 'Test Component', 'test', true, '1', '/test', '/test.png', NULL);
    END IF;
END $$;
