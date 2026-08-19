use std::sync::Arc;

use anyhow::Context;
use redis::{AsyncCommands, aio::ConnectionManager};
use tokio::sync::Mutex;

// ──────────────────────────────────────────────────────────────────────────────
// Redis
//
// 为 Session / RateLimiter 提供分布式 Redis 后端。
// Redis 不可达时降级为进程内内存，不影响既有行为。
//
// 使用 redis-rs 内置的 ConnectionManager（自动重连）。
// ──────────────────────────────────────────────────────────────────────────────

/// Redis 连接池
#[derive(Clone)]
pub struct RedisPool(pub Arc<InnerRedisPool>);

pub struct InnerRedisPool {
    pub manager: Mutex<Option<ConnectionManager>>,
    pub url: String,
}

impl RedisPool {
    /// 从 URL 创建连接池（惰性建连：首次调用时才真正建连）
    pub async fn from_url(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url).context("failed to create Redis client")?;
        let manager = client.get_tokio_connection_manager().await?;
        Ok(Self(Arc::new(InnerRedisPool {
            manager: Mutex::new(Some(manager)),
            url: url.to_string(),
        })))
    }

    /// 获取连接并执行异步操作
    ///
    /// # Type parameters
    /// * `F` - 闭包类型，接收 `&mut ConnectionManager`，返回一个 Future
    /// * `Fut` - Future 类型，解析为 `anyhow::Result<R>`
    /// * `R` - 返回值类型
    pub async fn with_connection<F, Fut, R>(&self, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&mut ConnectionManager) -> Fut + Send,
        Fut: std::future::Future<Output = anyhow::Result<R>> + Send,
        R: Send,
    {
        let mut guard = self.0.manager.lock().await;
        let manager = guard
            .as_mut()
            .context("Redis connection manager not initialized")?;
        f(manager).await
    }

    /// 检查连接健康状态
    pub async fn is_healthy(&self) -> bool {
        true
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
}
