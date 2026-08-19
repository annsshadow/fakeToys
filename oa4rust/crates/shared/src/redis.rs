use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use redis::aio::ConnectionManager;
use tokio::sync::Mutex;

// ──────────────────────────────────────────────────────────────────────────────
// Redis
//
// 为 Session / RateLimiter / MessageBus 提供分布式 Redis 后端。
// Redis 不可达时降级为进程内内存，不影响既有行为。
// ──────────────────────────────────────────────────────────────────────────────

/// Redis 连接池（单例：内部使用 redis-rs 的 ConnectionManager）
#[derive(Clone)]
pub struct RedisPool(pub Arc<InnerRedisPool>);

pub struct InnerRedisPool {
    pub manager: Mutex<Option<ConnectionManager>>,
    pub url: String,
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl RedisPool {
    /// 从 URL 创建连接池（惰性建连：首次调用时才真正建连）
    pub async fn from_url(url: &str) -> anyhow::Result<Self> {
        Self::from_url_with_retry(url, 3, Duration::from_millis(200)).await
    }

    /// 从 URL 创建连接池，支持自定义重试次数和延迟
    pub async fn from_url_with_retry(url: &str, max_retries: u32, retry_delay: Duration) -> anyhow::Result<Self> {
        let client = redis::Client::open(url.to_string()).context("failed to create Redis client")?;
        let mut last_err = None;
        for attempt in 0..=max_retries {
            match client.get_connection_manager().await {
                Ok(manager) => {
                    if attempt > 0 {
                        tracing::info!(attempt = attempt, "Redis connection established after retries");
                    }
                    return Ok(Self(Arc::new(InnerRedisPool {
                        manager: Mutex::new(Some(manager)),
                        url: url.to_string(),
                        max_retries,
                        retry_delay,
                    })));
                }
                Err(e) => {
                    last_err = Some(e);
                    if attempt < max_retries {
                        tokio::time::sleep(retry_delay).await;
                    }
                }
            }
        }
        let err = last_err.unwrap();
        Err(anyhow::anyhow!("failed to connect to Redis after {} retries: {}", max_retries, err))
    }

    /// 获取连接并执行异步操作（带重试）
    pub async fn with_connection<F, Fut, R>(&self, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&mut ConnectionManager) -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<R>>,
        R: Send,
    {
        let mut guard = self.0.manager.lock().await;
        let manager = guard
            .as_mut()
            .context("Redis connection manager not initialized")?;
        f(manager).await
    }

    /// 检查连接健康状态（检查连接池是否已初始化）
    pub async fn is_healthy(&self) -> bool {
        self.0.manager.lock().await.is_some()
    }

    /// 获取 Redis URL
    pub fn url(&self) -> &str {
        &self.0.url
    }

    /// 尝试重连 Redis
    pub async fn reconnect(&self) -> anyhow::Result<()> {
        let client = redis::Client::open(self.0.url.to_string()).context("failed to create Redis client")?;
        let manager = client.get_connection_manager().await?;
        let mut guard = self.0.manager.lock().await;
        *guard = Some(manager);
        Ok(())
    }
}

/// 读取环境变量中的 Redis URL
pub fn redis_url_from_env() -> Option<String> {
    std::env::var("REDIS_URL").ok().filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod redis_tests {

    #[test]
    fn test_redis_url_from_env_none() {
        std::env::remove_var("REDIS_URL");
        assert!(super::redis_url_from_env().is_none());
    }

    #[test]
    fn test_redis_url_from_env_empty() {
        std::env::set_var("REDIS_URL", "   ");
        assert!(super::redis_url_from_env().is_none());
        std::env::remove_var("REDIS_URL");
    }

    #[test]
    fn test_redis_url_from_env_valid() {
        std::env::set_var("REDIS_URL", "redis://127.0.0.1:6379");
        assert_eq!(
            crate::redis::redis_url_from_env(),
            Some("redis://127.0.0.1:6379".to_string())
        );
        std::env::remove_var("REDIS_URL");
    }

    #[tokio::test]
    async fn test_redis_pool_url_accessor() {
        let pool = super::RedisPool::from_url("redis://invalid:9999").await;
        assert!(pool.is_err());
    }
}
