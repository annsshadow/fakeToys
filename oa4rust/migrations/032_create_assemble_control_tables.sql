-- Auto-generated: missing raw-SQL tables for *_assemble_control/*_express/*_service_processing handlers.
-- Types derived from handler row.get read types.

CREATE TABLE IF NOT EXISTS "application" (
    "id" TEXT,
    "name" TEXT,
    "application_category" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "bbs_forum_info" (
    "id" TEXT,
    "name" TEXT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "bbs_section_info" (
    "id" TEXT,
    "name" TEXT,
    "forum_id" TEXT,
    "sort" BIGINT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "bbs_subject_info" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "cal_calendar" (
    "id" TEXT,
    "name" TEXT,
    "type" TEXT,
    "is_public" TEXT,
    "status" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "cal_control_config" (
    "config_key" TEXT,
    "config_value" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "cal_event" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "corr_c_correlation" (
    "id" TEXT,
    "from_bundle" TEXT,
    "target_bundle" TEXT,
    "person" TEXT,
    "site" TEXT,
    "order_number" TEXT,
    "cnt" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "cpt_component" (
    "cnt" TEXT,
    "type" TEXT,
    "id" TEXT,
    "name" TEXT,
    "title" TEXT,
    "visible" BOOLEAN,
    "order_number" TEXT,
    "path" TEXT,
    "icon_path" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "district" (
    "id" TEXT,
    "name" TEXT,
    "level" TEXT,
    "parent_id" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "gen_dict" (
    "cnt" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "mind_base_info" (
    "id" TEXT,
    "name" TEXT,
    "folder_id" TEXT,
    "icon" TEXT,
    "description" TEXT,
    "creator" TEXT,
    "creator_unit" TEXT,
    "shared" BOOLEAN,
    "file_version" BIGINT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "mind_folder_info" (
    "id" TEXT,
    "name" TEXT,
    "parent_id" TEXT,
    "order_number" BIGINT,
    "description" TEXT,
    "creator" TEXT,
    "creator_unit" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "mind_version_info" (
    "id" TEXT,
    "mind_id" TEXT,
    "name" TEXT,
    "folder_id" TEXT,
    "description" TEXT,
    "creator" TEXT,
    "creator_unit" TEXT,
    "file_version" BIGINT,
    "shared" BOOLEAN,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "oa_process" (
    "id" TEXT,
    "flag" TEXT,
    "name" TEXT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "oa_workcompleted" (
    "id" TEXT,
    "work_or_work_completed" TEXT,
    "title" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "org_unit" (
    "cnt" TEXT,
    "id" TEXT,
    "name" TEXT,
    "superior" TEXT,
    "level" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pg_class" (
    "cache_count" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_data_record" (
    "xid" TEXT,
    "xjob" TEXT,
    "xdata" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_doc_sign" (
    "xid" TEXT,
    "xtitle" TEXT,
    "xwork" TEXT,
    "xtask" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_documentversion" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_draft" (
    "xid" TEXT,
    "xtitle" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xprocessAlias" TEXT,
    "xactivity" TEXT,
    "xactivityName" TEXT,
    "xactivityType" TEXT,
    "xactivityToken" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_handover" (
    "xid" TEXT,
    "xtitle" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_job" (
    "xid" TEXT,
    "xtitle" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xunit" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xstartTime" TEXT,
    "xcompletedTime" TEXT,
    "xstatus" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_keylock" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_read" (
    "xid" TEXT,
    "xjob" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xread" TEXT,
    "xtitle" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xserial" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xunit" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_readcompleted" (
    "xid" TEXT,
    "xjob" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xcompleted" TEXT,
    "xtitle" TEXT,
    "xstartTime" TEXT,
    "xviewTime" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xserial" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xunit" TEXT,
    "xopinion" TEXT,
    "xopinionLob" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_record" (
    "xid" TEXT,
    "xjob" TEXT,
    "xtitle" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_review" (
    "xid" TEXT,
    "xjob" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xcompleted" TEXT,
    "xtitle" TEXT,
    "xserial" TEXT,
    "xstartTime" TEXT,
    "xcompletedTime" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xperson" TEXT,
    "xactivityUnique" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xopinion" TEXT,
    "xopinionLob" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_serialnumber" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xactivity" TEXT,
    "xactivityName" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_task" (
    "xid" TEXT,
    "xjob" TEXT,
    "xtitle" TEXT,
    "xstartTime" TEXT,
    "xwork" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xactivity" TEXT,
    "xactivityName" TEXT,
    "xactivityType" TEXT,
    "xactivityToken" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xunit" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xexpireTime" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_task_process_mode" (
    "xid" TEXT,
    "xtitle" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_taskcompleted" (
    "xid" TEXT,
    "xjob" TEXT,
    "xtitle" TEXT,
    "xstartTime" TEXT,
    "xcompletedTime" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xserial" TEXT,
    "xperson" TEXT,
    "xactivityUnique" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_work" (
    "xid" TEXT,
    "xjob" TEXT,
    "xtitle" TEXT,
    "xstartTime" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xactivity" TEXT,
    "xactivityType" TEXT,
    "xactivityName" TEXT,
    "xactivityAlias" TEXT,
    "xactivityDescription" TEXT,
    "xactivityToken" TEXT,
    "xactivityUnique" TEXT,
    "xactivityArrivedTime" TEXT,
    "xserial" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_workcompleted" (
    "xid" TEXT,
    "xjob" TEXT,
    "xtitle" TEXT,
    "xstartTime" TEXT,
    "xcompletedTime" TEXT,
    "xcreatorPerson" TEXT,
    "xcreatorIdentity" TEXT,
    "xcreatorUnit" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xapplicationAlias" TEXT,
    "xprocess" TEXT,
    "xprocessName" TEXT,
    "xactivity" TEXT,
    "xactivityType" TEXT,
    "xactivityName" TEXT,
    "xactivityAlias" TEXT,
    "xactivityDescription" TEXT,
    "xactivityToken" TEXT,
    "xserial" TEXT,
    "xform" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_c_worklog" (
    "xid" TEXT,
    "xjob" TEXT,
    "xwork" TEXT,
    "xworkCompleted" TEXT,
    "xtitle" TEXT,
    "xperson" TEXT,
    "xidentity" TEXT,
    "xactivity" TEXT,
    "xactivityName" TEXT,
    "xactivityType" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_application" (
    "xid" TEXT,
    "xname" TEXT,
    "xalias" TEXT,
    "xdescription" TEXT,
    "xapplicationCategory" TEXT,
    "xicon" TEXT,
    "xiconHue" TEXT,
    "xcreatorPerson" TEXT,
    "xlastUpdateTime" TEXT,
    "xlastUpdatePerson" TEXT,
    "xproperties" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_applicationcategory" (
    "xid" TEXT,
    "xname" TEXT,
    "xdescription" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_applicationdict" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_file" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_form" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_formversion" (
    "xid" TEXT,
    "xform" TEXT,
    "xname" TEXT,
    "xcontent" TEXT,
    "xversion" BIGINT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_item_access" (
    "xid" TEXT,
    "xname" TEXT,
    "xprocess" TEXT,
    "xpath" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_mapping" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xsource" TEXT,
    "xtarget" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_mergeitemplan" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_output" (
    "xid" TEXT,
    "xname" TEXT,
    "xprocess" TEXT,
    "xoutput" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_process" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "xalias" TEXT,
    "xdescription" TEXT,
    "xicon" TEXT,
    "xafterBeginScript" TEXT,
    "xafterEndScript" TEXT,
    "xserialTexture" TEXT,
    "xserialActivity" TEXT,
    "xserialPhase" TEXT,
    "xexpireType" TEXT,
    "xexpireDay" TEXT,
    "xexpireHour" TEXT,
    "xexpireWorkTime" TEXT,
    "xworkId" TEXT,
    "xcompletedTime" TEXT,
    "xcreator" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_process_activity" (
    "xid" TEXT,
    "xname" TEXT,
    "xactivityType" TEXT,
    "xdescription" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_process_element" (
    "xid" TEXT,
    "xname" TEXT,
    "xprocessId" TEXT,
    "xelementType" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_processversion" (
    "xid" TEXT,
    "xprocess" TEXT,
    "xname" TEXT,
    "xcontent" TEXT,
    "xversion" BIGINT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_route" (
    "xid" TEXT,
    "xname" TEXT,
    "xprocess" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_script" (
    "xid" TEXT,
    "xname" TEXT,
    "xapplication" TEXT,
    "xapplicationName" TEXT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_scriptversion" (
    "xid" TEXT,
    "xscript" TEXT,
    "xname" TEXT,
    "xcode" TEXT,
    "xversion" BIGINT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_templateform" (
    "xid" TEXT,
    "xname" TEXT,
    "xcategory" TEXT,
    "xcontent" TEXT,
    "xcreatorPerson" TEXT,
    "xcreateTime" TEXT,
    "xupdateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "pp_e_workcompleted" (
    "xid" TEXT,
    "xworkId" TEXT,
    "xcompletedTime" TEXT,
    "xcreator" TEXT,
    "xcreateTime" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "process_application" (
    "id" TEXT,
    "name" TEXT,
    "description" TEXT,
    "form_definition" TEXT,
    "status" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "query_import" (
    "id" TEXT,
    "view_id" TEXT,
    "file_name" TEXT,
    "status" TEXT,
    "import_time" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "query_item" (
    "id" TEXT,
    "view_id" TEXT,
    "name" TEXT,
    "field_name" TEXT,
    "data_type" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "query_view" (
    "id" TEXT,
    "name" TEXT,
    "description" TEXT,
    "query_sql" TEXT,
    "creator_id" TEXT,
    "status" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "route" (
    "id" TEXT,
    "name" TEXT,
    "process_id" TEXT,
    "type" TEXT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "set" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "sub" (
    "id" TEXT,
    "parent_id" TEXT,
    "level" TEXT,
    "sort" TEXT,
    "name" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "unit_id" TEXT,
    "type" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "sup" (
    "id" TEXT,
    "parent_id" TEXT,
    "level" TEXT,
    "sort" TEXT,
    "name" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "unit_id" TEXT,
    "type" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_ai_file" (
    "id" TEXT,
    "name" TEXT,
    "file_name" TEXT,
    "file_size" BIGINT,
    "file_type" TEXT,
    "enabled" BOOLEAN,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_ai_index" (
    "id" TEXT,
    "doc_id" TEXT,
    "app_id" TEXT,
    "title" TEXT,
    "enabled" BOOLEAN,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_ai_mcp_config" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_ai_model_config" (
    "id" TEXT,
    "name" TEXT,
    "url" TEXT,
    "enabled" BOOLEAN,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_applications" (
    "id" TEXT,
    "name" TEXT,
    "app_id" TEXT,
    "disable" BOOLEAN,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_admin" (
    "id" TEXT,
    "unit_name" TEXT,
    "unit_ou" TEXT,
    "admin_name" TEXT,
    "admin" TEXT,
    "admin_level" TEXT,
    "person_id" TEXT,
    "unit_id" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_appeal_info" (
    "id" TEXT,
    "person_id" TEXT,
    "appeal_status" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_assemble_control_rule" (
    "id" TEXT,
    "rule_name" TEXT,
    "rule_type" TEXT,
    "enabled" BOOLEAN,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_config" (
    "id" TEXT,
    "name" TEXT,
    "value" TEXT,
    "category" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_detail" (
    "id" TEXT,
    "person_id" TEXT,
    "date" TEXT,
    "status" TEXT,
    "creator" TEXT,
    "unit_id" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_employee_config" (
    "id" TEXT,
    "top_unit_name" TEXT,
    "top_unit_ou" TEXT,
    "unit_name" TEXT,
    "unit_ou" TEXT,
    "employee_name" TEXT,
    "employee_number" TEXT,
    "config_type" TEXT,
    "emp_in_top_unit_time" TEXT,
    "person_id" TEXT,
    "config_data" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_import_file_info" (
    "id" TEXT,
    "file_name" TEXT,
    "file_size" BIGINT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_schedule_setting" (
    "id" TEXT,
    "name" TEXT,
    "setting_data" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "unit_id" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_selfholiday" (
    "id" TEXT,
    "person_id" TEXT,
    "holiday_date" TEXT,
    "reason" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_setting" (
    "id" TEXT,
    "code" TEXT,
    "name" TEXT,
    "value" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_statistic" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_require_log" (
    "id" TEXT,
    "require_type" TEXT,
    "status" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_statistical_cycle" (
    "id" TEXT,
    "top_unit_name" TEXT,
    "unit_name" TEXT,
    "cycle_year" TEXT,
    "cycle_month" TEXT,
    "cycle_start_date_string" TEXT,
    "cycle_end_date_string" TEXT,
    "description" TEXT,
    "year" TEXT,
    "month" TEXT,
    "cycle_status" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_statisticshow" (
    "id" TEXT,
    "person_id" TEXT,
    "year" TEXT,
    "month" TEXT,
    "status" TEXT,
    "unit_id" TEXT,
    "work_date" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_workday_config" (
    "id" TEXT,
    "work_date" TEXT,
    "is_workday" BOOLEAN,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_workplace" (
    "id" TEXT,
    "name" TEXT,
    "address" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_bbs_shutup" (
    "id" TEXT,
    "person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_cms_assemble_control_config" (
    "enabled" BOOLEAN,
    "max_category_count" BIGINT,
    "allow_anonymous" BOOLEAN,
    "id" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_cms_assemble_control_section" (
    "id" TEXT,
    "name" TEXT,
    "enabled" BOOLEAN,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_cms_content" (
    "id" TEXT,
    "title" TEXT,
    "category_id" TEXT,
    "status" TEXT,
    "content" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_cms_control_config" (
    "enabled" BOOLEAN,
    "max_category_count" BIGINT,
    "allow_anonymous" BOOLEAN,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_cms_control_section" (
    "id" TEXT,
    "name" TEXT,
    "enabled" BOOLEAN,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_cms_templateform" (
    "xid" TEXT,
    "xname" TEXT,
    "xcategory" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_component" (
    "id" TEXT,
    "name" TEXT,
    "title" TEXT,
    "type" TEXT,
    "visible" BOOLEAN,
    "order_number" TEXT,
    "path" TEXT,
    "icon_path" TEXT,
    "cnt" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "total" TEXT,
    "active" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_correlation" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_file" (
    "creator" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_file_assemble_control_category" (
    "id" TEXT,
    "name" TEXT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_file_assemble_control_config" (
    "enabled" TEXT,
    "default_storage" TEXT,
    "max_upload_size" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_file_assemble_control_storage_pool" (
    "id" TEXT,
    "name" TEXT,
    "enabled" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_area" (
    "id" TEXT,
    "name" TEXT,
    "parent_id" TEXT,
    "level" TEXT,
    "province" TEXT,
    "city" TEXT,
    "district" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_control_config" (
    "id" TEXT,
    "system_name" TEXT,
    "maintenance_mode" TEXT,
    "allow_registration" TEXT,
    "version" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_control_permission" (
    "id" TEXT,
    "module_name" TEXT,
    "user_id" TEXT,
    "can_view" BOOLEAN,
    "can_edit" BOOLEAN,
    "can_delete" BOOLEAN,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_ecnet_config" (
    "id" TEXT,
    "name" TEXT,
    "value" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_excel" (
    "id" TEXT,
    "sheet_name" TEXT,
    "excel_id" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_excel_sheet" (
    "id" TEXT,
    "sheet_name" TEXT,
    "excel_id" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_invoice" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "status" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "cnt" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_office" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_qrcode" (
    "id" TEXT,
    "width" BIGINT,
    "height" BIGINT,
    "text" TEXT,
    "content" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_security_clearance" (
    "id" TEXT,
    "name" TEXT,
    "type" TEXT,
    "enabled" BOOLEAN,
    "subject" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "object" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_upgrade" (
    "id" TEXT,
    "version" TEXT,
    "description" TEXT,
    "file_url" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_assemble_worktime" (
    "cnt" TEXT,
    "total" TEXT,
    "id" TEXT,
    "date" TEXT,
    "is_holiday" BOOLEAN,
    "is_workday" BOOLEAN,
    "is_worktime" BOOLEAN,
    "minutes" BIGINT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_general_attend_scope" (
    "id" TEXT,
    "name" TEXT,
    "unit_id" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_jpush" (
    "id" TEXT,
    "title" TEXT,
    "content" TEXT,
    "target" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "cnt" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_meeting_assemble_control" (
    "id" TEXT,
    "meeting_id" TEXT,
    "control_type" TEXT,
    "enabled" BOOLEAN,
    "config" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_meeting_checkin" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_meeting_config" (
    "id" TEXT,
    "config_key" TEXT,
    "config_value" TEXT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message" (
    "id" TEXT,
    "conversation_id" TEXT,
    "content" TEXT,
    "sender" TEXT,
    "type" TEXT,
    "create_time" TEXT,
    "mass_id" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_collection" (
    "id" TEXT,
    "message_id" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_consume" (
    "id" TEXT,
    "consume" TEXT,
    "content" TEXT,
    "sender" TEXT,
    "create_time" TEXT,
    "read_status" TEXT,
    "type" TEXT,
    "consume_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_conversation" (
    "id" TEXT,
    "name" TEXT,
    "type" TEXT,
    "last_message" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_conversation_member" (
    "id" TEXT,
    "conversation_id" TEXT,
    "person_id" TEXT,
    "role" TEXT,
    "join_time" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_file" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_instant" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_message_mass" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_mind" (
    "id" TEXT,
    "name" TEXT,
    "content" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_mind_assemble_control_config" (
    "id" TEXT,
    "config_data" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_org_config" (
    "config_value" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_design" (
    "id" TEXT,
    "name" TEXT,
    "description" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "category" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_dict" (
    "id" TEXT,
    "name" TEXT,
    "app_name" TEXT,
    "app_data" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "key_name" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_file" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "file_type" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_input" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_layout" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_output" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "app_name" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_page_version" (
    "id" TEXT,
    "page_id" TEXT,
    "version" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_script" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "category" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_script_version" (
    "id" TEXT,
    "script_id" TEXT,
    "version" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_surface" (
    "id" TEXT,
    "published" BOOLEAN,
    "published_at" TEXT,
    "portal_id" TEXT,
    "name" TEXT,
    "preview_url" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_template_page" (
    "id" TEXT,
    "name" TEXT,
    "category" TEXT,
    "content" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_portal_widget" (
    "id" TEXT,
    "name" TEXT,
    "portal_id" TEXT,
    "category" TEXT,
    "config" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_process_surface" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_agent" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_callback_registration" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_collect" (
    "id" TEXT,
    "person_id" TEXT,
    "title" TEXT,
    "url" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_config" (
    "id" TEXT,
    "key" TEXT,
    "value" TEXT,
    "category" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_deploy" (
    "id" TEXT,
    "name" TEXT,
    "version" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_deploy_resource" (
    "id" TEXT,
    "resource_name" TEXT,
    "resource_type" TEXT,
    "path" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_design" (
    "id" TEXT,
    "name" TEXT,
    "category" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_dict" (
    "id" TEXT,
    "name" TEXT,
    "key_name" TEXT,
    "app_name" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "app_data" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_field" (
    "id" TEXT,
    "entity" TEXT,
    "field_name" TEXT,
    "field_label" TEXT,
    "field_type" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_message_log" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_module" (
    "id" TEXT,
    "name" TEXT,
    "entity" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_mpweixin_menu" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_output" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "app_name" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_prompt_error_log" (
    "exception_class" TEXT,
    "cnt" TEXT,
    "logger_name" TEXT,
    "id" TEXT,
    "message" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_schedule" (
    "id" TEXT,
    "name" TEXT,
    "cron_expression" TEXT,
    "status" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "server_node" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_schedule_log" (
    "id" TEXT,
    "schedule_id" TEXT,
    "application" TEXT,
    "status" TEXT,
    "message" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_script" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "category" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_sync_log" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_table" (
    "id" TEXT,
    "table_name" TEXT,
    "entity" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_program_unexpected_error_log" (
    "id" TEXT,
    "error_type" TEXT,
    "message" TEXT,
    "stack_trace" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query" (
    "id" TEXT,
    "name" TEXT,
    "query_type" TEXT,
    "count" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_design" (
    "id" TEXT,
    "name" TEXT,
    "category" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "cnt" TEXT,
    "creator" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_entity_property" (
    "field_name" TEXT,
    "field_label" TEXT,
    "field_type" TEXT,
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_import_model" (
    "id" TEXT,
    "name" TEXT,
    "model_flag" TEXT,
    "query_flag" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_import_model_record" (
    "id" TEXT,
    "model_flag" TEXT,
    "data" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_import_record" (
    "id" TEXT,
    "name" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_input" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_neural_calculate" (
    "id" TEXT,
    "model_flag" TEXT,
    "work_id" TEXT,
    "result" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_neural_model" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "status" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_output" (
    "id" TEXT,
    "name" TEXT,
    "flag" TEXT,
    "app_name" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_stat" (
    "id" TEXT,
    "name" TEXT,
    "query_flag" TEXT,
    "stat_type" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_surface" (
    "id" TEXT PRIMARY KEY,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_table" (
    "id" TEXT,
    "name" TEXT,
    "table_flag" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "query_flag" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_table_data" (
    "id" TEXT,
    "table_flag" TEXT,
    "data" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator" TEXT,
    "creator_person" TEXT,
    "update_person" TEXT
);

CREATE TABLE IF NOT EXISTS "x_query_view_excel" (
    "id" TEXT,
    "view_flag" TEXT,
    "excel_data" TEXT,
    "creator" TEXT,
    "create_time" TEXT,
    "update_time" TEXT,
    "sequence" TEXT,
    "order_number" BIGINT,
    "creator_person" TEXT,
    "update_person" TEXT
);

