-- plan002 U2: attendance legacy 族闭合所需表
-- 对齐 x_attendance_core_entity:
--   AttendanceDingtalkDetail / AttendanceQywxDetail / DingdingQywxSyncRecord /
--   StatisticDingding(Qywx)PersonForMonth / StatisticDingding(Qywx)UnitForDay|Month /
--   AttendanceV2AppealInfo / AttendanceV2GroupSchedule / AttendanceV2GroupScheduleConfig /
--   AttendanceV2CheckInRecord
-- Idempotent: safe to run repeatedly.

CREATE TABLE IF NOT EXISTS "x_attendance_dingding_detail" (
    "id" TEXT PRIMARY KEY,
    "user_id" TEXT,
    "time" TEXT,
    "checkin_type" TEXT,
    "location_result" TEXT,
    "source_type" TEXT,
    "group_id" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_dd_detail_user_time"
    ON "x_attendance_dingding_detail" ("user_id", "time");

CREATE TABLE IF NOT EXISTS "x_attendance_qywx_detail" (
    "id" TEXT PRIMARY KEY,
    "user_id" TEXT,
    "time" TEXT,
    "checkin_type" TEXT,
    "location_result" TEXT,
    "source_type" TEXT,
    "group_id" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_qywx_detail_user_time"
    ON "x_attendance_qywx_detail" ("user_id", "time");

CREATE TABLE IF NOT EXISTS "x_attendance_sync_record" (
    "id" TEXT PRIMARY KEY,
    "type" TEXT,
    "status" TEXT,
    "exception_message" TEXT,
    "start_date" TEXT,
    "end_date" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_sync_record_type"
    ON "x_attendance_sync_record" ("type");

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_dd_person_month" (
    "id" TEXT PRIMARY KEY,
    "o2_user" TEXT,
    "o2_unit" TEXT,
    "statistic_year" TEXT,
    "statistic_month" TEXT,
    "work_day_count" BIGINT DEFAULT 0,
    "on_duty_times" BIGINT DEFAULT 0,
    "off_duty_times" BIGINT DEFAULT 0,
    "result_normal" BIGINT DEFAULT 0,
    "late_times" BIGINT DEFAULT 0,
    "serious_late_times" BIGINT DEFAULT 0,
    "leave_early_times" BIGINT DEFAULT 0,
    "absenteeism_times" BIGINT DEFAULT 0,
    "not_signed_count" BIGINT DEFAULT 0,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_dd_pm_user"
    ON "x_attendance_statistic_dd_person_month" ("o2_user", "statistic_year", "statistic_month");
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_dd_pm_unit"
    ON "x_attendance_statistic_dd_person_month" ("o2_unit", "statistic_year", "statistic_month");

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_dd_unit_day" (
    "id" TEXT PRIMARY KEY,
    "o2_unit" TEXT,
    "statistic_year" TEXT,
    "statistic_month" TEXT,
    "statistic_date" TEXT,
    "work_day_count" BIGINT DEFAULT 0,
    "on_duty_times" BIGINT DEFAULT 0,
    "off_duty_times" BIGINT DEFAULT 0,
    "result_normal" BIGINT DEFAULT 0,
    "late_times" BIGINT DEFAULT 0,
    "serious_late_times" BIGINT DEFAULT 0,
    "leave_early_times" BIGINT DEFAULT 0,
    "absenteeism_times" BIGINT DEFAULT 0,
    "not_signed_count" BIGINT DEFAULT 0,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_dd_ud_unit"
    ON "x_attendance_statistic_dd_unit_day" ("o2_unit", "statistic_date");

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_dd_unit_month" (
    "id" TEXT PRIMARY KEY,
    "o2_unit" TEXT,
    "statistic_year" TEXT,
    "statistic_month" TEXT,
    "work_day_count" BIGINT DEFAULT 0,
    "on_duty_times" BIGINT DEFAULT 0,
    "off_duty_times" BIGINT DEFAULT 0,
    "result_normal" BIGINT DEFAULT 0,
    "late_times" BIGINT DEFAULT 0,
    "serious_late_times" BIGINT DEFAULT 0,
    "leave_early_times" BIGINT DEFAULT 0,
    "absenteeism_times" BIGINT DEFAULT 0,
    "not_signed_count" BIGINT DEFAULT 0,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_dd_um_unit"
    ON "x_attendance_statistic_dd_unit_month" ("o2_unit", "statistic_year", "statistic_month");

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_qywx_person_month" (
    "id" TEXT PRIMARY KEY,
    "o2_user" TEXT,
    "o2_unit" TEXT,
    "statistic_year" TEXT,
    "statistic_month" TEXT,
    "work_day_count" BIGINT DEFAULT 0,
    "on_duty_times" BIGINT DEFAULT 0,
    "off_duty_times" BIGINT DEFAULT 0,
    "result_normal" BIGINT DEFAULT 0,
    "late_times" BIGINT DEFAULT 0,
    "serious_late_times" BIGINT DEFAULT 0,
    "leave_early_times" BIGINT DEFAULT 0,
    "absenteeism_times" BIGINT DEFAULT 0,
    "not_signed_count" BIGINT DEFAULT 0,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_qywx_pm_user"
    ON "x_attendance_statistic_qywx_person_month" ("o2_user", "statistic_year", "statistic_month");
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_qywx_pm_unit"
    ON "x_attendance_statistic_qywx_person_month" ("o2_unit", "statistic_year", "statistic_month");

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_qywx_unit_day" (
    "id" TEXT PRIMARY KEY,
    "o2_unit" TEXT,
    "statistic_year" TEXT,
    "statistic_month" TEXT,
    "statistic_date" TEXT,
    "work_day_count" BIGINT DEFAULT 0,
    "on_duty_times" BIGINT DEFAULT 0,
    "off_duty_times" BIGINT DEFAULT 0,
    "result_normal" BIGINT DEFAULT 0,
    "late_times" BIGINT DEFAULT 0,
    "serious_late_times" BIGINT DEFAULT 0,
    "leave_early_times" BIGINT DEFAULT 0,
    "absenteeism_times" BIGINT DEFAULT 0,
    "not_signed_count" BIGINT DEFAULT 0,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_qywx_ud_unit"
    ON "x_attendance_statistic_qywx_unit_day" ("o2_unit", "statistic_date");

CREATE TABLE IF NOT EXISTS "x_attendance_statistic_qywx_unit_month" (
    "id" TEXT PRIMARY KEY,
    "o2_unit" TEXT,
    "statistic_year" TEXT,
    "statistic_month" TEXT,
    "work_day_count" BIGINT DEFAULT 0,
    "on_duty_times" BIGINT DEFAULT 0,
    "off_duty_times" BIGINT DEFAULT 0,
    "result_normal" BIGINT DEFAULT 0,
    "late_times" BIGINT DEFAULT 0,
    "serious_late_times" BIGINT DEFAULT 0,
    "leave_early_times" BIGINT DEFAULT 0,
    "absenteeism_times" BIGINT DEFAULT 0,
    "not_signed_count" BIGINT DEFAULT 0,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_stat_qywx_um_unit"
    ON "x_attendance_statistic_qywx_unit_month" ("o2_unit", "statistic_year", "statistic_month");

CREATE TABLE IF NOT EXISTS "x_attendance_v2_appeal_info" (
    "id" TEXT PRIMARY KEY,
    "record_id" TEXT,
    "user_id" TEXT,
    "record_date_string" TEXT,
    "start_time" TEXT,
    "end_time" TEXT,
    "reason" TEXT,
    "status" INTEGER DEFAULT 0,
    "job_id" TEXT,
    "update_status_admin_person" TEXT,
    "send_status" BOOLEAN DEFAULT FALSE,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_v2_appeal_user"
    ON "x_attendance_v2_appeal_info" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_x_att_v2_appeal_record"
    ON "x_attendance_v2_appeal_info" ("record_id");

CREATE TABLE IF NOT EXISTS "x_attendance_v2_group_schedule" (
    "id" TEXT PRIMARY KEY,
    "group_id" TEXT,
    "user_id" TEXT,
    "schedule_month_string" TEXT,
    "schedule_date_string" TEXT,
    "shift_id" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_v2_sched_group_month"
    ON "x_attendance_v2_group_schedule" ("group_id", "schedule_month_string");

CREATE TABLE IF NOT EXISTS "x_attendance_v2_group_schedule_config" (
    "id" TEXT PRIMARY KEY,
    "group_id" TEXT,
    "config_json" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_v2_checkin_record" (
    "id" TEXT PRIMARY KEY,
    "user_id" TEXT,
    "record_date_string" TEXT,
    "source_type" TEXT,
    "check_in_result" TEXT,
    "check_in_type" TEXT,
    "description" TEXT,
    "record_address" TEXT,
    "longitude" TEXT,
    "latitude" TEXT,
    "group_id" TEXT,
    "shift_id" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);
CREATE INDEX IF NOT EXISTS "idx_x_att_v2_checkin_user_date"
    ON "x_attendance_v2_checkin_record" ("user_id", "record_date_string");
