use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info};

use tokio_cron_scheduler::{Job, JobScheduler};

// ──────────────────────────────────────────────────────────────────────────────
// scheduler
//
// 基于 tokio-cron-scheduler 的分布式/集群友好定时调度。
// 支持一次性任务和周期性 cron 任务。
// ──────────────────────────────────────────────────────────────────────────────

/// 调度器错误
#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
    #[error("job not found: {0}")]
    JobNotFound(String),
    #[error("job already exists: {0}")]
    JobAlreadyExists(String),
    #[error("scheduler error: {0}")]
    SchedulerError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
}

pub type SchedulerResult<T> = Result<T, SchedulerError>;

/// 调度器任务 ID
pub type JobId = String;

/// 任务执行结果
#[derive(Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub job_id: JobId,
    pub success: bool,
    pub message: String,
    pub executed_at_ms: u64,
}

/// 调度器 Trait
#[async_trait]
pub trait Scheduler: Send + Sync {
    /// 提交一次性任务（在指定时间执行）
    async fn schedule_once(
        &self,
        job_id: JobId,
        at: SystemTime,
        handler: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> SchedulerResult<()>;

    /// 提交周期性 cron 任务
    /// cron 表达式格式：秒 分 时 日 月 周（6 字段）
    async fn schedule_cron(
        &self,
        job_id: JobId,
        cron_expr: &str,
        handler: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> SchedulerResult<()>;

    /// 取消任务
    async fn cancel(&self, job_id: &JobId) -> SchedulerResult<()>;

    /// 列出所有任务 ID
    async fn list_jobs(&self) -> SchedulerResult<Vec<JobId>>;
}

// ──────────────────────────────────────────────────────────────────────────────
// TokioCronScheduler（基于 tokio-cron-scheduler）
// ──────────────────────────────────────────────────────────────────────────────

/// 基于 tokio-cron-scheduler 的调度器实现
pub struct TokioCronScheduler {
    scheduler: Arc<RwLock<JobScheduler>>,
    jobs: Arc<RwLock<HashMap<JobId, bool>>>,
}

impl TokioCronScheduler {
    /// 创建新调度器实例
    pub async fn new() -> SchedulerResult<Self> {
        let scheduler = JobScheduler::new().await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;
        Ok(Self {
            scheduler: Arc::new(RwLock::new(scheduler)),
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
        )
    }

    /// 启动调度器（阻塞直到被停止）
    pub async fn start(&self) -> SchedulerResult<()> {
        self.scheduler.write().await.start().await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;
        Ok(())
    }

    /// 关闭调度器
    pub async fn shutdown(&self) -> SchedulerResult<()> {
        self.scheduler.write().await.shutdown().await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl Scheduler for TokioCronScheduler {
    async fn schedule_once(
        &self,
        job_id: JobId,
        at: SystemTime,
        handler: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> SchedulerResult<()> {
        let job_id_arc = Arc::new(job_id.clone());
        let cron_expr = systemtime_to_cron(at);
        let h = handler.clone();

        let job = Job::new_async_tz(
            cron_expr.as_str(),
            chrono::Utc,
            move |_uuid, _lock| {
                let h = h.clone();
                let id = job_id_arc.clone();
                Box::pin(async move {
                    h();
                    debug!(job_id = %id, "one-shot job executed");
                }) as std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            },
        )
        .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;

        self.scheduler.write().await.add(job).await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;

        self.jobs.write().await.insert(job_id.clone(), true);
        info!(job_id = %job_id, "scheduled one-shot job");
        Ok(())
    }

    async fn schedule_cron(
        &self,
        job_id: JobId,
        cron_expr: &str,
        handler: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> SchedulerResult<()> {
        let job_id_arc = Arc::new(job_id.clone());
        let h = handler.clone();

        let job = Job::new_async_tz(
            cron_expr,
            chrono::Utc,
            move |_uuid, _lock| {
                let h = h.clone();
                let id = job_id_arc.clone();
                Box::pin(async move {
                    h();
                    debug!(job_id = %id, "cron job executed");
                })
            },
        )
        .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;

        self.scheduler.write().await.add(job).await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;

        self.jobs.write().await.insert(job_id.clone(), true);
        info!(job_id = %job_id, cron = %cron_expr, "scheduled cron job");
        Ok(())
    }

    async fn cancel(&self, job_id: &JobId) -> SchedulerResult<()> {
        let jobs = self.jobs.read().await;
        if jobs.contains_key(job_id) {
            drop(jobs);
            self.jobs.write().await.remove(job_id);
            info!(job_id = %job_id, "cancelled job");
            Ok(())
        } else {
            Err(SchedulerError::JobNotFound(job_id.clone()))
        }
    }

    async fn list_jobs(&self) -> SchedulerResult<Vec<JobId>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.keys().cloned().collect())
    }
}

/// 将 SystemTime 转换为 cron 表达式（近似，仅用于一次性任务）
fn systemtime_to_cron(at: SystemTime) -> String {
    if let Ok(duration) = at.duration_since(SystemTime::now()) {
        let secs = duration.as_secs();
        if secs < 60 {
            return format!("*/{} * * * * *", secs.max(1));
        }
        let minutes = (secs / 60) as u32;
        if minutes < 60 {
            return format!("0 */{} * * * *", minutes.max(1));
        }
    }
    "0 0 * * * *".to_string()
}

// ──────────────────────────────────────────────────────────────────────────────
// 测试辅助：Mock Scheduler
// ──────────────────────────────────────────────────────────────────────────────

/// 内存模拟调度器（测试用）
#[derive(Clone, Default)]
pub struct MockScheduler {
    jobs: Arc<RwLock<HashMap<JobId, bool>>>,
}

#[async_trait]
impl Scheduler for MockScheduler {
    async fn schedule_once(
        &self,
        job_id: JobId,
        _at: SystemTime,
        handler: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> SchedulerResult<()> {
        let mut jobs = self.jobs.write().await;
        if jobs.contains_key(&job_id) {
            return Err(SchedulerError::JobAlreadyExists(job_id));
        }
        jobs.insert(job_id.clone(), true);
        handler();
        info!(job_id = %job_id, "mock scheduled one-shot job");
        Ok(())
    }

    async fn schedule_cron(
        &self,
        job_id: JobId,
        _cron_expr: &str,
        handler: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> SchedulerResult<()> {
        let mut jobs = self.jobs.write().await;
        if jobs.contains_key(&job_id) {
            return Err(SchedulerError::JobAlreadyExists(job_id));
        }
        jobs.insert(job_id.clone(), true);
        handler();
        info!(job_id = %job_id, "mock scheduled cron job");
        Ok(())
    }

    async fn cancel(&self, job_id: &JobId) -> SchedulerResult<()> {
        let mut jobs = self.jobs.write().await;
        if jobs.remove(job_id).is_some() {
            info!(job_id = %job_id, "mock cancelled job");
            Ok(())
        } else {
            Err(SchedulerError::JobNotFound(job_id.clone()))
        }
    }

    async fn list_jobs(&self) -> SchedulerResult<Vec<JobId>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.keys().cloned().collect())
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 单元测试
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod scheduler_tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_mock_scheduler_schedule_once() {
        let scheduler = MockScheduler::default();
        let executed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let executed_clone = executed.clone();

        scheduler
            .schedule_once("job-1".into(), SystemTime::now(), Arc::new(move || {
                executed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            }))
            .await
            .unwrap();

        let jobs = scheduler.list_jobs().await.unwrap();
        assert!(jobs.contains(&"job-1".to_string()));
        assert!(executed.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_mock_scheduler_schedule_cron() {
        let scheduler = MockScheduler::default();
        let executed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let executed_clone = executed.clone();

        scheduler
            .schedule_cron("cron-1".into(), "0 0 * * * *", Arc::new(move || {
                executed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            }))
            .await
            .unwrap();

        let jobs = scheduler.list_jobs().await.unwrap();
        assert!(jobs.contains(&"cron-1".to_string()));
        assert!(executed.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_mock_scheduler_duplicate_job() {
        let scheduler = MockScheduler::default();
        let handler = Arc::new(|| {});

        scheduler.schedule_once("dup".into(), SystemTime::now(), handler.clone()).await.unwrap();
        let result = scheduler.schedule_once("dup".into(), SystemTime::now(), handler).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_scheduler_cancel() {
        let scheduler = MockScheduler::default();
        let handler = Arc::new(|| {});

        scheduler.schedule_once("to-cancel".into(), SystemTime::now(), handler).await.unwrap();
        scheduler.cancel(&"to-cancel".into()).await.unwrap();
        let jobs = scheduler.list_jobs().await.unwrap();
        assert!(!jobs.contains(&"to-cancel".to_string()));
    }

    #[tokio::test]
    async fn test_mock_scheduler_cancel_nonexistent() {
        let scheduler = MockScheduler::default();
        let result = scheduler.cancel(&"nonexistent".into()).await;
        assert!(result.is_err());
    }
}
