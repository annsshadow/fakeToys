-- tables: 82
-- 第六轮 path-IDOR 造数（自有 scratch 库 oa_audit_20261105）
-- 覆盖本轮实测越权的表；受害者统一置为 person-admin，攻击者为 audit-user / audit-user2。
-- 可复跑：固定 id = r6v-<表名>（uuid 表用 md5(<id>)::uuid 派生稳定值），先删后插。
-- 用法：docker cp 本文件 lyp-pg:/tmp/ && docker exec lyp-pg psql -U postgres -d oa_audit_20261105 -f /tmp/seed-round6-idor.sql

DELETE FROM "x_ai_mcp_config" WHERE "id" = 'r6v-x_ai_mcp_config';
INSERT INTO "x_ai_mcp_config" ("id","creator") VALUES ('r6v-x_ai_mcp_config','person-admin');

DELETE FROM "x_attendance_admin" WHERE "id" = 'r6v-x_attendance_admin';
INSERT INTO "x_attendance_admin" ("id","person_id","creator") VALUES ('r6v-x_attendance_admin','person-admin','person-admin');

DELETE FROM "x_attendance_appeal_info" WHERE "id" = 'r6v-x_attendance_appeal_info';
INSERT INTO "x_attendance_appeal_info" ("id","person_id","creator") VALUES ('r6v-x_attendance_appeal_info','person-admin','person-admin');

DELETE FROM "x_attendance_detail" WHERE "id" = 'r6v-x_attendance_detail';
INSERT INTO "x_attendance_detail" ("id","person_id","creator") VALUES ('r6v-x_attendance_detail','person-admin','person-admin');

DELETE FROM "x_attendance_employee_config" WHERE "id" = 'r6v-x_attendance_employee_config';
INSERT INTO "x_attendance_employee_config" ("id","person_id","creator") VALUES ('r6v-x_attendance_employee_config','person-admin','person-admin');

DELETE FROM "x_attendance_import_file_info" WHERE "id" = 'r6v-x_attendance_import_file_info';
INSERT INTO "x_attendance_import_file_info" ("id","creator") VALUES ('r6v-x_attendance_import_file_info','person-admin');

DELETE FROM "x_attendance_schedule_setting" WHERE "id" = 'r6v-x_attendance_schedule_setting';
INSERT INTO "x_attendance_schedule_setting" ("id","creator") VALUES ('r6v-x_attendance_schedule_setting','person-admin');

DELETE FROM "x_attendance_selfholiday" WHERE "id" = 'r6v-x_attendance_selfholiday';
INSERT INTO "x_attendance_selfholiday" ("id","person_id","creator") VALUES ('r6v-x_attendance_selfholiday','person-admin','person-admin');

DELETE FROM "x_attendance_setting" WHERE "id" = 'r6v-x_attendance_setting';
INSERT INTO "x_attendance_setting" ("id","creator") VALUES ('r6v-x_attendance_setting','person-admin');

DELETE FROM "x_attendance_statistical_cycle" WHERE "id" = 'r6v-x_attendance_statistical_cycle';
INSERT INTO "x_attendance_statistical_cycle" ("id","creator") VALUES ('r6v-x_attendance_statistical_cycle','person-admin');

DELETE FROM "x_attendance_statistic_require_log" WHERE "id" = 'r6v-x_attendance_statistic_require_log';
INSERT INTO "x_attendance_statistic_require_log" ("id","creator") VALUES ('r6v-x_attendance_statistic_require_log','person-admin');

DELETE FROM "x_attendance_workday_config" WHERE "id" = 'r6v-x_attendance_workday_config';
INSERT INTO "x_attendance_workday_config" ("id","creator") VALUES ('r6v-x_attendance_workday_config','person-admin');

DELETE FROM "x_attendance_v2_appeal_info" WHERE "id" = 'r6v-x_attendance_v2_appeal_info';
INSERT INTO "x_attendance_v2_appeal_info" ("id","user_id") VALUES ('r6v-x_attendance_v2_appeal_info','person-admin');

DELETE FROM "x_attendance_record" WHERE "id" = 'r6v-x_attendance_record';
INSERT INTO "x_attendance_record" ("id","user_id","check_in_time","status") VALUES ('r6v-x_attendance_record','person-admin','r6seed','r6seed');

DELETE FROM "x_attendance_workplace" WHERE "id" = 'r6v-x_attendance_workplace';
INSERT INTO "x_attendance_workplace" ("id","creator") VALUES ('r6v-x_attendance_workplace','person-admin');

DELETE FROM "x_cal_calendar" WHERE "id" = 'r6v-x_cal_calendar';
INSERT INTO "x_cal_calendar" ("id","name","type","target","color","createor","status") VALUES ('r6v-x_cal_calendar','r6seed','r6seed','r6seed','r6seed','person-admin','r6seed');

DELETE FROM "x_cms_data_document" WHERE "id" = 'r6v-x_cms_data_document';
INSERT INTO "x_cms_data_document" ("id","author_id","creator") VALUES ('r6v-x_cms_data_document','person-admin','person-admin');

DELETE FROM "x_cms_form" WHERE "id" = 'r6v-x_cms_form';
INSERT INTO "x_cms_form" ("id","app_id","name","creator") VALUES ('r6v-x_cms_form','r6seed','r6seed','person-admin');

DELETE FROM "x_cms_appinfo" WHERE "id" = 'r6v-x_cms_appinfo';
INSERT INTO "x_cms_appinfo" ("id","app_type","creator") VALUES ('r6v-x_cms_appinfo','r6seed','person-admin');

DELETE FROM "x_cms_categoryinfo" WHERE "id" = 'r6v-x_cms_categoryinfo';
INSERT INTO "x_cms_categoryinfo" ("id","name","creator") VALUES ('r6v-x_cms_categoryinfo','r6seed','person-admin');

DELETE FROM "x_cms_commend" WHERE "id" = 'r6v-x_cms_commend';
INSERT INTO "x_cms_commend" ("id","doc_id","person_id") VALUES ('r6v-x_cms_commend','r6seed','person-admin');

DELETE FROM "x_cms_comment" WHERE "id" = 'r6v-x_cms_comment';
INSERT INTO "x_cms_comment" ("id","doc_id","person_id") VALUES ('r6v-x_cms_comment','r6seed','person-admin');

DELETE FROM "x_cms_file" WHERE "id" = 'r6v-x_cms_file';
INSERT INTO "x_cms_file" ("id","creator") VALUES ('r6v-x_cms_file','person-admin');

DELETE FROM "x_cms_form_v2" WHERE "id" = 'r6v-x_cms_form_v2';
INSERT INTO "x_cms_form_v2" ("id","app_id","name","creator") VALUES ('r6v-x_cms_form_v2','r6seed','r6seed','person-admin');

DELETE FROM "x_cms_script" WHERE "id" = 'r6v-x_cms_script';
INSERT INTO "x_cms_script" ("id","app_id","name","creator") VALUES ('r6v-x_cms_script','r6seed','r6seed','person-admin');

DELETE FROM "x_cms_article" WHERE "id" = 'r6v-x_cms_article';
INSERT INTO "x_cms_article" ("id","category_id","title","author_id") VALUES ('r6v-x_cms_article','r6seed','r6seed','person-admin');

DELETE FROM "x_cms_content" WHERE "id" = 'r6v-x_cms_content';
INSERT INTO "x_cms_content" ("id","creator") VALUES ('r6v-x_cms_content','person-admin');

DELETE FROM "x_component" WHERE "id" = 'r6v-x_component';
INSERT INTO "x_component" ("id","creator") VALUES ('r6v-x_component','person-admin');

DELETE FROM "x_correlation" WHERE "id" = 'r6v-x_correlation';
INSERT INTO "x_correlation" ("id","creator","person_id","person") VALUES ('r6v-x_correlation','person-admin','person-admin','person-admin');

DELETE FROM "x_file" WHERE "id" = 'r6v-x_file';
INSERT INTO "x_file" ("creator","id") VALUES ('person-admin','r6v-x_file');

DELETE FROM "file_file" WHERE "id" = 'r6v-file_file';
INSERT INTO "file_file" ("id","name","person") VALUES ('r6v-file_file','r6seed','person-admin');

DELETE FROM "x_general_assemble_area" WHERE "id" = 'r6v-x_general_assemble_area';
INSERT INTO "x_general_assemble_area" ("id","creator") VALUES ('r6v-x_general_assemble_area','person-admin');

DELETE FROM "x_general_attend_scope" WHERE "id" = 'r6v-x_general_attend_scope';
INSERT INTO "x_general_attend_scope" ("id","creator") VALUES ('r6v-x_general_attend_scope','person-admin');

DELETE FROM "x_general_assemble_invoice" WHERE "id" = 'r6v-x_general_assemble_invoice';
INSERT INTO "x_general_assemble_invoice" ("id","creator") VALUES ('r6v-x_general_assemble_invoice','person-admin');

DELETE FROM "x_general_assemble_qrcode" WHERE "id" = 'r6v-x_general_assemble_qrcode';
INSERT INTO "x_general_assemble_qrcode" ("id","creator") VALUES ('r6v-x_general_assemble_qrcode','person-admin');

DELETE FROM "x_general_assemble_security_clearance" WHERE "id" = 'r6v-x_general_assemble_security_clearance';
INSERT INTO "x_general_assemble_security_clearance" ("id","creator") VALUES ('r6v-x_general_assemble_security_clearance','person-admin');

DELETE FROM "x_general_file" WHERE "id" = 'r6v-x_general_file';
INSERT INTO "x_general_file" ("id","name","mime_type","creator") VALUES ('r6v-x_general_file','r6seed','r6seed','person-admin');

DELETE FROM "x_general_invoice" WHERE "id" = 'r6v-x_general_invoice';
INSERT INTO "x_general_invoice" ("id","number","date","status","creator") VALUES ('r6v-x_general_invoice','r6seed','r6seed','r6seed','person-admin');

DELETE FROM "x_jpush_device" WHERE "id" = 'r6v-x_jpush_device';
INSERT INTO "x_jpush_device" ("id","user_id","platform","token") VALUES ('r6v-x_jpush_device','person-admin','r6seed','r6seed');

DELETE FROM "x_jpush" WHERE "id" = 'r6v-x_jpush';
INSERT INTO "x_jpush" ("id","creator") VALUES ('r6v-x_jpush','person-admin');

DELETE FROM "x_meeting" WHERE "id" = 'r6v-x_meeting';
INSERT INTO "x_meeting" ("id","title","start_time","end_time","creator") VALUES ('r6v-x_meeting','r6seed',NOW(),NOW(),'person-admin');

DELETE FROM "x_meeting_attachment" WHERE "id" = 'r6v-x_meeting_attachment';
INSERT INTO "x_meeting_attachment" ("id","meeting_id","person") VALUES ('r6v-x_meeting_attachment','r6seed','person-admin');

DELETE FROM "x_message_mass" WHERE "id" = 'r6v-x_message_mass';
INSERT INTO "x_message_mass" ("id","creator") VALUES ('r6v-x_message_mass','person-admin');

DELETE FROM "mind_base_info" WHERE "id" = 'r6v-mind_base_info';
INSERT INTO "mind_base_info" ("id","creator") VALUES ('r6v-mind_base_info','person-admin');

DELETE FROM "x_org_identity" WHERE "id" = 'r6v-x_org_identity';
INSERT INTO "x_org_identity" ("id","name","creator","person_id") VALUES ('r6v-x_org_identity','r6seed','person-admin','person-admin');

DELETE FROM "x_portal" WHERE "id" = 'r6v-x_portal';
INSERT INTO "x_portal" ("id","name","alias","description","portal_category","creator") VALUES ('r6v-x_portal','r6seed','r6seed','r6seed','r6seed','person-admin');

DELETE FROM "x_portal_dict" WHERE "id" = 'r6v-x_portal_dict';
INSERT INTO "x_portal_dict" ("id","creator") VALUES ('r6v-x_portal_dict','person-admin');

DELETE FROM "x_portal_file" WHERE "id" = 'r6v-x_portal_file';
INSERT INTO "x_portal_file" ("id","creator") VALUES ('r6v-x_portal_file','person-admin');

DELETE FROM "x_portal_design" WHERE "id" = 'r6v-x_portal_design';
INSERT INTO "x_portal_design" ("id","creator") VALUES ('r6v-x_portal_design','person-admin');

DELETE FROM "x_portal_script_version" WHERE "id" = 'r6v-x_portal_script_version';
INSERT INTO "x_portal_script_version" ("id","creator") VALUES ('r6v-x_portal_script_version','person-admin');

DELETE FROM "x_portal_template_page" WHERE "id" = 'r6v-x_portal_template_page';
INSERT INTO "x_portal_template_page" ("id","creator") VALUES ('r6v-x_portal_template_page','person-admin');

DELETE FROM "x_portal_surface" WHERE "id" = 'r6v-x_portal_surface';
INSERT INTO "x_portal_surface" ("id","creator") VALUES ('r6v-x_portal_surface','person-admin');

DELETE FROM "x_portal_page" WHERE "id" = 'r6v-x_portal_page';
INSERT INTO "x_portal_page" ("id","portal_id","name","status","creator") VALUES ('r6v-x_portal_page','r6seed','r6seed','r6seed','person-admin');

DELETE FROM "x_process_definition" WHERE "id" = 'r6v-x_process_definition';
INSERT INTO "x_process_definition" ("id","name","creator","person") VALUES ('r6v-x_process_definition','r6seed','person-admin','person-admin');

DELETE FROM "x_process_surface" WHERE "id" = 'r6v-x_process_surface';
INSERT INTO "x_process_surface" ("id","creator") VALUES ('r6v-x_process_surface','person-admin');

DELETE FROM "x_process_work" WHERE "id" = 'r6v-x_process_work';
INSERT INTO "x_process_work" ("id","title","creator_id","status") VALUES ('r6v-x_process_work','r6seed','person-admin','r6seed');

DELETE FROM "x_task" WHERE "id" = 'r6v-x_task';
INSERT INTO "x_task" ("id","work","person","creator") VALUES ('r6v-x_task','r6seed','person-admin','person-admin');

DELETE FROM "x_work" WHERE "id" = 'r6v-x_work';
INSERT INTO "x_work" ("id","title","process","creator","person") VALUES ('r6v-x_work','r6seed','r6seed','person-admin','person-admin');

DELETE FROM "x_attachment" WHERE "id" = 'r6v-x_attachment';
INSERT INTO "x_attachment" ("id","creator") VALUES ('r6v-x_attachment','person-admin');

DELETE FROM "x_workcompleted" WHERE "id" = 'r6v-x_workcompleted';
INSERT INTO "x_workcompleted" ("id","work_id","creator") VALUES ('r6v-x_workcompleted','r6seed','person-admin');

DELETE FROM "x_readcompleted" WHERE "id" = 'r6v-x_readcompleted';
INSERT INTO "x_readcompleted" ("id","work_id","person") VALUES ('r6v-x_readcompleted','r6seed','person-admin');

DELETE FROM "x_record" WHERE "id" = 'r6v-x_record';
INSERT INTO "x_record" ("id","creator","person") VALUES ('r6v-x_record','person-admin','person-admin');

DELETE FROM "x_review" WHERE "id" = 'r6v-x_review';
INSERT INTO "x_review" ("id","work_id","creator","person") VALUES ('r6v-x_review','r6seed','person-admin','person-admin');

DELETE FROM "x_snap" WHERE "id" = 'r6v-x_snap';
INSERT INTO "x_snap" ("id","work_id","snap_type","creator","person") VALUES ('r6v-x_snap','r6seed','r6seed','person-admin','person-admin');

DELETE FROM "x_applications" WHERE "id" = 'r6v-x_applications';
INSERT INTO "x_applications" ("id","creator") VALUES ('r6v-x_applications','person-admin');

DELETE FROM "x_program_collect" WHERE "id" = 'r6v-x_program_collect';
INSERT INTO "x_program_collect" ("id","person_id","creator") VALUES ('r6v-x_program_collect','person-admin','person-admin');

DELETE FROM "x_program_deploy" WHERE "id" = 'r6v-x_program_deploy';
INSERT INTO "x_program_deploy" ("id","creator") VALUES ('r6v-x_program_deploy','person-admin');

DELETE FROM "x_program_dict" WHERE "id" = 'r6v-x_program_dict';
INSERT INTO "x_program_dict" ("id","creator") VALUES ('r6v-x_program_dict','person-admin');

DELETE FROM "x_program_module" WHERE "id" = 'r6v-x_program_module';
INSERT INTO "x_program_module" ("id","creator") VALUES ('r6v-x_program_module','person-admin');

DELETE FROM "x_program_prompt_error_log" WHERE "id" = 'r6v-x_program_prompt_error_log';
INSERT INTO "x_program_prompt_error_log" ("id","creator") VALUES ('r6v-x_program_prompt_error_log','person-admin');

DELETE FROM "x_program_unexpected_error_log" WHERE "id" = 'r6v-x_program_unexpected_error_log';
INSERT INTO "x_program_unexpected_error_log" ("id","creator") VALUES ('r6v-x_program_unexpected_error_log','person-admin');

DELETE FROM "x_query_design" WHERE "id" = 'r6v-x_query_design';
INSERT INTO "x_query_design" ("id","creator") VALUES ('r6v-x_query_design','person-admin');

DELETE FROM "x_query_stat" WHERE "id" = 'r6v-x_query_stat';
INSERT INTO "x_query_stat" ("id","creator") VALUES ('r6v-x_query_stat','person-admin');

DELETE FROM "x_query_table" WHERE "id" = 'r6v-x_query_table';
INSERT INTO "x_query_table" ("id","creator") VALUES ('r6v-x_query_table','person-admin');

DELETE FROM "x_query_view" WHERE "id" = 'r6v-x_query_view';
INSERT INTO "x_query_view" ("id","name","query_type","creator") VALUES ('r6v-x_query_view','r6seed','r6seed','person-admin');

DELETE FROM "x_query_surface" WHERE "id" = 'r6v-x_query_surface';
INSERT INTO "x_query_surface" ("id","creator") VALUES ('r6v-x_query_surface','person-admin');

DELETE FROM "query_view" WHERE "id" = 'r6v-query_view';
INSERT INTO "query_view" ("id","creator_id","creator") VALUES ('r6v-query_view','person-admin','person-admin');

DELETE FROM "file_folder" WHERE "id" = 'r6v-file_folder';
INSERT INTO "file_folder" ("id","name","person") VALUES ('r6v-file_folder','r6seed','person-admin');

DELETE FROM "x_hotpic" WHERE "id" = 'r6v-x_hotpic';
INSERT INTO "x_hotpic" ("id","application","info_id","title","creator") VALUES ('r6v-x_hotpic','r6seed','r6seed','r6seed','person-admin');

DELETE FROM "x_org_definition" WHERE "id" = 'r6v-x_org_definition';
INSERT INTO "x_org_definition" ("id","name","category","type","creator") VALUES ('r6v-x_org_definition','r6seed','r6seed','r6seed','person-admin');

DELETE FROM "x_mind_folder_info" WHERE "id" = 'r6v-x_mind_folder_info';
INSERT INTO "x_mind_folder_info" ("id","name","creator") VALUES ('r6v-x_mind_folder_info','r6seed','person-admin');

DELETE FROM "x_mind_base_info" WHERE "id" = 'r6v-x_mind_base_info';
INSERT INTO "x_mind_base_info" ("id","name","folder_id","creator") VALUES ('r6v-x_mind_base_info','r6seed','r6seed','person-admin');
