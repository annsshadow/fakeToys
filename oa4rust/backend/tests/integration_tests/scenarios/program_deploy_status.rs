// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 16：`column:status`（`x_program_deploy.status` 可空，代码按非 Option 读）。
//!
//! `market_flag_installed_version` 与 `deploy_id` 都用 `row.get::<_, String>("status")`；
//! 列的 `status` 是 **nullable text**（information_schema 现场确认），NULL 时
//! tokio-postgres 解包失败（`error deserializing column status`）→ 500。
//! 改为 `Option<String>` + 空串兜底。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const DEPLOY_ID: &str = "it-deploy-status";

pub async fn program_deploy_status_flow(pool: Arc<TestPool>) {
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

    // 1）installed/version（基线里的那条）
    let body = get_json(
        &client,
        &auth,
        &format!("{base}/api/program_center/market/it-market/installed/version"),
    )
    .await;
    assert_eq!(body["data"]["id"], DEPLOY_ID, "body={body}");
    assert_eq!(
        body["data"]["status"], "",
        "status 为 NULL 时应输出空串而不是 500，body={body}"
    );

    // 2）deploy/{id}（同根因的第二处，顺带修复 + 回归）
    let body = get_json(
        &client,
        &auth,
        &format!("{base}/api/program_center/deploy/{DEPLOY_ID}"),
    )
    .await;
    assert_eq!(body["data"]["id"], DEPLOY_ID, "body={body}");
    assert_eq!(body["data"]["status"], "", "body={body}");

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_program_deploy WHERE id = $1", &[&DEPLOY_ID])
        .await;
    db.execute(
        "INSERT INTO x_program_deploy (id, name, version, status, creator, create_time) \
         VALUES ($1, 'it-deploy', '1.0.0', NULL, 'it-admin', '2026-01-01 10:00:00')",
        &[&DEPLOY_ID],
    )
    .await
    .expect("seed x_program_deploy failed");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_program_deploy WHERE id = $1", &[&DEPLOY_ID])
        .await;
}

async fn get_json(client: &Client, auth: &str, url: &str) -> Value {
    let resp = client
        .get(url)
        .header("Authorization", auth)
        .send()
        .await
        .expect("request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "应 200（status 可空却按非 Option 读 → 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    body
}

async fn pg_client(pool: &Arc<TestPool>) -> deadpool_postgres::Object {
    pool.as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client")
}
