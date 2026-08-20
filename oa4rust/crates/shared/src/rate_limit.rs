use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::warn;

use anyhow::Context;
use crate::error::AppError;
use crate::redis::RedisPool;
use redis::AsyncCommands;

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit
//
// 内存滑动窗口频率限制器（可选 Redis 分布式模式）。
// RateLimiter 由 main.rs 构造单一实例注入各 router 与速率限制中间件，
// 统一对认证接口（10 次/分钟/IP）与普通接口（100 次/分钟/IP）限流。
//
// Redis 可用时优先使用分布式锁，保证多实例限流一致性；
// Redis 不可用时降级为内存滑动窗口，不影响既有行为。
// ──────────────────────────────────────────────────────────────────────────────

/// Redis 中 rate limit key 的前缀
const RATE_LIMIT_KEY_PREFIX: &str = "oa4rust:ratelimit:";
/// 内存模式清理间隔（秒）
const CLEANUP_INTERVAL_SECONDS: u64 = 60;

#[derive(Clone)]
pub struct RateLimiter {
    pub attempts: Arc<RwLock<std::collections::HashMap<String, Vec<Instant>>>>,
    /// 可选 Redis 连接池：存在时使用分布式限流（多实例一致性）
    redis_pool: Arc<std::sync::Mutex<Option<RedisPool>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        let limiter = Self {
            attempts: Arc::new(RwLock::new(std::collections::HashMap::new())),
            redis_pool: Arc::new(std::sync::Mutex::new(None)),
        };
        let limiter_clone = limiter.clone();
        tokio::spawn(async move {
            let mut timer = tokio::time::interval(Duration::from_secs(CLEANUP_INTERVAL_SECONDS));
            loop {
                timer.tick().await;
                limiter_clone.cleanup_in_memory().await;
            }
        });
        limiter
    }

    /// 惰性初始化 Redis（同步入口，创建临时 tokio runtime）。
    /// 成功返回 true，失败降级为内存模式。
    pub fn init_redis(&self) -> bool {
        let url = std::env::var("REDIS_URL")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "redis://127.0.0.1:6379".to_string());

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return false,
        };

        let result = rt.block_on(async {
            tokio::time::timeout(
                std::time::Duration::from_secs(2),
                RedisPool::from_url(&url),
            )
            .await
        });

        match result {
            Ok(Ok(pool)) => {
                let mut guard = self.redis_pool.lock().unwrap();
                *guard = Some(pool);
                true
            }
            Ok(Err(e)) => {
                warn!(
                    error = %e,
                    "failed to connect to Redis; rate limiter falling back to in-memory"
                );
                false
            }
            Err(_) => {
                warn!("Redis connection timed out after 2s; rate limiter falling back to in-memory");
                false
            }
        }
    }

    /// 异步惰性初始化 Redis
    pub async fn init_redis_async(&self) -> bool {
        let url = std::env::var("REDIS_URL")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "redis://127.0.0.1:6379".to_string());

        let mut guard = self.redis_pool.lock().unwrap();
        if guard.is_some() {
            return true;
        }
        drop(guard);

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            RedisPool::from_url(&url),
        )
        .await;

        match result {
            Ok(Ok(pool)) => {
                let mut guard = self.redis_pool.lock().unwrap();
                *guard = Some(pool);
                true
            }
            Ok(Err(e)) => {
                warn!(
                    error = %e,
                    "failed to connect to Redis; rate limiter falling back to in-memory"
                );
                false
            }
            Err(_) => {
                warn!("Redis connection timed out after 2s; rate limiter falling back to in-memory");
                false
            }
        }
    }

    /// 获取 Redis pool
    fn get_redis_pool(&self) -> Option<RedisPool> {
        self.redis_pool.lock().unwrap().clone()
    }

    /// 清理过期的滑动窗口条目，防止内存泄漏
    pub async fn cleanup(&self) {
        self.cleanup_in_memory().await;
    }

    /// 内存模式清理
    pub async fn cleanup_in_memory(&self) {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let window = Duration::from_secs(60 * 60);
        let cutoff = now - window;
        attempts.retain(|_, timestamps| {
            timestamps.retain(|&t| t > cutoff);
            !timestamps.is_empty()
        });
    }

    /// 检查是否超出频率限制
    ///
    /// 优先使用 Redis 分布式原子操作；Redis 不可用时降级为内存滑动窗口。
    pub async fn check_rate_limit(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        if let Some(ref redis_pool) = self.get_redis_pool() {
            let redis_key = format!("{}{}", RATE_LIMIT_KEY_PREFIX, key);
            let window_seconds = window_minutes * 60;

            let mut guard = redis_pool.0.manager.lock().await;
            let conn = guard
                .as_mut()
                .context("Redis connection manager not initialized")?;

            let current: u64 = conn.incr(&redis_key, 1).await?;
            if current == 1 {
                conn.expire::<_, ()>(&redis_key, window_seconds).await?;
            }
            if (current as i32) >= max_attempts {
                return Err(AppError::BadRequest(
                    format!(
                        "rate limit exceeded: {} attempts in last {} minutes",
                        current, window_minutes
                    )
                ));
            }
            return Ok(());
        }

        self.check_rate_limit_in_memory(key, max_attempts, window_minutes)
            .await
    }

    /// 内存模式检查限流
    async fn check_rate_limit_in_memory(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let window = Duration::from_secs((window_minutes * 60) as u64);
        let window_start = now - window;

        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.retain(|&t| t > window_start);

        if entry.len() >= max_attempts as usize {
            return Err(AppError::BadRequest(
                format!(
                    "rate limit exceeded: {} attempts in last {} minutes",
                    entry.len(),
                    window_minutes
                )
            ));
        }

        entry.push(now);
        Ok(())
    }

    /// 记录一次失败尝试（递增计数器）
    pub async fn record_failure(&self, key: &str) {
        if let Some(ref redis_pool) = self.get_redis_pool() {
            let redis_key = format!("{}{}", RATE_LIMIT_KEY_PREFIX, key);
            let mut guard = redis_pool.0.manager.lock().await;
            let conn = match guard.as_mut() {
                Some(c) => c,
                None => return,
            };
            let _: u64 = conn.incr(&redis_key, 1).await.ok().unwrap_or(0);
            let _ = conn.expire::<_, ()>(&redis_key, 3600).await;
            return;
        }
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.push(now);
    }

    /// 重置指定 key 的尝试计数（登录成功后调用）
    pub async fn reset(&self, key: &str) {
        if let Some(ref redis_pool) = self.get_redis_pool() {
            let redis_key = format!("{}{}", RATE_LIMIT_KEY_PREFIX, key);
            let mut guard = redis_pool.0.manager.lock().await;
            if let Some(conn) = guard.as_mut() {
                let _ = conn.del::<_, ()>(&redis_key).await;
            }
        }
        self.attempts.write().await.remove(key);
    }
}
