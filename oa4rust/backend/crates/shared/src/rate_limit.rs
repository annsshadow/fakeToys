// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::warn;

use crate::error::AppError;
use crate::middleware::rate_limit_distributed::{
    distributed_rate_key, select_distributed_policy, window_decision, RedisWindowCounter,
    WindowCounter,
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

        // 已在 tokio 运行时里时不能再建新运行时：`Runtime::new()` 会直接
        // panic（"Cannot start a runtime from within a runtime"）。
        // SessionManager::init_redis 一直有这道守卫，这里补齐 ——
        // 两个 init_redis 是同一条启动链上的兄弟调用，一个能优雅降级、
        // 另一个直接 panic，属于实现漂移。
        // 运行时内的调用方应改用 `init_redis_async()`。
        if tokio::runtime::Handle::try_current().is_ok() {
            return false;
        }

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return false,
        };

        let result = rt.block_on(async {
            tokio::time::timeout(std::time::Duration::from_secs(2), RedisPool::from_url(&url)).await
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
                warn!(
                    "Redis connection timed out after 2s; rate limiter falling back to in-memory"
                );
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

        if self.redis_pool.lock().unwrap().is_some() {
            return true;
        }

        let result =
            tokio::time::timeout(std::time::Duration::from_secs(2), RedisPool::from_url(&url))
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
                warn!(
                    "Redis connection timed out after 2s; rate limiter falling back to in-memory"
                );
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
    pub async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: i32,
        window_minutes: i64,
    ) -> Result<(), AppError> {
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
    async fn check_rate_limit_in_memory(
        &self,
        key: &str,
        max_attempts: i32,
        window_minutes: i64,
    ) -> Result<(), AppError> {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let window = Duration::from_secs((window_minutes * 60) as u64);
        let window_start = now - window;

        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.retain(|&t| t > window_start);

        if entry.len() >= max_attempts as usize {
            return Err(AppError::BadRequest(format!(
                "rate limit exceeded: {} attempts in last {} minutes",
                entry.len(),
                window_minutes
            )));
        }

        entry.push(now);
        Ok(())
    }

    /// 记录一次失败尝试（递增计数器）
    pub async fn record_failure(&self, key: &str) {
        if let Some(ref redis_pool) = self.get_redis_pool() {
            let redis_key = format!("{}{}", RATE_LIMIT_KEY_PREFIX, key);
            let mut conn = match redis_pool
                .0
                .manager
                .lock()
                .await
                .as_ref()
                .map(|m| m.clone())
            {
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
            if let Some(mut conn) = redis_pool
                .0
                .manager
                .lock()
                .await
                .as_ref()
                .map(|m| m.clone())
            {
                let _ = conn.del::<_, ()>(&redis_key).await;
            }
        }
        self.attempts.write().await.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ──────────────────────────────────────────────────────────────────────
    // 这一组补的是**内存滑动窗口自身的簿记**，不是限流决策。
    //
    // `middleware/rate_limit_distributed.rs` 已经覆盖了「决策」与「Redis 降级」，
    // 但那个文件用的是注入计数器，`attempts` 这张表怎么增、怎么清、怎么按 key 隔离
    // 没人验。而它错了不会让请求被误拒——只会让内存缓慢增长或让「重置密码后
    // 仍然被限流」，两者都极难从功能上发现。
    // ──────────────────────────────────────────────────────────────────────

    async fn attempts_of(limiter: &RateLimiter, key: &str) -> usize {
        limiter
            .attempts
            .read()
            .await
            .get(key)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    #[tokio::test]
    async fn new_limiter_starts_with_empty_table() {
        // 新实例必须空表：非空意味着进程级共享状态（跨租户串扰 + 测试互相污染）。
        let limiter = RateLimiter::new();
        assert!(limiter.attempts.read().await.is_empty());
        // Default 与 new 等价
        let default_limiter = RateLimiter::default();
        assert!(default_limiter.attempts.read().await.is_empty());
    }

    #[tokio::test]
    async fn memory_window_allows_up_to_max_then_rejects_with_observable_message() {
        let limiter = RateLimiter::new();
        for _ in 0..2 {
            limiter
                .check_rate_limit("1.1.1.1", 2, 1)
                .await
                .expect("within budget");
        }

        let err = limiter
            .check_rate_limit("1.1.1.1", 2, 1)
            .await
            .expect_err("3rd attempt must be rejected");

        // 拒绝时**不能**把这次尝试也记进窗口（否则窗口会被失败请求继续填充，
        // 越限之后恢复时间被无限推后）。
        assert_eq!(attempts_of(&limiter, "1.1.1.1").await, 2);
        match err {
            AppError::BadRequest(msg) => {
                assert!(msg.contains('2'), "message should carry the count: {msg}");
                assert!(msg.contains("1 minutes"), "message should carry the window: {msg}");
            }
            other => panic!("expected BadRequest, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn budgets_are_isolated_per_key() {
        let limiter = RateLimiter::new();
        limiter.check_rate_limit("a", 1, 1).await.unwrap();

        // a 已用满，b 不受影响
        assert!(limiter.check_rate_limit("a", 1, 1).await.is_err());
        assert!(limiter.check_rate_limit("b", 1, 1).await.is_ok());
        assert_eq!(attempts_of(&limiter, "a").await, 1);
        assert_eq!(attempts_of(&limiter, "b").await, 1);
    }

    #[tokio::test]
    async fn reset_clears_only_the_named_key() {
        // 登录成功后调 reset：必须只清掉自己的 key，否则一次成功登录会把
        // 别人的失败计数也抹掉（等于给攻击者免费重试）。
        let limiter = RateLimiter::new();
        limiter.check_rate_limit("alice", 5, 1).await.unwrap();
        limiter.check_rate_limit("bob", 5, 1).await.unwrap();

        limiter.reset("alice").await;

        assert_eq!(attempts_of(&limiter, "alice").await, 0);
        assert_eq!(attempts_of(&limiter, "bob").await, 1);
        assert!(limiter.attempts.read().await.get("alice").is_none());
    }

    #[tokio::test]
    async fn record_failure_consumes_budget_like_a_check() {
        // record_failure 是「失败但不算一次正常请求」的入口（如登录密码错），
        // 它必须同样占用额度，否则暴力破解可以无限重试。
        let limiter = RateLimiter::new();
        limiter.record_failure("2.2.2.2").await;
        limiter.record_failure("2.2.2.2").await;

        assert_eq!(attempts_of(&limiter, "2.2.2.2").await, 2);
        assert!(limiter.check_rate_limit("2.2.2.2", 2, 60).await.is_err());
    }

    #[tokio::test]
    async fn cleanup_drops_entries_older_than_one_hour_and_emptied_keys() {
        // 清理窗口固定 1 小时（与 check 的 window_minutes 无关）：这是防内存泄漏的
        // 唯一机制，只保留「最近一小时内出现过」的 key。
        let limiter = RateLimiter::new();
        let now = Instant::now();
        {
            let mut table = limiter.attempts.write().await;
            table.insert("stale".to_string(), vec![now - Duration::from_secs(7200)]);
            table.insert(
                "mixed".to_string(),
                vec![now - Duration::from_secs(7200), now - Duration::from_secs(1)],
            );
            table.insert("fresh".to_string(), vec![now - Duration::from_secs(1)]);
        }

        limiter.cleanup_in_memory().await;

        let table = limiter.attempts.read().await;
        assert!(!table.contains_key("stale"), "全过期的 key 必须整条删除");
        assert_eq!(table.get("mixed").map(Vec::len), Some(1), "只删过期的时间戳");
        assert_eq!(table.get("fresh").map(Vec::len), Some(1));
    }

    #[tokio::test]
    async fn cleanup_is_also_reachable_through_public_cleanup() {
        // `cleanup()` 是给外部定时器用的公开入口，语义必须与 cleanup_in_memory 一致。
        let limiter = RateLimiter::new();
        limiter
            .attempts
            .write()
            .await
            .insert("x".to_string(), vec![Instant::now() - Duration::from_secs(7200)]);

        limiter.cleanup().await;

        assert!(limiter.attempts.read().await.is_empty());
    }

    #[tokio::test]
    async fn check_prunes_attempts_outside_the_requested_window() {
        // 窗口过期必须**逐次**判定，而不是等 1 小时清理任务：
        // 否则「1 分钟内 3 次」的限制在 3 次之后仍然生效，用户要等一小时才能重试。
        let limiter = RateLimiter::new();
        limiter.attempts.write().await.insert(
            "3.3.3.3".to_string(),
            vec![Instant::now() - Duration::from_secs(90)],
        );

        // 窗口 1 分钟，90 秒前的记录已出窗 → 允许
        assert!(limiter.check_rate_limit("3.3.3.3", 1, 1).await.is_ok());
        // 且旧记录被剔除，只剩本次
        assert_eq!(attempts_of(&limiter, "3.3.3.3").await, 1);
    }

    #[tokio::test]
    async fn init_redis_inside_runtime_declines() {
        // 与 SessionManager 同构：运行时不 block_on，返回 false 由调用方走 async 版。
        let limiter = RateLimiter::new();
        assert!(!limiter.init_redis());
        assert!(limiter.get_redis_pool().is_none());
    }

    #[tokio::test]
    async fn init_redis_async_is_idempotent_and_never_blocks_rate_limiting() {
        // 不断言「连不上 Redis」——测试机可能真的配了 REDIS_URL（.env / CI service）。
        // 断言的是两条与环境无关的契约：
        //   1. 重复调用不 panic、不重建连接池（幂等）；
        //   2. 无论 Redis 是否可用，内存限流始终能兜住。
        let limiter = RateLimiter::new();
        let first = limiter.init_redis_async().await;
        let second = limiter.init_redis_async().await;

        if first {
            assert!(second, "已初始化后必须直接返回 true");
            assert!(limiter.get_redis_pool().is_some());
        }

        assert!(
            limiter.check_rate_limit("4.4.4.4", 1, 1).await.is_ok(),
            "Redis 状态不得影响首次请求放行"
        );
        assert!(
            limiter.check_rate_limit("4.4.4.4", 1, 1).await.is_err(),
            "第二次必须被拒（内存窗口生效）"
        );
    }

    #[tokio::test]
    async fn injected_counter_bypasses_the_redis_url_gate() {
        // 测试注入是「有意模拟」，不受 REDIS_URL 门控约束 —— 否则所有
        // 分布式限流的用例都得依赖环境变量才能跑。
        struct AlwaysOne;
        #[async_trait::async_trait]
        impl crate::middleware::rate_limit_distributed::WindowCounter for AlwaysOne {
            async fn incr_window(&self, _key: &str, _window_secs: i64) -> anyhow::Result<u64> {
                Ok(1)
            }
        }

        let limiter = RateLimiter::new();
        limiter.set_window_counter_override_for_test(Some(Arc::new(AlwaysOne)));
        assert!(limiter.active_window_counter().await.is_some());
    }
}
