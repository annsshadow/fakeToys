-- 102_create_pp_c_hot_path_indexes.sql
-- 流程平台 pp_c_* 双轨表族热路径索引：09-27 的 xid 批次只覆盖了 xid 主键查找，
-- 而 待办/已办/待阅/已阅 计数与分页、job/work/worklog/record 等查询全部走
-- xperson / xwork / xjob / xprocess / xapplication 谓词，此前均为顺序扫描。
-- 本迁移每条索引都对应 handler 中至少一条真实 SQL（轮6 证据化扫描，
-- 无投机索引：无 handler 查询证据的列一律不建）。

CREATE INDEX IF NOT EXISTS idx_pp_c_task_xperson ON pp_c_task (xperson);
CREATE INDEX IF NOT EXISTS idx_pp_c_task_xwork ON pp_c_task (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_task_xjob ON pp_c_task (xjob);
CREATE INDEX IF NOT EXISTS idx_pp_c_task_xprocess ON pp_c_task (xprocess);
CREATE INDEX IF NOT EXISTS idx_pp_c_task_xapplication ON pp_c_task (xapplication);

CREATE INDEX IF NOT EXISTS idx_pp_c_taskcompleted_xperson ON pp_c_taskcompleted (xperson);
CREATE INDEX IF NOT EXISTS idx_pp_c_taskcompleted_xwork ON pp_c_taskcompleted (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_taskcompleted_xjob ON pp_c_taskcompleted (xjob);
CREATE INDEX IF NOT EXISTS idx_pp_c_taskcompleted_xprocess ON pp_c_taskcompleted (xprocess);
CREATE INDEX IF NOT EXISTS idx_pp_c_taskcompleted_xapplication ON pp_c_taskcompleted (xapplication);

CREATE INDEX IF NOT EXISTS idx_pp_c_read_xperson ON pp_c_read (xperson);
CREATE INDEX IF NOT EXISTS idx_pp_c_read_xwork ON pp_c_read (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_read_xjob ON pp_c_read (xjob);
CREATE INDEX IF NOT EXISTS idx_pp_c_read_xprocess ON pp_c_read (xprocess);
CREATE INDEX IF NOT EXISTS idx_pp_c_read_xapplication ON pp_c_read (xapplication);

CREATE INDEX IF NOT EXISTS idx_pp_c_readcompleted_xperson ON pp_c_readcompleted (xperson);
CREATE INDEX IF NOT EXISTS idx_pp_c_readcompleted_xwork ON pp_c_readcompleted (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_readcompleted_xjob ON pp_c_readcompleted (xjob);
CREATE INDEX IF NOT EXISTS idx_pp_c_readcompleted_xprocess ON pp_c_readcompleted (xprocess);
CREATE INDEX IF NOT EXISTS idx_pp_c_readcompleted_xapplication ON pp_c_readcompleted (xapplication);

CREATE INDEX IF NOT EXISTS idx_pp_c_review_xperson ON pp_c_review (xperson);
CREATE INDEX IF NOT EXISTS idx_pp_c_review_xjob ON pp_c_review (xjob);

CREATE INDEX IF NOT EXISTS idx_pp_c_work_xprocess ON pp_c_work (xprocess);
CREATE INDEX IF NOT EXISTS idx_pp_c_work_xapplication ON pp_c_work (xapplication);

CREATE INDEX IF NOT EXISTS idx_pp_c_workcompleted_xapplication ON pp_c_workcompleted (xapplication);

CREATE INDEX IF NOT EXISTS idx_pp_c_job_xjob ON pp_c_job (xjob);
CREATE INDEX IF NOT EXISTS idx_pp_c_job_xperson ON pp_c_job (xperson);

CREATE INDEX IF NOT EXISTS idx_pp_c_keylock_xwork ON pp_c_keylock (xwork);

CREATE INDEX IF NOT EXISTS idx_pp_c_record_xwork ON pp_c_record (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_record_xjob ON pp_c_record (xjob);

CREATE INDEX IF NOT EXISTS idx_pp_c_data_record_xwork ON pp_c_data_record (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_data_record_xjob ON pp_c_data_record (xjob);

CREATE INDEX IF NOT EXISTS idx_pp_c_worklog_xwork ON pp_c_worklog (xwork);

CREATE INDEX IF NOT EXISTS idx_pp_c_documentversion_xwork ON pp_c_documentversion (xwork);
CREATE INDEX IF NOT EXISTS idx_pp_c_documentversion_xjob ON pp_c_documentversion (xjob);

CREATE INDEX IF NOT EXISTS idx_pp_c_doc_sign_xjob ON pp_c_doc_sign (xjob);

CREATE INDEX IF NOT EXISTS idx_pp_c_attachment_xjob ON pp_c_attachment (xjob);

CREATE INDEX IF NOT EXISTS idx_pp_c_serialnumber_xapplication ON pp_c_serialnumber (xapplication);
