-- Migration 071: organization_assemble_express U2 收尾支撑结构
-- 对齐 Java x_organization_assemble_express 剩余端点所需的最小增量：
-- 1) x_org_group.parent_id：群组层级（sub/sup/tree 端点，o2 Group.groupList 递归）
-- 2) x_org_unit."type"：组织类型（unit identity/type、list/types 端点）
-- 3) x_empower.from_identity/to_identity：empower/list/identity/object 的身份维度
-- 4) x_org_empower_log：POST /jaxrs/empowerlog 落库表
-- 均为增量变更（IF NOT EXISTS），向后兼容。

ALTER TABLE "x_org_group" ADD COLUMN IF NOT EXISTS "parent_id" VARCHAR(255);
CREATE INDEX IF NOT EXISTS idx_x_org_group_parent ON x_org_group(parent_id);

ALTER TABLE "x_org_unit" ADD COLUMN IF NOT EXISTS "type" VARCHAR(255);
CREATE INDEX IF NOT EXISTS idx_x_org_unit_type ON x_org_unit(type);

ALTER TABLE "x_empower" ADD COLUMN IF NOT EXISTS "from_identity" VARCHAR(255);
ALTER TABLE "x_empower" ADD COLUMN IF NOT EXISTS "to_identity" VARCHAR(255);
CREATE INDEX IF NOT EXISTS idx_x_empower_from_identity ON x_empower(from_identity);
CREATE INDEX IF NOT EXISTS idx_x_empower_to_identity ON x_empower(to_identity);

CREATE TABLE IF NOT EXISTS x_org_empower_log (
    id VARCHAR(255) PRIMARY KEY,
    application VARCHAR(255),
    process VARCHAR(255),
    work VARCHAR(255),
    from_identity VARCHAR(255) NOT NULL,
    to_identity VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_org_empower_log_from ON x_org_empower_log(from_identity);
