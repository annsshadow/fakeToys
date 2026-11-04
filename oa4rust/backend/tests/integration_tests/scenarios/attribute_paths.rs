// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 9：`E42703`（`resolve_generic_id` 对无 `name` 列的表拼 `OR name = $1`）。
//!
//! `x_org_person_attribute` / `x_org_unit_attribute` 的实际列是
//! `id / person_id|unit_id / attribute_key / attribute_value / creator / create_time / deleted_at`
//! —— **没有 `name`**。旧实现走通用 `resolve_generic_id` → E42703 undefined column "name"。
//! 这两张表的路径参数本来就是记录 id（GET 变体也按 id 查），改为按 id 精确解析。
//!
//! 每条都断言「写入后立即读回同一值」。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const PERSON_ATTR_ID: &str = "it-attr-person";
const UNIT_ATTR_ID: &str = "it-attr-unit";

pub async fn attribute_paths_flow(pool: Arc<TestPool>) {
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

    case_person_put(&client, &base, &auth, &pool).await;
    case_person_delete(&client, &base, &auth, &pool).await;
    case_unit_put(&client, &base, &auth, &pool).await;
    case_unit_delete(&client, &base, &auth, &pool).await;

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute(
            "DELETE FROM x_org_person_attribute WHERE id = $1",
            &[&PERSON_ATTR_ID],
        )
        .await;
    let _ = db
        .execute(
            "DELETE FROM x_org_unit_attribute WHERE id = $1",
            &[&UNIT_ATTR_ID],
        )
        .await;
    db.execute(
        "INSERT INTO x_org_person_attribute (id, person_id, attribute_key, attribute_value) \
         VALUES ($1, 'it-person', 'it-key', 'old')",
        &[&PERSON_ATTR_ID],
    )
    .await
    .expect("seed person attribute failed");
    db.execute(
        "INSERT INTO x_org_unit_attribute (id, unit_id, attribute_key, attribute_value) \
         VALUES ($1, 'it-unit', 'it-key', 'old')",
        &[&UNIT_ATTR_ID],
    )
    .await
    .expect("seed unit attribute failed");
}

async fn case_person_put(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::PUT,
        &format!("{base}/api/organization/assemble/control/personattribute/{PERSON_ATTR_ID}"),
        Some(json!({ "attributeValue": "new" })),
    )
    .await;
    assert_success(&raw, "personattribute PUT");
    let db = pg_client(pool).await;
    let value = db
        .query_one(
            "SELECT attribute_value FROM x_org_person_attribute WHERE id = $1",
            &[&PERSON_ATTR_ID],
        )
        .await
        .expect("write-then-read failed")
        .get::<_, Option<String>>(0)
        .unwrap_or_default();
    assert_eq!(value, "new", "attribute_value 必须落库为请求值");
}

async fn case_person_delete(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::DELETE,
        &format!("{base}/api/organization/assemble/control/personattribute/{PERSON_ATTR_ID}"),
        None,
    )
    .await;
    assert_success(&raw, "personattribute DELETE");
    let db = pg_client(pool).await;
    let deleted: Option<String> = db
        .query_one(
            "SELECT deleted_at::text FROM x_org_person_attribute WHERE id = $1",
            &[&PERSON_ATTR_ID],
        )
        .await
        .expect("write-then-read failed")
        .get(0);
    assert!(deleted.is_some(), "软删后 deleted_at 必须非空");

    let raw = send(
        client,
        auth,
        reqwest::Method::PUT,
        &format!("{base}/api/organization/assemble/control/personattribute/{PERSON_ATTR_ID}"),
        Some(json!({ "attributeValue": "after-delete" })),
    )
    .await;
    // 该 handler 的「未命中」走 o2server 旧语义：HTTP 200 + `type=error`（与其它
    // organization 路由一致），所以这里断言响应体而不是状态码。
    assert_eq!(
        raw.0,
        reqwest::StatusCode::OK,
        "已软删记录 PUT 状态码走旧语义 200，body={}",
        raw.1
    );
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(
        body["type"], "error",
        "已软删记录不可再被 PUT 命中，body={}",
        raw.1
    );
}

async fn case_unit_put(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::PUT,
        &format!("{base}/api/organization/assemble/control/unitattribute/{UNIT_ATTR_ID}"),
        Some(json!({ "attributeValue": "new-u" })),
    )
    .await;
    assert_success(&raw, "unitattribute PUT");
    let db = pg_client(pool).await;
    let value = db
        .query_one(
            "SELECT attribute_value FROM x_org_unit_attribute WHERE id = $1",
            &[&UNIT_ATTR_ID],
        )
        .await
        .expect("write-then-read failed")
        .get::<_, Option<String>>(0)
        .unwrap_or_default();
    assert_eq!(value, "new-u", "attribute_value 必须落库为请求值");
}

async fn case_unit_delete(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::DELETE,
        &format!("{base}/api/organization/assemble/control/unitattribute/{UNIT_ATTR_ID}"),
        None,
    )
    .await;
    assert_success(&raw, "unitattribute DELETE");
    let db = pg_client(pool).await;
    let deleted: Option<String> = db
        .query_one(
            "SELECT deleted_at::text FROM x_org_unit_attribute WHERE id = $1",
            &[&UNIT_ATTR_ID],
        )
        .await
        .expect("write-then-read failed")
        .get(0);
    assert!(deleted.is_some(), "软删后 deleted_at 必须非空");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute(
            "DELETE FROM x_org_person_attribute WHERE id = $1",
            &[&PERSON_ATTR_ID],
        )
        .await;
    let _ = db
        .execute(
            "DELETE FROM x_org_unit_attribute WHERE id = $1",
            &[&UNIT_ATTR_ID],
        )
        .await;
}

fn assert_success((status, raw): &(reqwest::StatusCode, String), label: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "{label} 应为 200（旧实现走通用解析 OR name = $1 → E42703 500），实际 {status}，body={raw}"
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
