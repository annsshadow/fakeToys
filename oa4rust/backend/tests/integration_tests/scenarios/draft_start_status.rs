// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 5：`draft/start` 写 `xstatus='started'`（2 条基线，GET/POST 共用 handler）。
//!
//! 裁定：不删路由、不降级为只 touch `xupdateTime`；字面量收进具名常量
//! `DRAFT_STARTED_STATUS`（当前唯一写方）。schema 取证：`pp_c_draft.xstatus` 是 text、
//! 可空、无 DEFAULT、无 CHECK；全仓无读取方。
//!
//! 本用例钉住：
//! 1. POST `/api/processplatform/assemble/surface/draft/start/{id}` → 200，body 语义不变
//!    （`type=success`、`data.id` 回显、`data.updateTime` 非空）；
//! 2. GET `/api/processplatform/assemble/surface/draft/{id}/start` → 200 同上；
//! 3. 回库 `xstatus='started'` 且 `xupdateTime` 已更新。
//!
//! 破坏实验锚点：把绑定改回 `&[&id]`（去掉常量子句）→ 首条红（Parameters 失配）。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const DRAFT_ID: &str = "it-draft-start";

pub async fn draft_start_status_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    seed_draft(&pool).await;
    case_post_start(&client, &base, &auth, &pool).await;
    case_get_start(&client, &base, &auth, &pool).await;

    server.abort();
    let _ = server.await;
}

async fn case_post_start(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = request(
        client,
        auth,
        &format!("{base}/api/processplatform/assemble/surface/draft/start/{DRAFT_ID}"),
        true,
    )
    .await;
    assert_success(&raw);
    assert_stored_started(pool).await;
}

async fn case_get_start(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = request(
        client,
        auth,
        &format!("{base}/api/processplatform/assemble/surface/draft/{DRAFT_ID}/start"),
        false,
    )
    .await;
    assert_success(&raw);
    assert_stored_started(pool).await;
}

fn assert_success((status, raw): &(reqwest::StatusCode, String)) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "draft/start 应为 200（旧实现 SET xstatus = $1 只绑 id → Parameters 失配），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["id"], DRAFT_ID, "body={raw}");
    assert!(
        body["data"]["updateTime"]
            .as_str()
            .map(|s| !s.is_empty())
            .unwrap_or(false),
        "data.updateTime 必须非空，body={raw}"
    );
}

async fn assert_stored_started(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT xstatus, \"xupdateTime\" FROM pp_c_draft WHERE xid = $1",
            &[&DRAFT_ID],
        )
        .await
        .expect("read draft failed");
    assert_eq!(
        row.get::<_, Option<String>>("xstatus").as_deref(),
        Some("started"),
        "xstatus 必须是裁定字面量 started"
    );
    assert!(
        row.get::<_, Option<String>>("xupdateTime")
            .map(|s| !s.is_empty())
            .unwrap_or(false),
        "xupdateTime 必须已更新"
    );
}

async fn seed_draft(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute("DELETE FROM pp_c_draft WHERE xid = $1", &[&DRAFT_ID])
        .await
        .expect("delete draft failed");
    db.execute(
        "INSERT INTO pp_c_draft \
         (id, xid, xtitle, \"xcreateTime\", \"xupdateTime\", xstatus, deleted_at) \
         VALUES ($1, $2, 'it draft', '2026-11-05 00:00:00', '', NULL, NULL)",
        &[&DRAFT_ID, &DRAFT_ID],
    )
    .await
    .expect("insert draft failed");
}

async fn request(
    client: &Client,
    auth: &str,
    url: &str,
    post: bool,
) -> (reqwest::StatusCode, String) {
    let builder = if post {
        client.post(url)
    } else {
        client.get(url)
    };
    let resp = builder
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
