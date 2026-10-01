-- plan002 U2: cms_assemble_control missing-endpoint columns.
-- Supports the Java-alignment endpoints added in this round:
--   x_cms_data_document.is_top -> GET /document/{id}/top and /unTop
--     (DocumentAction topDocument / unTopDocument persist an isTop flag)
--   x_cms_appinfo.config       -> POST /appconfig/{appId}, GET /appconfig/{id}
--     (AppInfoConfigAction stores the app-level config JSON on the app row)
-- Idempotent: safe to run repeatedly. Follows 060_add_view_count.sql precedent.
-- Rollback file: 064_cms_u2_columns_rollback.sql

ALTER TABLE "x_cms_data_document"
    ADD COLUMN IF NOT EXISTS "is_top" BOOLEAN NOT NULL DEFAULT false;

ALTER TABLE "x_cms_appinfo"
    ADD COLUMN IF NOT EXISTS "config" TEXT;
