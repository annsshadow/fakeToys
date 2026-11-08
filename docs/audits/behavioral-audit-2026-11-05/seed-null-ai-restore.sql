UPDATE x_ai_clue SET create_time=NULL WHERE id='audit-clue-null';
UPDATE x_ai_completion SET create_time=NULL, "generateType"=NULL WHERE id='audit-comp-null';
UPDATE x_ai_conversation SET create_time=NULL WHERE id='audit-conv-null';
