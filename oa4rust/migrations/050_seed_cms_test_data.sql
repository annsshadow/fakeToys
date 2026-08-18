-- Seed data for CMS tables (test environment)
INSERT INTO x_query_view (id, name, query_type, view_flag, query_flag, content, creator, create_time)
VALUES
    ('query-view-test-1', 'Test View', 'test', 'test-id', 'test-id', '{"fields": [{"name": "id", "type": "string"}]}', 'system', NOW())
ON CONFLICT (id) DO NOTHING;
