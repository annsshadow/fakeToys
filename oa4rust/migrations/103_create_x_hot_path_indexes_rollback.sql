-- 103_create_x_hot_path_indexes_rollback.sql
-- 回滚轮7 的 x_* 经典族热路径索引批次。

DROP INDEX IF EXISTS idx_x_record_work_id_record_type;
DROP INDEX IF EXISTS idx_x_read_person;
DROP INDEX IF EXISTS idx_x_readcompleted_person;
DROP INDEX IF EXISTS idx_x_correlation_person;
DROP INDEX IF EXISTS idx_x_work_process;
DROP INDEX IF EXISTS idx_x_work_creator;
DROP INDEX IF EXISTS idx_x_workcompleted_work_status;
DROP INDEX IF EXISTS idx_x_task_task_status_pending;
DROP INDEX IF EXISTS idx_x_meeting_start_time;
DROP INDEX IF EXISTS idx_x_meeting_status_invited;
DROP INDEX IF EXISTS idx_x_bbs_reply_topic_id;
