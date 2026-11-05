// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O25：`x_timer_job` 的真实行为契约（once 任务）。
//!
//! 修复前的三个既有缺陷：
//! 1. 写入口把 `fire_at` / `created_at` 按 String 绑定 TIMESTAMP → 真实 POST 500；
//! 2. ticker 每 30s 重复 fire，且不检查 `fired_at` / `cancelled_at`；
//! 3. `restore` 从未接线，重启后未完成 job 彻底失联。
//!
//! 本文件按节点拆成独立入口，由 integration_runner 分别调用。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use tokio::time::sleep;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;
use processplatform_service_processing::timer::TimerRegistry;

fn uniq(prefix: &str) -> String {
    format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

/// O25 节点 1：真实 POST `timer/start` → 200，且回库行存在、字段类型与值正确。
pub async fn timer_start_write_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    let work_id = uniq("tw");
    let pg = pool.as_pg().expect("postgres pool");
    {
        let c = pg.get().await.expect("pool client");
        c.execute(
            "INSERT INTO x_work (id, title, process, creator) VALUES ($1, 'Timer Work', 'proc-timer', 'it-admin')",
            &[&work_id],
        )
        .await
        .expect("seed x_work for timer job FK");
    }

    // 未来 1 小时，避免 30s ticker 在用例内提前触发。
    let fire_at = Utc::now().naive_utc() + chrono::Duration::hours(1);
    let fire_at_str = fire_at.format("%Y-%m-%d %H:%M:%S").to_string();

    let resp = client
        .post(format!(
            "{base}/api/processplatform/service/processing/timer/start"
        ))
        .header("Authorization", &auth)
        .json(&json!({ "workId": work_id, "fireAt": fire_at_str }))
        .send()
        .await
        .expect("POST timer/start failed");
    let status = resp.status();
    let body: Value = resp.json().await.expect("timer/start non-json body");
    assert_eq!(
        status,
        StatusCode::OK,
        "POST timer/start 应为 200（修前 TIMESTAMP 按 String 绑定是 500）：{body}"
    );
    let job_id = body["data"]["jobId"]
        .as_str()
        .expect("timer/start 响应应含 data.jobId")
        .to_string();
    assert_eq!(body["data"]["workId"].as_str(), Some(work_id.as_str()));

    let row = pg
        .get()
        .await
        .expect("pool client")
        .query_opt(
            "SELECT fire_at, created_at, cron, kind, fired_at, cancelled_at \
             FROM x_timer_job WHERE id = $1",
            &[&job_id],
        )
        .await
        .expect("read back x_timer_job")
        .unwrap_or_else(|| panic!("timer/start 返回 200 但库里没有 job {job_id}"));

    let stored_fire_at: NaiveDateTime = row.get("fire_at");
    let stored_created_at: NaiveDateTime = row.get("created_at");
    assert_eq!(
        stored_fire_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        fire_at_str,
        "fire_at 应原样落库（NaiveDateTime 绑定，不被截断/改写）"
    );
    assert!(
        stored_created_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str()
            > "2020-01-01 00:00:00",
        "created_at 应落真实时间，实际 {stored_created_at}"
    );
    assert_eq!(row.get::<_, Option<String>>("cron"), None);
    assert_eq!(row.get::<_, String>("kind"), "once");
    assert_eq!(
        row.get::<_, Option<NaiveDateTime>>("fired_at"),
        None,
        "刚创建的 job 不应已 fired"
    );
    assert_eq!(
        row.get::<_, Option<NaiveDateTime>>("cancelled_at"),
        None,
        "刚创建的 job 不应已取消"
    );

    println!("timer/start 写后回库: job={job_id} fire_at={stored_fire_at}");

    server.abort();
    let _ = server.await;
}

async fn seed_work(pg: &deadpool_postgres::Pool, work_id: &str) {
    pg.get()
        .await
        .expect("pool client")
        .execute(
            "INSERT INTO x_work (id, title, process, creator) VALUES ($1, 'Timer Work', 'proc-timer', 'it-admin')",
            &[&work_id],
        )
        .await
        .expect("seed x_work for timer job FK");
}

async fn read_fired_at(pg: &deadpool_postgres::Pool, job_id: &str) -> Option<NaiveDateTime> {
    pg.get()
        .await
        .expect("pool client")
        .query_one("SELECT fired_at FROM x_timer_job WHERE id = $1", &[&job_id])
        .await
        .expect("read fired_at")
        .get("fired_at")
}

/// O25 节点 2：到期的 once job 只执行一次——连续 ≥3 个 tick 后 `fired_at`
/// 仍只被写一次，且该 job 已从内存待执行集合移除。
pub async fn timer_once_fires_once_flow(pool: Arc<TestPool>) {
    let pg = pool.as_pg().expect("postgres pool").clone();
    let work_id = uniq("tw");
    seed_work(&pg, &work_id).await;

    let registry = TimerRegistry::with_pool(pg.clone());
    let fire_at = Utc::now().naive_utc() - chrono::Duration::seconds(1);
    let job_id = registry
        .start(work_id, None, fire_at, None)
        .await
        .expect("timer.start 应写入 x_timer_job");

    // 200ms 一个 tick：等待 900ms = ≥4 个 tick 周期。
    registry.start_background_with_interval(Duration::from_millis(200));
    sleep(Duration::from_millis(900)).await;

    let first = read_fired_at(&pg, &job_id)
        .await
        .expect("到期 job 应在首个 tick 被 fire（fired_at 非空）");
    assert!(
        registry.jobs.read().await.get(&job_id).is_none(),
        "fire 后必须从待执行集合移除，否则下一个 tick 会再次执行"
    );

    // 再等 700ms（≥3 个 tick）：fired_at 必须未被覆盖。
    sleep(Duration::from_millis(700)).await;
    let second = read_fired_at(&pg, &job_id)
        .await
        .expect("fired_at 不应被清空");
    assert_eq!(
        first, second,
        "fired_at 只能被写一次；再等 3 个 tick 后不应变化（实际 {first} → {second}）"
    );

    println!("timer once: job={job_id} fired_at={first}（≥3 tick 后未变）");
}
