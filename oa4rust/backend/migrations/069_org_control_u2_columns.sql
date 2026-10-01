-- 067: organization_assemble_control U2 端点闭合所需列与表
-- 依据 Java x_organization_assemble_control 实体语义补齐既有表缺失列，
-- 并新增 x_org_personcard（对齐 Java PersonCard 实体最小字段集）。

ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "password" TEXT;
ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "status" VARCHAR(50);
ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "status_des" TEXT;
ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "lock_expired_time" TIMESTAMP;
ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "password_expired_time" TIMESTAMP;
ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "icon" TEXT DEFAULT '';
ALTER TABLE "x_org_person" ADD COLUMN IF NOT EXISTS "pinyin_initial" TEXT;

ALTER TABLE "x_org_unit" ADD COLUMN IF NOT EXISTS "pinyin_initial" TEXT;
ALTER TABLE "x_org_group" ADD COLUMN IF NOT EXISTS "pinyin_initial" TEXT;
ALTER TABLE "x_org_role" ADD COLUMN IF NOT EXISTS "pinyin_initial" TEXT;

-- Identity 与 Person 的归属关系（Java Identity.person）
ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "person_id" VARCHAR(255);
ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "pinyin_initial" TEXT;
CREATE INDEX IF NOT EXISTS idx_x_org_identity_person ON x_org_identity(person_id);

CREATE INDEX IF NOT EXISTS idx_x_org_person_status ON x_org_person(status);

CREATE TABLE IF NOT EXISTS x_org_personcard (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    group_type VARCHAR(255),
    distinguished_name VARCHAR(255),
    mobile VARCHAR(50),
    office_phone VARCHAR(255),
    address TEXT,
    description TEXT,
    status VARCHAR(50),
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_org_personcard_name ON x_org_personcard(name);
