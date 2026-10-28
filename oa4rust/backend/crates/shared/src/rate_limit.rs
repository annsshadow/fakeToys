// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::error::AppError;

// ──────────────────────────────────────────────────────────────────────────────
// rate_limit
//
// 进程内滑动窗口频率限制器。单实例部署（2026-10-28 决策），无分布式计数。
// RateLimiter 由 main.rs 构造单一实例注入各 router 与速率限制中间件，
// 统一对认证接口与普通接口按客户端 IP 限流。
// ──────────────────────────────────────────────────────────────────────────────

/// 内存模式清理间隔（秒）
const CLEANUP_INTERVAL_SECONDS: u64 = 60;

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
        let limiter = Self {
            attempts: Arc::new(RwLock::new(std::collections::HashMap::new())),
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
    pub async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: i32,
        window_minutes: i64,
    ) -> Result<(), AppError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    // ──────────────────────────────────────────────────────────────────────
    // 这一组补的是**内存滑动窗口自身的簿记**，不是限流决策。
    //
    // `attempts` 这张表怎么增、怎么清、怎么按 key 隔离没人验。而它错了不会让
    // 请求被误拒——只会让内存缓慢增长或让「重置密码后仍然被限流」，两者都
    // 极难从功能上发现。
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
}
