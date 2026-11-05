// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//
//! O26-2：维护作业周期触发（仅 `loglongdetained` / `touchdelay` 两条已批能力）。
//!
//! 与 `timer::TimerRegistry` 同一模式：`router()` 构造时 spawn 后台循环。
//! 本仓库没有 cron，这里只做固定间隔的「业务日幂等」维护，不引入周期表达式。
//!
//! 首轮不立即执行：先消费一次 `interval` 的立即 tick，再等一个完整周期，
//! 避免进程启动瞬间对生产库产生未预期的批量写入（测试环境也不会被污染）。
//! 间隔由 `MAINTENANCE_INTERVAL_SECS` 覆盖，默认 3600s；便于运行时观测取证。

use deadpool_postgres::Pool;
use std::time::Duration;

pub const DEFAULT_INTERVAL_SECS: u64 = 3600;

fn interval_secs() -> u64 {
    std::env::var("MAINTENANCE_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v >= 1)
        .unwrap_or(DEFAULT_INTERVAL_SECS)
}

/// 启动维护循环：每 `interval_secs` 依次执行两条已批能力，逐条记录计数/失败。
pub fn start_background(pool: Pool) {
    let secs = interval_secs();
    tokio::spawn(async move {
        tracing::info!(
            interval_secs = secs,
            "maintenance background started (loglongdetained, touchdelay)"
        );
        let mut ticker = tokio::time::interval(Duration::from_secs(secs));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        // 消费 interval 的立即 tick：首个周期完整等待后再执行。
        ticker.tick().await;
        loop {
            ticker.tick().await;
            match crate::u2::run_log_long_detained(&pool).await {
                Ok(rows) => {
                    tracing::info!(
                        capability = "loglongdetained",
                        inserted = rows.len(),
                        "maintenance run done"
                    )
                }
                Err(e) => {
                    tracing::warn!(capability = "loglongdetained", error = %e, "maintenance run failed")
                }
            }
            match crate::u2::run_touch_delay(&pool).await {
                Ok(rows) => {
                    tracing::info!(
                        capability = "touchdelay",
                        inserted = rows.len(),
                        "maintenance run done"
                    )
                }
                Err(e) => {
                    tracing::warn!(capability = "touchdelay", error = %e, "maintenance run failed")
                }
            }
        }
    });
}
