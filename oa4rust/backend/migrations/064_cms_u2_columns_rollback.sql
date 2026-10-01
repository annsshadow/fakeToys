-- Rollback of 064_cms_u2_columns.sql.
-- Drops the additive columns introduced for the cms_assemble_control
-- plan002 U2 endpoint alignment. Idempotent.

ALTER TABLE "x_cms_data_document"
    DROP COLUMN IF EXISTS "is_top";

ALTER TABLE "x_cms_appinfo"
    DROP COLUMN IF EXISTS "config";
