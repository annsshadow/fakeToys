-- 029: create all remaining SeaORM core_entity tables that were never migrated.
-- Every *_core_entity (and query_express) crate defines SeaORM entities with a
-- table_name, but only a subset were ever created by earlier migrations.
-- Any endpoint querying a missing table 500s with "internal server error".
-- This migration creates all of them (idempotent) so the entity-backed list/get
-- endpoints return success instead of 500. Column types mirror the entity Model
-- definitions in each crate's src/entities/*.rs.

-- program_center_core_entity ----------
CREATE TABLE IF NOT EXISTS "cte_agent" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "alias" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "validated" BOOLEAN NOT NULL DEFAULT FALSE,
    "enable" BOOLEAN NOT NULL DEFAULT FALSE,
    "cron" TEXT NOT NULL,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "cte_agent_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "cte_invoke" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "alias" TEXT NOT NULL,
    "category" TEXT NOT NULL,
    "validated" BOOLEAN NOT NULL DEFAULT FALSE,
    "creator_person" TEXT NOT NULL,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "cte_invoke_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "cte_structure" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "extension" TEXT,
    "storage" TEXT NOT NULL,
    "length" BIGINT,
    "description" TEXT NOT NULL,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "cte_structure_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_application" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "category" TEXT NOT NULL,
    "sub_category" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "publisher" TEXT NOT NULL,
    "creator_person" TEXT NOT NULL,
    "deleted_at" TEXT,
    CONSTRAINT "x_application_pkey" PRIMARY KEY ("id")
);

-- attendance_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_attendance_record" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "user_id" TEXT NOT NULL,
    "check_in_time" TEXT NOT NULL,
    "check_out_time" TEXT,
    "status" TEXT NOT NULL,
    "create_time" TEXT,
    CONSTRAINT "x_attendance_record_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_attendance_rule" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "start_time" TEXT NOT NULL,
    "end_time" TEXT NOT NULL,
    "create_time" TEXT,
    "update_time" TEXT,
    CONSTRAINT "x_attendance_rule_pkey" PRIMARY KEY ("id")
);

-- calendar_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_cal_calendar" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "type_" TEXT NOT NULL,
    "target" TEXT NOT NULL,
    "color" TEXT NOT NULL,
    "description" TEXT,
    "source" TEXT,
    "createor" TEXT NOT NULL,
    "is_public" BOOLEAN NOT NULL DEFAULT FALSE,
    "status" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_cal_calendar_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_cal_event" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "calendar_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT,
    "location" TEXT,
    "start_time" TIMESTAMP NOT NULL,
    "end_time" TIMESTAMP NOT NULL,
    "all_day" BOOLEAN NOT NULL DEFAULT FALSE,
    "visibility" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "createor" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_cal_event_pkey" PRIMARY KEY ("id")
);

-- correlation_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_corr_c_correlation" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "source_type" TEXT NOT NULL,
    "source_id" TEXT NOT NULL,
    "target_type" TEXT NOT NULL,
    "target_id" TEXT NOT NULL,
    "weight" INTEGER NOT NULL DEFAULT 0,
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_corr_c_correlation_pkey" PRIMARY KEY ("id")
);

-- general_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_gen_ara_district" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "code" TEXT NOT NULL,
    "parent_id" TEXT,
    CONSTRAINT "x_gen_ara_district_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_general_application_dict" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "application" TEXT NOT NULL,
    CONSTRAINT "x_general_application_dict_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_general_application_dict_item" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "dict_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    CONSTRAINT "x_general_application_dict_item_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_general_file" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "mime_type" TEXT NOT NULL,
    "size" BIGINT NOT NULL DEFAULT 0,
    "creator" TEXT,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_general_file_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_general_invoice" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "number" TEXT NOT NULL,
    "date" TEXT NOT NULL,
    "amount" DOUBLE PRECISION NOT NULL DEFAULT 0,
    "status" TEXT NOT NULL,
    "creator" TEXT,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_general_invoice_pkey" PRIMARY KEY ("id")
);

-- hotpic_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_hotpic" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "application" TEXT NOT NULL,
    "info_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "base64" TEXT,
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_hotpic_pkey" PRIMARY KEY ("id")
);

-- jpush_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_jpush_device" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "user_id" TEXT NOT NULL,
    "platform" TEXT NOT NULL,
    "token" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_jpush_device_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_jpush_template" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    CONSTRAINT "x_jpush_template_pkey" PRIMARY KEY ("id")
);

-- mind_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_mind_base_info" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "folder_id" TEXT NOT NULL,
    "description" TEXT,
    "creator" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_mind_base_info_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_mind_folder_info" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "parent_id" TEXT,
    "order_number" INTEGER NOT NULL DEFAULT 0,
    "description" TEXT,
    "creator" TEXT NOT NULL,
    CONSTRAINT "x_mind_folder_info_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_mind_version_info" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "mind_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "folder_id" TEXT NOT NULL,
    "description" TEXT,
    "creator" TEXT NOT NULL,
    "creator_unit" TEXT,
    "file_version" INTEGER NOT NULL DEFAULT 0,
    "shared" BOOLEAN NOT NULL DEFAULT FALSE,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    CONSTRAINT "x_mind_version_info_pkey" PRIMARY KEY ("id")
);

-- message_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_msg_message" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "title" TEXT NOT NULL,
    "body" TEXT,
    "type" TEXT NOT NULL,
    "consumer" TEXT NOT NULL,
    "is_read" BOOLEAN NOT NULL DEFAULT FALSE,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_msg_message_pkey" PRIMARY KEY ("id")
);

-- organization_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_org_bind" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "identity_id" TEXT NOT NULL,
    "group_id" TEXT NOT NULL,
    "role" TEXT,
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_org_bind_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_org_custom" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "identity_id" TEXT NOT NULL,
    "field_name" TEXT NOT NULL,
    "field_value" TEXT NOT NULL,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_org_custom_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_org_definition" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "category" TEXT NOT NULL,
    "type_" TEXT NOT NULL,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_org_definition_pkey" PRIMARY KEY ("id")
);

-- portal_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_portal" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "alias" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "portal_category" TEXT NOT NULL,
    CONSTRAINT "x_portal_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_portal_page" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "portal_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "content" TEXT,
    "status" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_portal_page_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_script" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "alias" TEXT NOT NULL,
    "validated" BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT "x_script_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_widget" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "alias" TEXT NOT NULL,
    "category" TEXT NOT NULL,
    "portal" TEXT NOT NULL,
    CONSTRAINT "x_widget_pkey" PRIMARY KEY ("id")
);

-- processplatform_core_entity ----------
CREATE TABLE IF NOT EXISTS "x_process_task" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "work_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "assignee_id" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_process_task_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_process_ticket" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "work_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "status" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_process_ticket_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_process_work" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "title" TEXT NOT NULL,
    "creator_id" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "form_data" TEXT,
    "create_time" TIMESTAMP,
    "update_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_process_work_pkey" PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "x_process_work_completed" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "work_id" TEXT NOT NULL,
    "result" TEXT NOT NULL,
    "complete_time" TIMESTAMP,
    "deleted_at" TIMESTAMP,
    CONSTRAINT "x_process_work_completed_pkey" PRIMARY KEY ("id")
);

-- query_express ----------
CREATE TABLE IF NOT EXISTS "x_query_view" (
    "id" CHARACTER VARYING(255) NOT NULL,
    "name" TEXT NOT NULL,
    "query_type" TEXT NOT NULL,
    "create_time" TIMESTAMP,
    CONSTRAINT "x_query_view_pkey" PRIMARY KEY ("id")
);
