-- Rollback seed data for x_message_consume table
DELETE FROM x_message_consume WHERE id = 'test-id';
