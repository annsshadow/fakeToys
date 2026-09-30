-- 103_create_x_hot_path_indexes.sql
-- x_* 经典族热路径索引（轮7 证据化扫描延续轮6 方法）：每条索引对应 handler 中
-- 至少一条真实 SQL 谓词组合；已覆盖列（x_task.work、x_record.work_id 单列、
-- x_work.work_status/application 单列）与低选择性布尔列（is_recommend）不重复建。

-- 流程记录探针：WHERE work_id = ? AND record_type = ? ORDER BY create_time DESC
CREATE INDEX IF NOT EXISTS idx_x_record_work_id_record_type ON x_record (work_id, record_type);

-- 待阅/已阅人员面：WHERE person = ?
CREATE INDEX IF NOT EXISTS idx_x_read_person ON x_read (person);
CREATE INDEX IF NOT EXISTS idx_x_readcompleted_person ON x_readcompleted (person);

-- 关联/协作面：WHERE person = ?
CREATE INDEX IF NOT EXISTS idx_x_correlation_person ON x_correlation (person);

-- 工作流程组合谓词：WHERE process/creator 与 work_status 组合
CREATE INDEX IF NOT EXISTS idx_x_work_process ON x_work (process);
CREATE INDEX IF NOT EXISTS idx_x_work_creator ON x_work (creator);
CREATE INDEX IF NOT EXISTS idx_x_workcompleted_work_status ON x_workcompleted (work_status);

-- 待办池扫描：WHERE task_status = 'pending' AND deleted_at IS NULL
CREATE INDEX IF NOT EXISTS idx_x_task_task_status_pending ON x_task (task_status) WHERE deleted_at IS NULL;

-- 会议面：start_time 排序/过滤 与 status+invited 组合
CREATE INDEX IF NOT EXISTS idx_x_meeting_start_time ON x_meeting (start_time);
CREATE INDEX IF NOT EXISTS idx_x_meeting_status_invited ON x_meeting (status, invited);

-- BBS 主题回复：WHERE topic_id = ?
CREATE INDEX IF NOT EXISTS idx_x_bbs_reply_topic_id ON x_bbs_reply (topic_id);
