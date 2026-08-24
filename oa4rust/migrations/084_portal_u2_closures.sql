-- plan002 U2: portal assemble surface/designer gap-closure 缺失列补建 (幂等)
-- 仅补建既有 x_portal_* 表中被本模块 handler 引用但尚不存在的列，
-- 使既有 u2 handler 与新补 endpoint 在真实 PG 上可运行。全部 IF NOT EXISTS。

ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "flag" TEXT;
ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "corner_mark" TEXT;
ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "logo_base64" TEXT;
ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "mobile_enabled" BOOLEAN;
ALTER TABLE "x_portal" ADD COLUMN IF NOT EXISTS "permission" TEXT;

ALTER TABLE "x_portal_page" ADD COLUMN IF NOT EXISTS "category" TEXT;
ALTER TABLE "x_portal_page" ADD COLUMN IF NOT EXISTS "creator" TEXT;
ALTER TABLE "x_portal_page" ADD COLUMN IF NOT EXISTS "mobile_content" TEXT;
ALTER TABLE "x_portal_page" ADD COLUMN IF NOT EXISTS "flag" TEXT;
ALTER TABLE "x_portal_page" ADD COLUMN IF NOT EXISTS "portal_flag" TEXT;

ALTER TABLE "x_portal_dict" ADD COLUMN IF NOT EXISTS "deleted_at" TIMESTAMP WITHOUT TIME ZONE;
ALTER TABLE "x_portal_dict" ADD COLUMN IF NOT EXISTS "flag" TEXT;
ALTER TABLE "x_portal_dict" ADD COLUMN IF NOT EXISTS "portal_flag" TEXT;
ALTER TABLE "x_portal_dict" ADD COLUMN IF NOT EXISTS "portal_id" TEXT;

ALTER TABLE "x_portal_file" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_portal_file" ADD COLUMN IF NOT EXISTS "portal_flag" TEXT;
ALTER TABLE "x_portal_file" ADD COLUMN IF NOT EXISTS "application_flag" TEXT;

ALTER TABLE "x_portal_script" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_portal_script" ADD COLUMN IF NOT EXISTS "imported_content" TEXT;
ALTER TABLE "x_portal_script" ADD COLUMN IF NOT EXISTS "deleted_at" TIMESTAMP WITHOUT TIME ZONE;
ALTER TABLE "x_portal_script" ADD COLUMN IF NOT EXISTS "portal_id" TEXT;

ALTER TABLE "x_portal_widget" ADD COLUMN IF NOT EXISTS "mobile_config" TEXT;

ALTER TABLE "x_portal_output" ADD COLUMN IF NOT EXISTS "portal_flag" TEXT;
ALTER TABLE "x_portal_output" ADD COLUMN IF NOT EXISTS "select_file" TEXT;

ALTER TABLE "x_portal_design" ADD COLUMN IF NOT EXISTS "content" TEXT;
ALTER TABLE "x_portal_design" ADD COLUMN IF NOT EXISTS "deleted_at" TIMESTAMP WITHOUT TIME ZONE;

ALTER TABLE "x_portal_surface" ADD COLUMN IF NOT EXISTS "html" TEXT;
ALTER TABLE "x_portal_surface" ADD COLUMN IF NOT EXISTS "category" TEXT;
ALTER TABLE "x_portal_surface" ADD COLUMN IF NOT EXISTS "template" TEXT;
ALTER TABLE "x_portal_surface" ADD COLUMN IF NOT EXISTS "deleted_at" TIMESTAMP WITHOUT TIME ZONE;

ALTER TABLE "x_portal_input" ADD COLUMN IF NOT EXISTS "content" TEXT;
