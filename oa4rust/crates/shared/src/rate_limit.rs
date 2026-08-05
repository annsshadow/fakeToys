use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::error::AppError;

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit
//
// 内存滑动窗口频率限制器。RateLimiter 由 main.rs 构造单一实例注入
// 各 router 与速率限制中间件，统一对认证接口（10 次/分钟/IP）与
// 普通接口（100 次/分钟/IP）限流。
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct RateLimiter {
    pub attempts: Arc<RwLock<std::collections::HashMap<String, Vec<Instant>>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            attempts: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 检查是否超出频率限制（滑动窗口）
    ///
    /// 记录每个 key（如 IP）的每次请求时间戳，在指定时间窗口内超过上限则返回错误。
    pub async fn check_rate_limit(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let window = Duration::from_secs((window_minutes * 60) as u64);
        let window_start = now - window;

        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.retain(|&t| t > window_start);

        if entry.len() >= max_attempts as usize {
            return Err(AppError::BadRequest(
                format!("rate limit exceeded: {} attempts in last {} minutes", entry.len(), window_minutes)
            ));
        }

        entry.push(now);
        Ok(())
    }

    /// 记录一次失败尝试（递增计数器）
    pub async fn record_failure(&self, key: &str) {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.push(now);
    }

    /// 重置指定 key 的尝试计数（登录成功后调用）
    pub async fn reset(&self, key: &str) {
        self.attempts.write().await.remove(key);
    }
}
