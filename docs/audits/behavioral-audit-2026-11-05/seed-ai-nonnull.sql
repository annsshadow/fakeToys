UPDATE x_ai_clue SET create_time=NOW() WHERE id IN ('clue-admin-1','audit-clue-null');
UPDATE x_ai_conversation SET create_time=NOW() WHERE id IN ('conv-admin-1','audit-conv-null');
UPDATE x_ai_completion SET create_time=NOW(), "generateType"='t' WHERE id='audit-comp-null';
SELECT 'clue' AS t, id, person FROM x_ai_clue
UNION ALL SELECT 'conv', id, user_id FROM x_ai_conversation;
