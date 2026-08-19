CREATE TABLE IF NOT EXISTS x_timer_job (
    id VARCHAR(36) PRIMARY KEY,
    work_id VARCHAR(36) NOT NULL REFERENCES x_work(id),
    task_id VARCHAR(36),
    fire_at TIMESTAMP NOT NULL,
    cron TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    kind VARCHAR(20) NOT NULL DEFAULT 'once',
    fired_at TIMESTAMP,
    cancelled_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_x_timer_job_work_id ON x_timer_job(work_id);
CREATE INDEX IF NOT EXISTS idx_x_timer_job_fire_at ON x_timer_job(fire_at);
