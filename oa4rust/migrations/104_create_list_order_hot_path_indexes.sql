-- 104_create_list_order_hot_path_indexes.sql
-- 轮33 配套索引：ORDER BY create_time 列上的 ::text 转换已全仓移除（使 btree
-- 索引可用），但下列最热列表表此前根本没有 create_time 索引——补齐后
-- "WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT n" 可走索引避免全表排序。

-- 人员分页/游标列表（u2_person：WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT）
CREATE INDEX IF NOT EXISTS idx_x_org_person_deleted_create_time
    ON x_org_person (deleted_at, create_time DESC);

-- CMS 点赞列表（WHERE doc_id = ? AND deleted_at IS NULL ORDER BY create_time DESC）
CREATE INDEX IF NOT EXISTS idx_x_cms_commend_doc_create_time
    ON x_cms_commend (doc_id, deleted_at, create_time DESC);

-- 图片轮播列表（WHERE deleted_at IS NULL ORDER BY create_time DESC）
CREATE INDEX IF NOT EXISTS idx_x_hotpic_deleted_create_time
    ON x_hotpic (deleted_at, create_time DESC);

-- 单条最新配置（ORDER BY create_time LIMIT 1）
CREATE INDEX IF NOT EXISTS idx_x_cms_assemble_control_config_create_time
    ON x_cms_assemble_control_config (create_time);

-- 公告列表（WHERE deleted_at IS NULL ORDER BY create_time DESC）
CREATE INDEX IF NOT EXISTS idx_x_ai_ann_deleted_create_time
    ON x_ai_ann (deleted_at, create_time DESC);
