DELETE FROM x_mind WHERE id IN ('mind-admin-secret','mind-audit-own');
INSERT INTO x_mind (id, name, content, creator, create_time, shared, file_version, folder_id)
VALUES ('mind-admin-secret','admin private mind','secret','person-admin',NOW(), false, 1, 'folder-1');
INSERT INTO x_mind (id, name, content, creator, create_time, shared, file_version, folder_id)
VALUES ('mind-audit-own','audit user mind','own','audit-u1',NOW(), false, 1, 'folder-1');
UPDATE auth_person SET password_hash=(SELECT password_hash FROM auth_person WHERE id='person-admin') WHERE id='audit-u1';
SELECT id, creator, folder_id FROM x_mind ORDER BY id;
