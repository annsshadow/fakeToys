use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use redis::AsyncCommands;

use crate::error::AppError;
use crate::redis::RedisPool;

// ──────────────────────────────────────────────────────────────────────────────
// 分布式限流（plan002 U7c）
//
// REDIS_URL 存在时，用 Redis INCR+EXPIRE 做分布式滑动窗口计数，
// key = rate:{client_ip}:{window_secs}，替代进程内存计数；
// Redis 不可达（初始化失败或运行期操作失败）时降级回进程内存限流并 warn。
//
// 可测试性：Redis 协议交互抽象为 `WindowCounter` trait。
// 真实实现为 `RedisWindowCounter`；单元测试用内存 Mock / 故障 Mock 验证
// 窗口逻辑与降级路径。中间件签名保持不变，策略切换仅发生在内部。
// ──────────────────────────────────────────────────────────────────────────────

/// 构造分布式限流 key：rate:{client_ip}:{window_secs}
pub fn distributed_rate_key(client_key: &str, window_secs: i64) -> String {
    format!("rate:{client_key}:{window_secs}")
}

/// 分布式窗口计数器抽象：INCR + 首次 EXPIRE 的原子语义
#[async_trait]
pub trait WindowCounter: Send + Sync {
    /// 计数自增 1；首次自增（返回值 == 1）时设置 TTL 为 window_secs。
    /// 返回自增后的当前窗口计数；Redis 不可达时返回 Err（由调用方降级）。
    async fn incr_window(&self, key: &str, window_secs: i64) -> anyhow::Result<u64>;
}

/// 基于 Redis INCR+EXPIRE 的真实实现
pub struct RedisWindowCounter {
    pool: RedisPool,
}

impl RedisWindowCounter {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WindowCounter for RedisWindowCounter {
    async fn incr_window(&self, key: &str, window_secs: i64) -> anyhow::Result<u64> {
        let mut guard = self.pool.0.manager.lock().await;
        let conn = guard
            .as_mut()
            .context("Redis connection manager not initialized")?;
        let current: u64 = conn.incr(key, 1).await?;
        if current == 1 {
            conn.expire::<_, ()>(key, window_secs).await?;
        }
        Ok(current)
    }
}

/// 纯决策逻辑：当前窗口计数达到上限即拒绝（与既有 Redis 路径语义一致）。
pub fn window_decision(count: u64, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
    if (count as i64) >= max_attempts as i64 {
        return Err(AppError::BadRequest(format!(
            "rate limit exceeded: {} attempts in last {} minutes",
            count, window_minutes
        )));
    }
    Ok(())
}

/// 门控策略选择（纯函数）：
/// - 未配置 REDIS_URL：一律进程内存限流；
/// - 配置了 REDIS_URL 且计数器可用：分布式限流；
/// - 配置了 REDIS_URL 但计数器不可用：降级内存限流（初始化阶段已 warn）。
pub fn select_distributed_policy(
    redis_url_configured: bool,
    counter: Option<Arc<dyn WindowCounter>>,
) -> Option<Arc<dyn WindowCounter>> {
    if redis_url_configured {
        counter
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rate_limit::RateLimiter;
    use std::collections::HashMap;

    /// 内存 Mock 计数器：验证窗口计数语义（首次自增设置 TTL、独立 key 计数）
    #[derive(Default)]
    struct MockWindowCounter {
        counts: std::sync::Mutex<HashMap<String, u64>>,
        ttl_keys: std::sync::Mutex<Vec<(String, i64)>>,
    }

    impl MockWindowCounter {
        fn count_of(&self, key: &str) -> u64 {
            *self.counts.lock().unwrap().get(key).unwrap_or(&0)
        }

        fn expire_calls(&self) -> usize {
            self.ttl_keys.lock().unwrap().len()
        }
    }

    #[async_trait]
    impl WindowCounter for MockWindowCounter {
        async fn incr_window(&self, key: &str, window_secs: i64) -> anyhow::Result<u64> {
            let mut counts = self.counts.lock().unwrap();
            let entry = counts.entry(key.to_string()).or_insert(0);
            *entry += 1;
            if *entry == 1 {
                self.ttl_keys.lock().unwrap().push((key.to_string(), window_secs));
            }
            Ok(*entry)
        }
    }

    /// 永远失败的 Mock：模拟 Redis 不可达
    struct FailingWindowCounter {
        calls: std::sync::atomic::AtomicUsize,
    }

    impl FailingWindowCounter {
        fn new() -> Self {
            Self { calls: std::sync::atomic::AtomicUsize::new(0) }
        }

        fn calls(&self) -> usize {
            self.calls.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl WindowCounter for FailingWindowCounter {
        async fn incr_window(&self, _key: &str, _window_secs: i64) -> anyhow::Result<u64> {
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Err(anyhow::anyhow!("connection refused"))
        }
    }

    #[test]
    fn test_distributed_key_format() {
        // 契约：key = rate:{client_ip}:{window_secs}
        assert_eq!(distributed_rate_key("1.2.3.4", 60), "rate:1.2.3.4:60");
        assert_eq!(distributed_rate_key("10.0.0.9", 3600), "rate:10.0.0.9:3600");
    }

    #[test]
    fn test_window_decision_boundaries() {
        // max=3：第 1、2 次放行，第 3 次（count==max）拒绝
        assert!(window_decision(1, 3, 1).is_ok());
        assert!(window_decision(2, 3, 1).is_ok());
        assert!(window_decision(3, 3, 1).is_err());
        assert!(window_decision(100, 3, 1).is_err());
        // 错误信息应携带计数与窗口分钟数，便于排查
        match window_decision(5, 3, 2) {
            Err(AppError::BadRequest(msg)) => {
                assert!(msg.contains("5"), "message should contain count: {msg}");
                assert!(msg.contains("2 minutes"), "message should contain window: {msg}");
            }
            other => panic!("expected BadRequest, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_mock_counter_window_logic() {
        let mock = MockWindowCounter::default();
        let key = distributed_rate_key("9.9.9.9", 60);

        assert_eq!(mock.incr_window(&key, 60).await.unwrap(), 1);
        assert_eq!(mock.incr_window(&key, 60).await.unwrap(), 2);
        assert_eq!(mock.incr_window(&key, 60).await.unwrap(), 3);

        // TTL 只在首次自增时设置一次（EXPIRE 幂等窗口起点语义）
        assert_eq!(mock.expire_calls(), 1);

        // 不同 client_ip 的窗口相互独立
        let other = distributed_rate_key("8.8.8.8", 60);
        assert_eq!(mock.incr_window(&other, 60).await.unwrap(), 1);
        assert_eq!(mock.expire_calls(), 2);
        assert_eq!(mock.count_of(&key), 3);
        assert_eq!(mock.count_of(&other), 1);

        // 决策层基于 mock 计数正确拦截
        assert!(window_decision(mock.count_of(&other), 1, 1).is_err());
    }

    #[test]
    fn test_select_policy_gates_on_redis_url() {
        // REDIS_URL 未配置：即使有可用计数器也不启用分布式限流
        let counter: Arc<dyn WindowCounter> = Arc::new(MockWindowCounter::default());
        assert!(select_distributed_policy(false, Some(counter.clone())).is_none());

        // REDIS_URL 已配置且计数器可用：启用分布式限流
        assert!(select_distributed_policy(true, Some(counter)).is_some());

        // REDIS_URL 已配置但初始化失败（counter=None）：降级内存限流
        assert!(select_distributed_policy(true, None).is_none());
        // REDIS_URL 未配置且无计数器：内存限流
        assert!(select_distributed_policy(false, None).is_none());
    }

    #[tokio::test]
    async fn test_degrade_to_memory_when_redis_unreachable() {
        // 无真实 Redis 环境：注入故障计数器，验证运行期降级到内存限流
        let failing = Arc::new(FailingWindowCounter::new());
        let limiter = RateLimiter::new();
        limiter.set_window_counter_override_for_test(Some(failing.clone() as Arc<dyn WindowCounter>));

        // max=2：前两次请求正常放行（Redis 失败后由内存窗口接管），第三次超限
        assert!(limiter.check_rate_limit("7.7.7.7", 2, 1).await.is_ok());
        assert!(limiter.check_rate_limit("7.7.7.7", 2, 1).await.is_ok());
        assert!(limiter.check_rate_limit("7.7.7.7", 2, 1).await.is_err());

        // 每次检查都先尝试过 Redis（共 3 次），失败才降级 —— 证明走了降级路径而非报错传播
        assert_eq!(failing.calls(), 3);
    }

    #[tokio::test]
    async fn test_memory_only_mode_without_redis() {
        // 全新实例（无 Redis pool、无注入）：纯内存限流仍然生效
        let limiter = RateLimiter::new();
        assert!(limiter.check_rate_limit("6.6.6.6", 2, 1).await.is_ok());
        assert!(limiter.check_rate_limit("6.6.6.6", 2, 1).await.is_ok());
        assert!(limiter.check_rate_limit("6.6.6.6", 2, 1).await.is_err());
    }

    #[tokio::test]
    async fn test_mock_counter_backed_limiter_blocks_at_threshold() {
        // 注入可用的 mock 计数器：验证限流决策基于分布式计数生效
        let limiter = RateLimiter::new();
        limiter.set_window_counter_override_for_test(Some(Arc::new(MockWindowCounter::default())));

        assert!(limiter.check_rate_limit("5.5.5.5", 3, 1).await.is_ok());
        assert!(limiter.check_rate_limit("5.5.5.5", 3, 1).await.is_ok());
        assert!(limiter.check_rate_limit("5.5.5.5", 3, 1).await.is_err()); // 第 3 次达到上限即拒
        // 其他 IP 不受影响
        assert!(limiter.check_rate_limit("4.4.4.4", 3, 1).await.is_ok());
    }
}
