-- Migration 066: organization_assemble_express U2 支撑列
-- o2 的 Identity 实体以 person（所属人员）与 major（主身份）为核心字段，
-- 而 022 建立的 x_org_identity 缺少这两列，导致 express 模块的
-- identity↔person 关联端点无法用真实 SQL 实现。均为增量列，向后兼容。

ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "person_id" VARCHAR(255);
ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "major" BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX IF NOT EXISTS idx_x_org_identity_person ON x_org_identity(person_id);
