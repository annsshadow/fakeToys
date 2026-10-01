-- Seed data for x_message_consume table (test environment)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM x_message_consume WHERE id = 'test-id') THEN
        INSERT INTO x_message_consume (id, consume, content, sender, type, create_time)
        VALUES ('test-id', 'test-id', 'test', 'test', 'test-id', NOW());
    END IF;
END $$;
