-- 106_create_trgm_search_indexes_rollback.sql

DROP INDEX IF EXISTS idx_x_org_person_name_trgm;
DROP INDEX IF EXISTS idx_x_org_person_mobile_trgm;
DROP INDEX IF EXISTS idx_x_org_person_email_trgm;
DROP INDEX IF EXISTS idx_x_bbs_topic_title_trgm;
DROP INDEX IF EXISTS idx_x_bbs_topic_content_trgm;
DROP INDEX IF EXISTS idx_x_meeting_room_name_trgm;
DROP INDEX IF EXISTS idx_x_meeting_room_pinyin_trgm;
DROP INDEX IF EXISTS idx_x_script_name_trgm;
DROP INDEX IF EXISTS idx_x_query_view_name_trgm;
DROP INDEX IF EXISTS idx_x_process_definition_name_trgm;

-- pg_trgm 扩展不回滚（可能被库内其他对象依赖）。
