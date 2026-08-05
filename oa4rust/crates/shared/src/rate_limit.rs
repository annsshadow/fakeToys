use chrono::{DateTime, Utc};
use std::sync::Arc;
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
    pub attempts: Arc<RwLock<std::collections::HashMap<String, (i32, DateTime<Utc>)>>>,
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

    /// 检查是否超出频率限制
    ///
    /// 记录每个 key（如 IP）的尝试次数，在指定时间窗口内超过上限则返回错误。
    pub async fn check_rate_limit(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        let mut attempts = self.attempts.write().await;
        let now = Utc::now();

        if let Some((count, last_attempt)) = attempts.get(key) {
            let elapsed = now - *last_attempt;
            if elapsed.num_minutes() < window_minutes
                && *count >= max_attempts {
                    return Err(AppError::BadRequest(
                        format!("rate limit exceeded: {} attempts in last {} minutes", count, window_minutes)
                    ));
                }
        }

        let count = attempts.get(key).map(|(c, _)| c + 1).unwrap_or(1);
        attempts.insert(key.to_string(), (count, now));
        Ok(())
    }

    /// 记录一次失败尝试（递增计数器）
    pub async fn record_failure(&self, key: &str) {
        let mut attempts = self.attempts.write().await;
        let now = Utc::now();
        let count = attempts.get(key).map(|(c, _)| c + 1).unwrap_or(1);
        attempts.insert(key.to_string(), (count, now));
    }

    /// 重置指定 key 的尝试计数（登录成功后调用）
    pub async fn reset(&self, key: &str) {
        self.attempts.write().await.remove(key);
    }
}
