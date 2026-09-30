-- 106_create_trgm_search_indexes.sql
-- 轮85 配套索引：LIKE/ILIKE 通配符注入转义（轮66）落库后，前缀不确定的
-- '%kw%' 模糊搜索无法走 btree——下列最热搜索列此前全部顺序扫描。
-- pg_trgm GIN 索引使 ILIKE '%kw%' 可走索引（gin_trgm_ops）。
-- 覆盖面仅限用户高频搜索入口：组织人员搜索（desktop 通讯录/移动 contacts）、
-- BBS 主题搜索（移动 search 页/桌面论坛）、会议室/楼栋搜索（会议预订）、
-- 全局脚本/视图/流程定义搜索（桌面设计器全局查找）。

CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- 组织人员搜索（u2_person：name/mobile/email ILIKE）
CREATE INDEX IF NOT EXISTS idx_x_org_person_name_trgm
    ON x_org_person USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_x_org_person_mobile_trgm
    ON x_org_person USING gin (mobile gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_x_org_person_email_trgm
    ON x_org_person USING gin (email gin_trgm_ops);

-- BBS 主题搜索（subject filter/search：title OR content ILIKE）
CREATE INDEX IF NOT EXISTS idx_x_bbs_topic_title_trgm
    ON x_bbs_topic USING gin (title gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_x_bbs_topic_content_trgm
    ON x_bbs_topic USING gin (content gin_trgm_ops);

-- 会议室/楼栋搜索（name/pinyin/address ILIKE）
CREATE INDEX IF NOT EXISTS idx_x_meeting_room_name_trgm
    ON x_meeting_room USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_x_meeting_room_pinyin_trgm
    ON x_meeting_room USING gin (pinyin gin_trgm_ops);

-- 设计器全局查找（query u2 全局搜索 name ILIKE 族的最常用三表）
CREATE INDEX IF NOT EXISTS idx_x_script_name_trgm
    ON x_script USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_x_query_view_name_trgm
    ON x_query_view USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_x_process_definition_name_trgm
    ON x_process_definition USING gin (name gin_trgm_ops);
