-- 031: x_org_identity (created by migration 022) has no type column, but the
-- org_identity SeaORM entity selects `type_` (column_name override). Add it.
-- The entity field type_ maps to column "type_" via #[sea_orm(column_name = "type_")].

ALTER TABLE "x_org_identity" ADD COLUMN IF NOT EXISTS "type_" TEXT NOT NULL DEFAULT '';
