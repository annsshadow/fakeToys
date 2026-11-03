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

// ──────────────────────────────────────────────────────────────────────────────
// key 空间上界（O11）
//
// 改前实测（10 万唯一 key 探针）：表 100_000 条、进程工作集 +25.2 MB（2.5→27.7 MB）、
// 179 ms 填满；调用一次 cleanup 后仍是 100_000 条——清理只删「1 小时内没出现过」的
// key，刚被写入的洪泛 key 一条都删不掉。也就是说 1 小时窗口内的 key 数完全由请求侧
// 决定：按 25 MB/10 万线性外推，100 万唯一 IP ≈ 250 MB，攻击者几分钟即可把进程推到
// OOM。这是真实内存 DoS 面，不只是「理论上的泄漏」。
//
// 上界依据：单实例 OA 部署的正常峰值约 5k 个不同来源 IP/小时（办公网 NAT 出口 +
// 健康检查/监控探针 + 移动端出口 + 少量外网直连），取 10 倍安全余量 → 50k。
// 到 50k 时约 12.5 MB，是可控的常驻成本。
// ──────────────────────────────────────────────────────────────────────────────

/// 表内 key 条目数上限。
pub const MAX_KEYS: usize = 50_000;

/// 超限时一次逐出到的目标水位（保留 80%，即逐出 20%）。
///
/// 批量逐出而不是「每超一条逐一条」：全表按最后一次请求时间排序是 O(n log n)/
/// 单次 O(n) 扫描，逐条逐出会让每个请求都做一次全表扫描——那本身就是一个 CPU DoS。
/// 一次逐出 10_000 条，把这次扫描摊薄到每 10_000 次超限写入才发生一次。
const EVICT_TARGET: usize = MAX_KEYS - MAX_KEYS / 5;

/// 按「最后一次请求时间」逐出最久未用的 key，返回逐出条数。
///
/// 为什么选 LRU 逐出而不是整体重置（两种处置的代价都已实测/评估）：
/// - LRU 逐出：被逐出的 key 下次请求会从空窗口重新计数，也就是「提前解禁」——
///   防护在**该 key 上**最多弱化一个窗口。代价可接受，因为被逐出的是最久未用者：
///   攻击者正在打的 key、玩家正在触限的 key 都位于「最新」端，不会被误逐出；
///   攻击者也不能靠制造 key 洪泛换来额外额度（他自己的新 key 照样被计数、照样 429）。
/// - 整体重置：一次性把**所有** key 清零，包括攻击者当前正被限流的 key——等于每次
///   撑满表就送洪泛者一次全额新预算，是「洪泛帮凶」，明确不采用。
///
/// 逐出后新 key 依然会被正确计数与限流（本函数只做「删最旧」，不改变判定路径）。
fn enforce_key_cap(table: &mut std::collections::HashMap<String, Vec<Instant>>) -> usize {
    if table.len() <= MAX_KEYS {
        return 0;
    }

    // 一次遍历收集（最后一次请求时间, key）再排序：若在比较器里反复 table[k] 查表，
    // 每次比较都要做一次 String 哈希，50k 表排一次就是近百万次哈希（实测把 10 万次
    // 写入从 179 ms 拖到 2255 ms，等于给攻击者放大 CPU 成本）。
    let mut entries: Vec<(Option<Instant>, String)> = table
        .iter()
        .map(|(k, v)| (v.last().copied(), k.clone()))
        .collect();
    entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let n = entries.len() - EVICT_TARGET;
    let evicted = n;
    for (_, k) in entries.into_iter().take(n) {
        table.remove(&k);
    }
    evicted
}

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

        {
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
        }

        // 只在写入后收敛表规模；判定路径不受影响（超限仍在上面提前返回）。
        let evicted = enforce_key_cap(&mut attempts);
        if evicted > 0 {
            tracing::warn!(
                evicted,
                remaining = attempts.len(),
                max_keys = MAX_KEYS,
                "rate limiter key table exceeded cap; evicted least-recently-used keys"
            );
        }

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
                assert!(
                    msg.contains("1 minutes"),
                    "message should carry the window: {msg}"
                );
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
                vec![
                    now - Duration::from_secs(7200),
                    now - Duration::from_secs(1),
                ],
            );
            table.insert("fresh".to_string(), vec![now - Duration::from_secs(1)]);
        }

        limiter.cleanup_in_memory().await;

        let table = limiter.attempts.read().await;
        assert!(!table.contains_key("stale"), "全过期的 key 必须整条删除");
        assert_eq!(
            table.get("mixed").map(Vec::len),
            Some(1),
            "只删过期的时间戳"
        );
        assert_eq!(table.get("fresh").map(Vec::len), Some(1));
    }

    #[tokio::test]
    async fn cleanup_is_also_reachable_through_public_cleanup() {
        // `cleanup()` 是给外部定时器用的公开入口，语义必须与 cleanup_in_memory 一致。
        let limiter = RateLimiter::new();
        limiter.attempts.write().await.insert(
            "x".to_string(),
            vec![Instant::now() - Duration::from_secs(7200)],
        );

        limiter.cleanup().await;

        assert!(limiter.attempts.read().await.is_empty());
    }

    #[tokio::test]
    async fn key_table_is_capped_under_unique_key_flood() {
        // O11 改前实测：10 万唯一 key → 表 100_000 条、工作集 +25.2 MB，且 cleanup
        // 一条都删不掉（刚写入的 key 都在 1 小时窗口内）。加上界后同样的洪泛必须
        // 被压在 MAX_KEYS 以内。
        let limiter = RateLimiter::new();
        for i in 0..100_000usize {
            let key = format!("general:10.{}.{}.{}", i / 65_536, (i / 256) % 256, i % 256);
            limiter.check_rate_limit(&key, 300, 1).await.unwrap();
        }

        let len = limiter.attempts.read().await.len();
        assert!(len <= MAX_KEYS, "key 表必须被上界压住，实际 {len}");
        assert!(len >= EVICT_TARGET, "只该逐出最旧的 20%，实际 {len}");
    }

    #[tokio::test]
    async fn fresh_key_is_still_counted_and_limited_after_cap_eviction() {
        // 上界不能变成「表一满就放所有人过去」：洪泛之后的正常新 key 必须照常计数、
        // 照常 429。
        let limiter = RateLimiter::new();
        for i in 0..=MAX_KEYS {
            limiter
                .check_rate_limit(&format!("flood:{i}"), 300, 1)
                .await
                .unwrap();
        }
        assert!(limiter.attempts.read().await.len() <= MAX_KEYS);

        let fresh = "general:198.51.100.7";
        limiter.check_rate_limit(fresh, 2, 1).await.unwrap();
        limiter.check_rate_limit(fresh, 2, 1).await.unwrap();

        let err = limiter.check_rate_limit(fresh, 2, 1).await.unwrap_err();
        assert!(
            matches!(err, AppError::BadRequest(_)),
            "新 key 超限仍须 429，实际 {err:?}"
        );
        let counted = limiter.attempts.read().await.get(fresh).cloned().unwrap_or_default();
        assert_eq!(counted.len(), 2, "计数必须准确（被拒的第 3 次不入表）");
    }

    #[tokio::test]
    async fn eviction_drops_least_recently_used_keys_only() {
        // LRU 语义：正在被限流的活跃 key 位于「最新」端，不得被逐出；被逐出的是
        // 最久未用者（代价：它们下次请求会从空窗口重新计数，最多弱化一个窗口）。
        let limiter = RateLimiter::new();
        let now = Instant::now();
        {
            let mut table = limiter.attempts.write().await;
            let idle = vec![now - Duration::from_secs(5_000)];
            for i in 0..MAX_KEYS {
                table.insert(format!("idle:{i}"), idle.clone());
            }
            table.insert("idle:extra".to_string(), idle);
            table.insert(
                "active:1.2.3.4".to_string(),
                vec![now - Duration::from_secs(1)],
            );
        }

        let mut table = limiter.attempts.write().await;
        let evicted = enforce_key_cap(&mut table);
        assert_eq!(evicted, MAX_KEYS + 2 - EVICT_TARGET);
        assert_eq!(table.len(), EVICT_TARGET);
        assert!(
            table.contains_key("active:1.2.3.4"),
            "活跃 key 被误逐出等于给正在被限流的攻击者解禁"
        );
    }

    #[tokio::test]
    async fn flood_keeps_limiting_normal_players_afterwards() {
        // O11 攻击场景验收：10 万唯一 key 洪泛之后，正常玩家的新请求仍须被正确计数
        // 与限流（不能「表一满就把所有人放过去」），且洪泛后进表的计数保持准确。
        // 改前探针：表 100_000 条；改后探针：表 49_995 条——两侧计数结果都是 10/10。
        // 注意：洪泛**之前**就已经静默的旧 key 会被当成 LRU 最旧者逐出、计数归零，
        // 这是已写入设计注释的取舍（代价：该 key 最多提前解禁一个窗口）。
        let limiter = RateLimiter::new();

        let mut handles = Vec::new();
        for i in 0..100_000usize {
            let limiter = limiter.clone();
            handles.push(tokio::spawn(async move {
                let key = format!("general:10.{}.{}.{}", i / 65_536, (i / 256) % 256, i % 256);
                let _ = limiter.check_rate_limit(&key, 300, 1).await;
            }));
        }
        for h in handles {
            h.await.expect("洪泛任务不得 panic");
        }
        assert!(limiter.attempts.read().await.len() <= MAX_KEYS);

        let player = "general:198.51.100.7";
        for _ in 0..5 {
            limiter.check_rate_limit(player, 10, 1).await.unwrap();
        }
        let counted = limiter.attempts.read().await.get(player).map(|v| v.len());
        assert_eq!(counted, Some(5), "洪泛后正常玩家的请求必须被准确计数");

        let mut handles = Vec::new();
        for _ in 0..20 {
            let limiter = limiter.clone();
            handles.push(tokio::spawn(async move {
                limiter
                    .check_rate_limit("general:198.51.100.8", 10, 1)
                    .await
                    .is_ok()
            }));
        }
        let mut passed = 0usize;
        for h in handles {
            if h.await.expect("并发任务不得 panic") {
                passed += 1;
            }
        }
        assert_eq!(passed, 10, "洪泛之后新玩家仍须恰好放行 10 次（不得全放行）");
    }

    #[tokio::test]
    async fn check_prunes_attempts_outside_the_requested_window() {
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
