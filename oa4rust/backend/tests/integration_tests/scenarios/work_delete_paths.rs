// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 12：`E23503`（`DELETE work/{id}` 先删父表触发 FK 违约）。
//!
//! `x_timer_job.work_id` 是指向 `x_work` 的 FK（NO ACTION，information_schema 现场确认），
//! 旧实现先删 `x_work` 再清 `x_task` / `x_draft`，只要该 work 有定时任务就
//! `E23503 foreign_key_violation` → 500。改为「先子后父」：timer_job → task → draft → work。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const WORK_ID: &str = "it-work-delete";
const MISSING_ID: &str = "it-work-missing";

pub async fn work_delete_paths_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    seed(&pool).await;

    let raw = send(
        &client,
        &auth,
        reqwest::Method::DELETE,
        &format!("{base}/api/processplatform/service/processing/work/{WORK_ID}"),
    )
    .await;
    assert_eq!(
        raw.0,
        reqwest::StatusCode::OK,
        "有定时任务的 work 删除应 200（旧实现先删父表 → E23503 500），实际 {}，body={}",
        raw.0,
        raw.1
    );
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={}", raw.1);
    assert_eq!(body["data"]["value"], true, "body={}", raw.1);

    // 写后必读：四张表都不再有该 work 的行。
    let db = pg_client(&pool).await;
    for (table, column) in [
        ("x_work", "id"),
        ("x_task", "work"),
        ("x_draft", "work_id"),
        ("x_timer_job", "work_id"),
    ] {
        let left: i64 = db
            .query_one(
                &format!("SELECT COUNT(*) FROM {table} WHERE {column} = $1"),
                &[&WORK_ID],
            )
            .await
            .expect("write-then-read failed")
            .get(0);
        assert_eq!(left, 0, "{table} 仍残留该 work 的行");
    }

    // 回归：不存在的 work 仍是「200 + type=error」旧语义，没有被改成 5xx。
    let raw = send(
        &client,
        &auth,
        reqwest::Method::DELETE,
        &format!("{base}/api/processplatform/service/processing/work/{MISSING_ID}"),
    )
    .await;
    assert_eq!(raw.0, reqwest::StatusCode::OK, "body={}", raw.1);
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["type"], "error", "body={}", raw.1);

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "INSERT INTO x_work (id, title, process, creator) VALUES ($1, 'it-work', 'it-proc', 'it-admin')",
        &[&WORK_ID],
    )
    .await
    .expect("seed x_work failed");
    db.execute(
        "INSERT INTO x_task (id, work) VALUES ('it-task-delete', $1)",
        &[&WORK_ID],
    )
    .await
    .expect("seed x_task failed");
    db.execute(
        "INSERT INTO x_draft (id, work_id) VALUES ('it-draft-delete', $1)",
        &[&WORK_ID],
    )
    .await
    .expect("seed x_draft failed");
    db.execute(
        "INSERT INTO x_timer_job (id, work_id, fire_at, created_at, kind) \
         VALUES ('it-timer-delete', $1, '2026-01-01 00:00:00', '2026-01-01 00:00:00', 'start')",
        &[&WORK_ID],
    )
    .await
    .expect("seed x_timer_job failed");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_timer_job WHERE work_id = $1", &[&WORK_ID])
        .await;
    let _ = db
        .execute("DELETE FROM x_task WHERE work = $1", &[&WORK_ID])
        .await;
    let _ = db
        .execute("DELETE FROM x_draft WHERE work_id = $1", &[&WORK_ID])
        .await;
    let _ = db
        .execute("DELETE FROM x_work WHERE id = $1", &[&WORK_ID])
        .await;
}

async fn send(
    client: &Client,
    auth: &str,
    method: reqwest::Method,
    url: &str,
) -> (reqwest::StatusCode, String) {
    let resp = client
        .request(method, url)
        .header("Authorization", auth)
        .send()
        .await
        .expect("request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("body");
    (status, raw)
}

async fn pg_client(pool: &Arc<TestPool>) -> deadpool_postgres::Object {
    pool.as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client")
}
