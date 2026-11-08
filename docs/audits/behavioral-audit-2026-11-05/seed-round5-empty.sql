-- 第五轮审计：空表最小造数（Rust scratch 库 oa_audit_20261105）
-- 用途：把 report-4 的「不可判·空表」转成可判。可重复执行（先删后插）。
-- 归属：admin（受害者，unique_id=admin）；攻击者 audit-user。

DELETE FROM bbs_subject_info WHERE id IN ('r5-subj-admin','r5-subj-attacker');
DELETE FROM bbs_section_info WHERE id = 'r5-sec';
DELETE FROM bbs_forum_info  WHERE id = 'r5-forum';

INSERT INTO bbs_forum_info (id, name, description, create_time, creator, creator_person, disable)
VALUES ('r5-forum', 'R5 forum', 'seed for round5', '2026-11-06 00:00:00', 'admin', 'admin', false);

INSERT INTO bbs_section_info (id, name, forum_id, sort, create_time, creator, creator_person, disable)
VALUES ('r5-sec', 'R5 section', 'r5-forum', 0, '2026-11-06 00:00:00', 'admin', 'admin', false);

INSERT INTO bbs_subject_info
  (id, title, author_id, section_id, content, reply_count, view_count, is_top, disable,
   create_time, creator, creator_person)
VALUES
  ('r5-subj-admin', 'R5 admin subject', 'admin', 'r5-sec', 'secret-ish content', 0, 0, false, false,
   '2026-11-06 00:00:00', 'admin', 'admin'),
  ('r5-subj-attacker', 'R5 attacker subject', 'audit-user', 'r5-sec', 'attacker content', 0, 0, false, false,
   '2026-11-06 00:00:00', 'audit-user', 'audit-user');

-- mind_core_entity 实体表名是 x_mind_base_info / x_mind_folder_info
DELETE FROM x_mind_base_info  WHERE id IN ('r5-mind-admin','r5-mind-attacker');
DELETE FROM x_mind_folder_info WHERE id = 'r5-mfolder';

INSERT INTO x_mind_folder_info (id, name, parent_id, order_number, description, creator)
VALUES ('r5-mfolder', 'R5 folder', NULL, 0, 'seed', 'admin');

INSERT INTO x_mind_base_info (id, name, folder_id, description, creator, create_time)
VALUES
  ('r5-mind-admin', 'R5 admin mind', 'r5-mfolder', 'admin private mind', 'admin', '2026-11-06 00:00:00'),
  ('r5-mind-attacker', 'R5 attacker mind', 'r5-mfolder', 'attacker mind', 'audit-user', '2026-11-06 00:00:00');
