UPDATE x_ai_clue SET create_time=NOW() WHERE id='audit-clue-null';
UPDATE x_ai_completion SET create_time=NOW(), "generateType"='type-a' WHERE id='audit-comp-null';
UPDATE x_ai_conversation SET create_time=NOW() WHERE id='audit-conv-null';
SELECT 'clue' AS t, create_time IS NULL AS null_ct, "generateType" IS NULL AS null_gen FROM x_ai_clue WHERE id='audit-clue-null';
