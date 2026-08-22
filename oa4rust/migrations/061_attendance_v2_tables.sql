-- plan002 U2: attendance v2 tables (AttendanceV2Group / AttendanceV2Shift /
-- AttendanceV2LeaveData from x_attendance_core_entity v2 package).
-- Idempotent: safe to run repeatedly.

CREATE TABLE IF NOT EXISTS "x_attendance_v2_group" (
    "id" TEXT PRIMARY KEY,
    "group_name" TEXT,
    "check_type" TEXT,
    "top_unit" TEXT,
    "unit_list" TEXT,
    "shift_id" TEXT,
    "status" INTEGER DEFAULT 1,
    "assist_admin_list" TEXT,
    "participate_list" TEXT,
    "work_place_id_list" TEXT,
    "work_date_list" TEXT,
    "start_date" TEXT,
    "end_date" TEXT,
    "operator" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_v2_shift" (
    "id" TEXT PRIMARY KEY,
    "shift_name" TEXT,
    "on_duty_time" TEXT,
    "off_duty_time" TEXT,
    "work_time" INTEGER,
    "serial_no" BIGINT DEFAULT 0,
    "properties_json" TEXT,
    "operator" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);

CREATE TABLE IF NOT EXISTS "x_attendance_v2_leave" (
    "id" TEXT PRIMARY KEY,
    "person" TEXT,
    "leave_type" TEXT,
    "start_time" TEXT,
    "end_time" TEXT,
    "description" TEXT,
    "job_id" TEXT,
    "batch_flag" TEXT,
    "creator_person" TEXT,
    "create_time" TEXT,
    "update_time" TEXT
);

CREATE INDEX IF NOT EXISTS "idx_x_attendance_v2_leave_batch_flag"
    ON "x_attendance_v2_leave" ("batch_flag");
