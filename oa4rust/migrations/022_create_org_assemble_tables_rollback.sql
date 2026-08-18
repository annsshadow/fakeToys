-- Rollback for 022_create_org_assemble_tables.sql
DROP TABLE IF EXISTS x_org_login_record CASCADE;
DROP TABLE IF EXISTS x_org_import_result CASCADE;
DROP TABLE IF EXISTS x_org_export CASCADE;
DROP TABLE IF EXISTS x_org_group_role CASCADE;
DROP TABLE IF EXISTS x_org_group_member CASCADE;
DROP TABLE IF EXISTS x_org_person_attribute CASCADE;
DROP TABLE IF EXISTS x_org_unit_attribute CASCADE;
DROP TABLE IF EXISTS x_org_permission_setting CASCADE;
DROP TABLE IF EXISTS x_org_duty CASCADE;
DROP TABLE IF EXISTS x_org_identity CASCADE;
DROP TABLE IF EXISTS x_org_group CASCADE;
DROP TABLE IF EXISTS x_org_person CASCADE;
DROP TABLE IF EXISTS x_org_unit CASCADE;
DROP TABLE IF EXISTS x_org_role CASCADE;
