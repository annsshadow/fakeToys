-- Rollback of 067_bbs_u2_tables.sql.
-- Drops the tables/columns introduced for the bbs_assemble_control
-- plan002 U2 endpoint alignment. Idempotent.

DROP TABLE IF EXISTS "x_bbs_user_info";
DROP TABLE IF EXISTS "x_bbs_vote_record";
DROP TABLE IF EXISTS "x_bbs_permission";
DROP TABLE IF EXISTS "x_bbs_role_bind";
DROP TABLE IF EXISTS "x_bbs_role";
DROP TABLE IF EXISTS "x_bbs_config_setting";
DROP TABLE IF EXISTS "x_bbs_subject_attachment";
DROP TABLE IF EXISTS "x_bbs_attachment";

ALTER TABLE "x_bbs_section" DROP COLUMN IF EXISTS "parent_id";
ALTER TABLE "x_bbs_shutup" DROP COLUMN IF EXISTS "reason";
ALTER TABLE "x_bbs_reply" DROP COLUMN IF EXISTS "accepted";

ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "subject_grade";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "subject_type";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "section_name";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "vote_count";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "accept_reply_id";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "completed";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "locked";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "top_to_section";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "top_to_main_section";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "top_to_forum";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "top_to_bbs";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "is_recommend";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "is_original";
ALTER TABLE "x_bbs_topic" DROP COLUMN IF EXISTS "is_cream";
