INSERT INTO x_ai_clue (id, person) VALUES ('audit-clue-null','audit-user') ON CONFLICT (id) DO NOTHING;
INSERT INTO x_ai_completion (id, "clueId") VALUES ('audit-comp-null','audit-clue-null') ON CONFLICT (id) DO NOTHING;
INSERT INTO x_ai_conversation (id, title, user_id) VALUES ('audit-conv-null','NULL-row','audit-user') ON CONFLICT (id) DO NOTHING;
SELECT 'clue' AS t, count(*) FROM x_ai_clue
UNION ALL SELECT 'completion', count(*) FROM x_ai_completion
UNION ALL SELECT 'conv', count(*) FROM x_ai_conversation;
