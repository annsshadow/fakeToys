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

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

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

    let work_id = uniq("timer-work");
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
