// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 1：`POST /api/portal/assemble/surface/publish/{id}` 的行为级用例。
//!
//! 该路由的危险点：`UPDATE ... SET published = true` 先落库，随后 `SELECT`
//! 把 BOOLEAN 列 `published` 按 `Option<String>` 解码 → ColumnDecode panic → 500。
//! 用户看到「发布失败」，数据却已经写进去了。本用例同时钉住三件事：
//!
//! 1. 有行 → **200**，body 的 `data.published` 是 **bool true**（不是字符串）；
//! 2. 落库事实：`x_portal_surface.published = true`、`published_at` 非空；
//! 3. 无此 id → 200 `type=error`（不写库，不制造幽灵行）。
//!
//! 破坏实验锚点：把 `row.get::<_, Option<bool>>("published")` 改回
//! `Option<String>`，第 1 条必须红（500 / panic `error retrieving column published`）。
#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const SURFACE_ID: &str = "it-portal-surface-publish";

pub async fn portal_surface_publish_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    delete_surface(&pool, SURFACE_ID).await;
    insert_surface(&pool, SURFACE_ID).await;

    case_publish_existing_returns_200_and_persists(&client, &base, &auth, &pool).await;
    case_publish_missing_does_not_write(&client, &base, &auth, &pool).await;

    server.abort();
    let _ = server.await;
}

/// 有行 → 200 + body 具体值 + 数据库真实落库。
async fn case_publish_existing_returns_200_and_persists(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let resp = client
        .post(format!(
            "{base}/api/portal/assemble/surface/publish/{SURFACE_ID}"
        ))
        .header("Authorization", auth)
        .send()
        .await
        .expect("publish request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("publish body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "发布已存在的表面应为 200（当前是写库成功但解码失败 → 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("publish body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["id"], SURFACE_ID, "body={raw}");
    // 真实列类型是 BOOLEAN（information_schema 现场确认），响应也必须是 bool。
    assert_eq!(
        body["data"]["published"], true,
        "data.published 必须是 bool true（不是字符串），body={raw}"
    );
    assert!(
        body["data"]["publishedAt"]
            .as_str()
            .map(|s| !s.is_empty())
            .unwrap_or(false),
        "data.publishedAt 必须是非空字符串，body={raw}"
    );

    // 落库事实：不能出现「响应 500 但数据已改」或「响应 200 但没写」。
    let client_db = pool
        .as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client");
    let row = client_db
        .query_one(
            "SELECT published, published_at FROM x_portal_surface WHERE id = $1",
            &[&SURFACE_ID],
        )
        .await
        .expect("read surface row failed");
    assert_eq!(
        row.get::<_, Option<bool>>("published"),
        Some(true),
        "发布后 x_portal_surface.published 必须是 true"
    );
    assert!(
        row.get::<_, Option<String>>("published_at")
            .map(|s| !s.is_empty())
            .unwrap_or(false),
        "发布后 published_at 必须非空"
    );
}

/// id 不存在 → 200 `type=error`，且不写库。
async fn case_publish_missing_does_not_write(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let missing = "it-portal-surface-missing";
    delete_surface(pool, missing).await;

    let resp = client
        .post(format!(
            "{base}/api/portal/assemble/surface/publish/{missing}"
        ))
        .header("Authorization", auth)
        .send()
        .await
        .expect("publish missing request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("publish body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "不存在的 id 应走 200 error 分支，实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("publish body must be JSON");
    assert_eq!(body["type"], "error", "body={raw}");
    assert_eq!(body["message"], "surface not found", "body={raw}");

    let client_db = pool
        .as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client");
    let count = client_db
        .query_one(
            "SELECT COUNT(*) FROM x_portal_surface WHERE id = $1",
            &[&missing],
        )
        .await
        .expect("count surface row failed")
        .get::<_, i64>(0);
    assert_eq!(count, 0, "发布不存在的 id 不得插入幽灵行");
}

async fn insert_surface(pool: &Arc<TestPool>, id: &str) {
    let client = pool
        .as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client");
    client
        .execute(
            "INSERT INTO x_portal_surface (id, portal_id, name, published, create_time, update_time) \
             VALUES ($1, 'it-portal', 'it-publish-surface', false, '2026-11-05 00:00:00', '2026-11-05 00:00:00')",
            &[&id],
        )
        .await
        .expect("insert surface row failed");
}

async fn delete_surface(pool: &Arc<TestPool>, id: &str) {
    let client = pool
        .as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client");
    client
        .execute("DELETE FROM x_portal_surface WHERE id = $1", &[&id])
        .await
        .expect("delete surface row failed");
}
