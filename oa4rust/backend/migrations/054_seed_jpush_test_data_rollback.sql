-- Rollback seed data for jpush tables
DELETE FROM x_jpush_device WHERE id = 'test-id';
DELETE FROM x_jpush_template WHERE id = 'test-id';
