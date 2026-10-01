-- plan002 U2: migration 084 回滚 (与 084_portal_u2_closures.sql 对称)
-- 仅 DROP 本次补建的列；IF EXISTS 保证幂等安全。

ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "flag";
ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "corner_mark";
ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "logo_base64";
ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "mobile_enabled";
ALTER TABLE "x_portal" DROP COLUMN IF EXISTS "permission";

ALTER TABLE "x_portal_page" DROP COLUMN IF EXISTS "category";
ALTER TABLE "x_portal_page" DROP COLUMN IF EXISTS "creator";
ALTER TABLE "x_portal_page" DROP COLUMN IF EXISTS "mobile_content";
ALTER TABLE "x_portal_page" DROP COLUMN IF EXISTS "flag";
ALTER TABLE "x_portal_page" DROP COLUMN IF EXISTS "portal_flag";

ALTER TABLE "x_portal_dict" DROP COLUMN IF EXISTS "deleted_at";
ALTER TABLE "x_portal_dict" DROP COLUMN IF EXISTS "flag";
ALTER TABLE "x_portal_dict" DROP COLUMN IF EXISTS "portal_flag";
ALTER TABLE "x_portal_dict" DROP COLUMN IF EXISTS "portal_id";

ALTER TABLE "x_portal_file" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_portal_file" DROP COLUMN IF EXISTS "portal_flag";
ALTER TABLE "x_portal_file" DROP COLUMN IF EXISTS "application_flag";

ALTER TABLE "x_portal_script" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_portal_script" DROP COLUMN IF EXISTS "imported_content";
ALTER TABLE "x_portal_script" DROP COLUMN IF EXISTS "deleted_at";
ALTER TABLE "x_portal_script" DROP COLUMN IF EXISTS "portal_id";

ALTER TABLE "x_portal_widget" DROP COLUMN IF EXISTS "mobile_config";

ALTER TABLE "x_portal_output" DROP COLUMN IF EXISTS "portal_flag";
ALTER TABLE "x_portal_output" DROP COLUMN IF EXISTS "select_file";

ALTER TABLE "x_portal_design" DROP COLUMN IF EXISTS "content";
ALTER TABLE "x_portal_design" DROP COLUMN IF EXISTS "deleted_at";

ALTER TABLE "x_portal_surface" DROP COLUMN IF EXISTS "html";
ALTER TABLE "x_portal_surface" DROP COLUMN IF EXISTS "category";
ALTER TABLE "x_portal_surface" DROP COLUMN IF EXISTS "template";
ALTER TABLE "x_portal_surface" DROP COLUMN IF EXISTS "deleted_at";

ALTER TABLE "x_portal_input" DROP COLUMN IF EXISTS "content";
