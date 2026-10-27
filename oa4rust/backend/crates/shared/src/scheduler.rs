// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
        let scheduler = JobScheduler::new()
            .await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;
        Ok(Self {
            scheduler: Arc::new(RwLock::new(scheduler)),
            jobs: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// 启动调度器（阻塞直到被停止）
    pub async fn start(&self) -> SchedulerResult<()> {
        self.scheduler
            .write()
            .await
            .start()
            .await
            .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;
        Ok(())
    }

    /// 关闭调度器
    pub async fn shutdown(&self) -> SchedulerResult<()> {
        self.scheduler
            .write()
            .await
            .shutdown()
            .await
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

        let job = Job::new_async_tz(cron_expr.as_str(), chrono::Utc, move |_uuid, _lock| {
            let h = h.clone();
            let id = job_id_arc.clone();
            Box::pin(async move {
                h();
                debug!(job_id = %id, "one-shot job executed");
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
        })
        .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;

        self.scheduler
            .write()
            .await
            .add(job)
            .await
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

        let job = Job::new_async_tz(cron_expr, chrono::Utc, move |_uuid, _lock| {
            let h = h.clone();
            let id = job_id_arc.clone();
            Box::pin(async move {
                h();
                debug!(job_id = %id, "cron job executed");
            })
        })
        .map_err(|e| SchedulerError::SchedulerError(e.to_string()))?;

        self.scheduler
            .write()
            .await
            .add(job)
            .await
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

    #[tokio::test]
    async fn test_mock_scheduler_schedule_once() {
        let scheduler = MockScheduler::default();
        let executed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let executed_clone = executed.clone();

        scheduler
            .schedule_once(
                "job-1".into(),
                SystemTime::now(),
                Arc::new(move || {
                    executed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                }),
            )
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
            .schedule_cron(
                "cron-1".into(),
                "0 0 * * * *",
                Arc::new(move || {
                    executed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                }),
            )
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

        scheduler
            .schedule_once("dup".into(), SystemTime::now(), handler.clone())
            .await
            .unwrap();
        let result = scheduler
            .schedule_once("dup".into(), SystemTime::now(), handler)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_scheduler_cancel() {
        let scheduler = MockScheduler::default();
        let handler = Arc::new(|| {});

        scheduler
            .schedule_once("to-cancel".into(), SystemTime::now(), handler)
            .await
            .unwrap();
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

#[cfg(test)]
mod tokio_scheduler_tests {
    use super::*;
    use std::time::Duration;

    // ──────────────────────────────────────────────────────────────────────
    // 已有的 `scheduler_tests` 只测了 `MockScheduler`（本文件里自带的测试替身），
    // 也就是说「调度器能不能用」这件事从来没有被测过 —— 测的是替身自己的行为。
    // 这是典型的「测试了 mock 而不是被测对象」：替身永远绿，真实现一行没跑。
    //
    // 下面这组直接驱动 `TokioCronScheduler`。任务注册但不 `start()` 就不会真的执行，
    // 所以用例是确定性的、不依赖时钟。
    // ──────────────────────────────────────────────────────────────────────

    fn noop_handler() -> Arc<dyn Fn() + Send + Sync + 'static> {
        Arc::new(|| {})
    }

    #[tokio::test]
    async fn new_scheduler_lists_no_jobs() {
        let scheduler = TokioCronScheduler::new().await.expect("scheduler");
        assert!(scheduler.list_jobs().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn schedule_cron_registers_the_job_id() {
        let scheduler = TokioCronScheduler::new().await.unwrap();
        scheduler
            .schedule_cron("nightly".into(), "0 0 3 * * *", noop_handler())
            .await
            .unwrap();

        let jobs = scheduler.list_jobs().await.unwrap();
        assert_eq!(jobs, vec!["nightly".to_string()]);
    }

    #[tokio::test]
    async fn schedule_once_registers_a_job() {
        let scheduler = TokioCronScheduler::new().await.unwrap();
        scheduler
            .schedule_once("later".into(), SystemTime::now() + Duration::from_secs(30), noop_handler())
            .await
            .unwrap();

        assert!(scheduler.list_jobs().await.unwrap().contains(&"later".to_string()));
    }

    #[tokio::test]
    async fn schedule_cron_rejects_malformed_expression() {
        // 非法 cron 必须是**可分类的错误**而不是 panic：cron 表达式常来自配置/接口，
        // panic 会把整个进程带走。
        let scheduler = TokioCronScheduler::new().await.unwrap();
        let err = scheduler
            .schedule_cron("bad".into(), "not a cron", noop_handler())
            .await
            .expect_err("malformed cron must be rejected");

        assert!(matches!(err, SchedulerError::SchedulerError(_)), "got {err:?}");
        assert!(scheduler.list_jobs().await.unwrap().is_empty(), "失败的注册不得留下 id");
    }

    #[tokio::test]
    async fn cancel_removes_job_and_then_reports_not_found() {
        let scheduler = TokioCronScheduler::new().await.unwrap();
        scheduler
            .schedule_cron("job".into(), "0 0 * * * *", noop_handler())
            .await
            .unwrap();

        scheduler.cancel(&"job".to_string()).await.unwrap();
        assert!(scheduler.list_jobs().await.unwrap().is_empty());

        // 二次取消必须报 JobNotFound（幂等取消会让「任务是否还在」不可判定）
        let err = scheduler
            .cancel(&"job".to_string())
            .await
            .expect_err("second cancel must fail");
        assert!(matches!(err, SchedulerError::JobNotFound(id) if id == "job"));
    }

    #[tokio::test]
    async fn cancel_unknown_job_reports_job_not_found() {
        let scheduler = TokioCronScheduler::new().await.unwrap();
        let err = scheduler
            .cancel(&"ghost".to_string())
            .await
            .expect_err("unknown job");
        assert!(matches!(err, SchedulerError::JobNotFound(_)));
    }

    #[tokio::test]
    async fn start_and_shutdown_are_both_reachable() {
        // start() 会起后台循环、shutdown() 停掉它。两者都不能因为「没有任务」而失败。
        let scheduler = TokioCronScheduler::new().await.unwrap();
        scheduler.start().await.expect("start");
        scheduler.shutdown().await.expect("shutdown");
    }

    // ─────────────────────────────── systemtime_to_cron（一次性任务的表达式映射）

    /// 解析 `*/N` 里的 N（断言分支与量级，而不是精确到秒 —— 从构造 `at`
    /// 到被测函数读时钟之间必然流逝几百微秒，写死数字会变成偶发红）。
    fn step_of(expr: &str) -> u32 {
        expr.split_whitespace()
            .find_map(|f| f.strip_prefix("*/"))
            .and_then(|n| n.parse().ok())
            .unwrap_or_else(|| panic!("not a step expression: {expr:?}"))
    }

    #[test]
    fn systemtime_to_cron_maps_seconds_minutes_and_fallback() {
        let now = SystemTime::now();

        // 未来 < 60s：秒级步进（6 字段 cron，秒位是 */N）
        let secs_expr = systemtime_to_cron(now + Duration::from_secs(30));
        assert!(
            (28..=30).contains(&step_of(&secs_expr)),
            "expected ~30s step, got {secs_expr:?}"
        );
        assert_eq!(secs_expr.split_whitespace().count(), 6, "必须是 6 字段 cron");

        // 未来 < 60min：分钟级步进（秒位固定 0，分位是 */N）
        let minutes_expr = systemtime_to_cron(now + Duration::from_secs(600));
        assert!(
            (9..=10).contains(&step_of(&minutes_expr)),
            "expected ~10min step, got {minutes_expr:?}"
        );
        assert!(minutes_expr.starts_with("0 "), "分钟级分支秒位必须固定为 0");

        // 过去的时间或 ≥1 小时：退化成「每天 0 点」
        assert_eq!(systemtime_to_cron(now - Duration::from_secs(5)), "0 0 * * * *");
        assert_eq!(systemtime_to_cron(now + Duration::from_secs(7200)), "0 0 * * * *");
    }

    #[test]
    fn systemtime_to_cron_never_emits_zero_step() {
        // `*/0` 是非法表达式。亚秒级未来会被截断成 0 秒，必须钳到 `*/1`，
        // 否则调用方拿到一个解析不了、任务直接丢失的表达式。
        let now = SystemTime::now();
        let expr = systemtime_to_cron(now + Duration::from_millis(200));
        assert_eq!(expr, "*/1 * * * * *");

        // 恰好「现在」有两种合法结果，取决于读时钟的先后：
        //   - 已落到过去 → 兜底 `0 0 * * * *`
        //   - 仍在同一毫秒内 → 截断成 0 秒 → 钳到 `*/1`
        // 两者都合法，**共同约束**是绝不产生 `*/0`。
        let now_expr = systemtime_to_cron(SystemTime::now());
        assert!(
            now_expr == "0 0 * * * *" || now_expr == "*/1 * * * * *",
            "unexpected expression for now: {now_expr:?}"
        );
        assert!(!now_expr.contains("*/0"));
    }

    #[tokio::test]
    async fn systemtime_to_cron_output_is_accepted_by_the_real_scheduler() {
        // 格式有效性不能靠肉眼看：把三种分支的输出都喂给真调度器，
        // 解析失败会直接返回 SchedulerError。
        let scheduler = TokioCronScheduler::new().await.unwrap();
        let now = SystemTime::now();
        for (idx, at) in [
            now,
            now + Duration::from_secs(45),
            now + Duration::from_secs(900),
            now - Duration::from_secs(60),
        ]
        .into_iter()
        .enumerate()
        {
            let expr = systemtime_to_cron(at);
            scheduler
                .schedule_cron(format!("j{idx}"), &expr, noop_handler())
                .await
                .unwrap_or_else(|e| panic!("expr {expr:?} rejected: {e:?}"));
        }
        assert_eq!(scheduler.list_jobs().await.unwrap().len(), 4);
    }
}
