-- Rollback for migration 023: CMS Assemble Control tables

DROP TABLE IF EXISTS x_cms_form_v2;
DROP TABLE IF EXISTS x_cms_document_cipher;
DROP TABLE IF EXISTS x_cms_data_document_field;
DROP TABLE IF EXISTS x_cms_data_document;
DROP TABLE IF EXISTS x_cms_viewrecord;
DROP TABLE IF EXISTS x_cms_viewfieldconfig;
DROP TABLE IF EXISTS x_cms_viewcategory;
DROP TABLE IF EXISTS x_cms_view;
DROP TABLE IF EXISTS x_cms_surface_appdict;
DROP TABLE IF EXISTS x_cms_searchfilter;
DROP TABLE IF EXISTS x_cms_script;
DROP TABLE IF EXISTS x_cms_permission;
DROP TABLE IF EXISTS x_cms_output;
DROP TABLE IF EXISTS x_cms_log;
DROP TABLE IF EXISTS x_cms_form_field;
DROP TABLE IF EXISTS x_cms_form;
DROP TABLE IF EXISTS x_cms_fileinfo;
DROP TABLE IF EXISTS x_cms_file;
DROP TABLE IF EXISTS x_cms_correlation;
DROP TABLE IF EXISTS x_cms_commend;
DROP TABLE IF EXISTS x_cms_comment;
DROP TABLE IF EXISTS x_cms_categoryinfo;
DROP TABLE IF EXISTS x_cms_appinfo;
