-- Rollback seed data for x_process_definition table
DELETE FROM x_process_definition WHERE id = 'test-id';
