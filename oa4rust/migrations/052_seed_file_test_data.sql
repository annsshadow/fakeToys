-- Seed data for FILE_FILE table (test environment)
INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, deleted_at)
VALUES
    ('test-id', 'test.txt', 'system', '', 'file', 'txt', 0, 'text/plain', NULL)
ON CONFLICT (id) DO NOTHING;
