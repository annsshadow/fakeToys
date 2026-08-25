use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::warn;

use crate::error::AppError;
use crate::middleware::rate_limit_distributed::{
    distributed_rate_key, select_distributed_policy, window_decision, RedisWindowCounter, WindowCounter,
};
use crate::redis::RedisPool;
use redis::AsyncCommands;

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit
//
// 内存滑动窗口频率限制器（可选 Redis 分布式模式）。
// RateLimiter 由 main.rs 构造单一实例注入各 router 与速率限制中间件，
// 统一对认证接口（10 次/分钟/IP）与普通接口（100 次/分钟/IP）限流。
//
// U7c 门控策略：仅当显式配置 REDIS_URL 时使用 Redis INCR+EXPIRE 分布式
// 滑动窗口（key = rate:{client_ip}:{window_secs}）替代进程内存计数；
// Redis 不可达（未配置 / 初始化失败 / 运行期操作失败）时降级回内存限流
// 并 warn。中间件签名保持不变，策略切换仅发生在 check_rate_limit 内部。
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
    /// 测试注入点：覆盖分布式计数器（模拟 Redis / 故障），生产路径为 None
    window_counter_override: Arc<std::sync::Mutex<Option<Arc<dyn WindowCounter>>>>,
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
            window_counter_override: Arc::new(std::sync::Mutex::new(None)),
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

    /// 测试注入点：覆盖分布式计数器（模拟 Redis / 故障场景）
    #[doc(hidden)]
    pub fn set_window_counter_override_for_test(&self, counter: Option<Arc<dyn WindowCounter>>) {
        *self.window_counter_override.lock().unwrap() = counter;
    }

    /// 解析当前生效的分布式窗口计数器。
    ///
    /// 门控（U7c）：生产路径仅当 REDIS_URL 已配置时才启用分布式限流；
    /// 测试显式注入的 override 视为有意模拟，不受环境门控约束。
    async fn active_window_counter(&self) -> Option<Arc<dyn WindowCounter>> {
        if let Some(override_counter) = self.window_counter_override.lock().unwrap().clone() {
            return Some(override_counter);
        }
        let derived = self
            .get_redis_pool()
            .map(|p| Arc::new(RedisWindowCounter::new(p)) as Arc<dyn WindowCounter>);
        select_distributed_policy(crate::redis::redis_url_from_env().is_some(), derived)
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
    /// U7c 策略：REDIS_URL 存在且 Redis 可达时使用分布式滑动窗口
    /// （INCR+EXPIRE，key = rate:{client_ip}:{window_secs}）；
    /// Redis 运行期不可达时降级为内存滑动窗口并 warn。
    pub async fn check_rate_limit(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        let window_secs = window_minutes * 60;

        if let Some(counter) = self.active_window_counter().await {
            let redis_key = distributed_rate_key(key, window_secs);
            match counter.incr_window(&redis_key, window_secs).await {
                Ok(current) => return window_decision(current, max_attempts, window_minutes),
                Err(e) => {
                    warn!(
                        error = %e,
                        key = %key,
                        "Redis rate limiter unavailable; falling back to in-memory sliding window"
                    );
                    // fall through：降级到进程内存限流
                }
            }
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
