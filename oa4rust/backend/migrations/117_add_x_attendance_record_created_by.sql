-- 117_add_x_attendance_record_created_by.sql
-- P0（2026-11-06）：考勤记录可被任意登录用户改删。
-- record_create 的 user_id 取自请求体，管理员代建需要留痕「代建人」。
-- 该列记录**实际写入人**（会话人）；本人自建时等于 user_id，代建时与 user_id 不同。
-- 允许为空：存量数据没有该信息，不伪造回填。
ALTER TABLE "x_attendance_record"
    ADD COLUMN IF NOT EXISTS "created_by" TEXT;
