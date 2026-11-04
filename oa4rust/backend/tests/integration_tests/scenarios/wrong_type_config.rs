// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 7：`wrong-type` 族的 bbs / general 配置写路径。
//!
//! - bbs：`max_forum_count` 实列 `integer`，旧实现绑 `i64` → `ToSql(1) WrongType { Int4, i64 }`。
//! - general：`maintenance_mode` / `allow_registration` 实列 `text`（无 CHECK/DEFAULT/行），
//!   旧实现绑 `bool` → `ToSql(0) WrongType { Text, bool }`。写入用 `$n::bool::text`
//!   （Postgres bool 字面量 `true`/`false`），读取用 `::bool` 强转，两侧保持同一契约。
//!
//! 每条路由都跑 POST + PUT（共用 handler），回库校验落值。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const BBS_CONFIG_ID: &str = "it-bbs-config";
const GENERAL_CONFIG_ID: &str = "global";

pub async fn wrong_type_config_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    seed_bbs_config(&pool).await;
    seed_general_config(&pool).await;

    case_bbs_post(&client, &base, &auth, &pool).await;
    case_bbs_put(&client, &base, &auth, &pool).await;
    case_general_post(&client, &base, &auth, &pool).await;
    case_general_put(&client, &base, &auth, &pool).await;
    case_general_read_after_write(&client, &base, &auth).await;

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed_bbs_config(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute("DELETE FROM x_bbs_assemble_control_config", &[])
        .await
        .expect("cleanup bbs config failed");
    db.execute(
        "INSERT INTO x_bbs_assemble_control_config (id, enabled, max_forum_count, allow_anonymous, create_time) \
         VALUES ($1, false, 1000, false, NOW())",
        &[&BBS_CONFIG_ID],
    )
    .await
    .expect("seed bbs config failed");
}

async fn seed_general_config(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute("DELETE FROM x_general_assemble_control_config", &[])
        .await
        .expect("cleanup general config failed");
    db.execute(
        "INSERT INTO x_general_assemble_control_config (id, maintenance_mode, allow_registration) \
         VALUES ($1, 'false', 'true')",
        &[&GENERAL_CONFIG_ID],
    )
    .await
    .expect("seed general config failed");
}

async fn case_bbs_post(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/bbs/assemble/control/update/control/config"),
        Some(json!({ "enabled": true, "maxForumCount": 42, "allowAnonymous": true })),
    )
    .await;
    assert_success(&raw, "bbs POST");
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT enabled, max_forum_count, allow_anonymous FROM x_bbs_assemble_control_config WHERE id = $1",
            &[&BBS_CONFIG_ID],
        )
        .await
        .expect("read bbs config failed");
    assert!(row.get::<_, bool>("enabled"));
    assert_eq!(row.get::<_, i32>("max_forum_count"), 42);
    assert!(row.get::<_, bool>("allow_anonymous"));
}

async fn case_bbs_put(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::PUT,
        &format!("{base}/api/bbs/assemble/control/update/control/config"),
        Some(json!({ "enabled": false, "maxForumCount": 43, "allowAnonymous": false })),
    )
    .await;
    assert_success(&raw, "bbs PUT");
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT enabled, max_forum_count, allow_anonymous FROM x_bbs_assemble_control_config WHERE id = $1",
            &[&BBS_CONFIG_ID],
        )
        .await
        .expect("read bbs config failed");
    assert!(!row.get::<_, bool>("enabled"));
    assert_eq!(row.get::<_, i32>("max_forum_count"), 43);
    assert!(!row.get::<_, bool>("allow_anonymous"));
}

async fn case_general_post(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/general/assemble/control/status/update"),
        Some(json!({ "maintenanceMode": true, "allowRegistration": false })),
    )
    .await;
    assert_success(&raw, "general POST");
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT maintenance_mode, allow_registration FROM x_general_assemble_control_config WHERE id = $1",
            &[&GENERAL_CONFIG_ID],
        )
        .await
        .expect("read general config failed");
    assert_eq!(row.get::<_, String>("maintenance_mode"), "true");
    assert_eq!(row.get::<_, String>("allow_registration"), "false");
}

async fn case_general_put(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::PUT,
        &format!("{base}/api/general/assemble/control/status/update"),
        Some(json!({ "maintenanceMode": false, "allowRegistration": true })),
    )
    .await;
    assert_success(&raw, "general PUT");
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT maintenance_mode, allow_registration FROM x_general_assemble_control_config WHERE id = $1",
            &[&GENERAL_CONFIG_ID],
        )
        .await
        .expect("read general config failed");
    assert_eq!(row.get::<_, String>("maintenance_mode"), "false");
    assert_eq!(row.get::<_, String>("allow_registration"), "true");
}

/// 写路径落文本后，读路径必须用 `::bool` 强转，否则 `Option<bool>` 解码 text 会 panic → 500。
async fn case_general_read_after_write(client: &Client, base: &str, auth: &str) {
    let raw = send(
        client,
        auth,
        reqwest::Method::GET,
        &format!("{base}/api/general/assemble/control/status"),
        None,
    )
    .await;
    let (status, body) = &raw;
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "读路径应 200，body={body}"
    );
    let v: Value = serde_json::from_str(body).expect("body must be JSON");
    assert_eq!(v["type"], "success", "body={body}");
    assert_eq!(
        v["data"]["maintenanceMode"], false,
        "读路径须解出 PUT 写入的 false，body={body}"
    );
    assert_eq!(
        v["data"]["allowRegistration"], true,
        "读路径须解出 PUT 写入的 true，body={body}"
    );
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_bbs_assemble_control_config", &[])
        .await;
    let _ = db
        .execute("DELETE FROM x_general_assemble_control_config", &[])
        .await;
}

fn assert_success((status, raw): &(reqwest::StatusCode, String), label: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "{label} 应为 200（旧实现按错误类型绑定 → WrongType 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
}

async fn send(
    client: &Client,
    auth: &str,
    method: reqwest::Method,
    url: &str,
    body: Option<Value>,
) -> (reqwest::StatusCode, String) {
    let mut builder = client.request(method, url).header("Authorization", auth);
    if let Some(b) = body {
        builder = builder.json(&b);
    }
    let resp = builder.send().await.expect("request failed");
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
