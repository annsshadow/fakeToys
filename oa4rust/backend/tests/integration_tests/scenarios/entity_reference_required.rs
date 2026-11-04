// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 6：`E23502` 实体引用必填（hotpic 的 `application`/`info_id`、
//! portal page 的 `portal_id`）。裁定口径：实体引用优先从请求上下文（路径/body/
//! 父资源）推导；推导不出来一律 400，**不许编造合法 id**。
//!
//! 信息来源证据：
//! - hotpic：body 字段 `application` / `infoId`（`HotpicRequest`，同 crate 既有契约）；
//!   前端 `HotpicApp.vue` 现在只发 `{title}` → 缺参，断言 400。
//! - portal page：body 字段 `portalId`（与同 crate `create_widget` 的取参口径一致）；
//!   前端 `PortalPageDesignerApp.vue` 只发 `{name,category,content}` → 缺参，断言 400。
//!
//! 本用例钉住：
//! 1. 缺实体引用 → 400（`ExceptionBadRequest`），且不落库；
//! 2. 带实体引用 → 200，回库 `application`/`info_id`、`portal_id` 指向正确父实体；
//! 3. GET 变体（`/api/hotpic_assemble_control/create/hotpic`）同样 400。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const HOTPIC_APP: &str = "it-hotpic-app";
const HOTPIC_INFO: &str = "it-hotpic-info";
const PORTAL_ID: &str = "it-portal-page-parent";

pub async fn entity_reference_required_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    case_hotpic_create_missing(&client, &base, &auth).await;
    case_hotpic_create_with_parent(&client, &base, &auth, &pool).await;
    case_hotpic_config_missing(&client, &base, &auth).await;
    case_hotpic_config_with_parent(&client, &base, &auth, &pool).await;
    case_hotpic_get_create_missing(&client, &base, &auth).await;
    case_portal_page_missing(&client, &base, &auth).await;
    case_portal_page_with_parent(&client, &base, &auth, &pool).await;

    server.abort();
    let _ = server.await;
}

async fn case_hotpic_create_missing(client: &Client, base: &str, auth: &str) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/hotpic/create/hotpic"),
        Some(json!({ "title": "it-no-parent" })),
    )
    .await;
    assert_bad_request(&raw, "application/infoId");
}

async fn case_hotpic_create_with_parent(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/hotpic/create/hotpic"),
        Some(json!({
            "title": "it-with-parent",
            "application": HOTPIC_APP,
            "infoId": HOTPIC_INFO,
        })),
    )
    .await;
    assert_success(&raw, "hotpic create");
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    let id = body["data"]["id"].as_str().expect("data.id").to_string();
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT application, info_id FROM x_hotpic WHERE id = $1",
            &[&id],
        )
        .await
        .expect("read hotpic failed");
    assert_eq!(
        row.get::<_, String>("application"),
        HOTPIC_APP,
        "落库 application 必须指向请求上下文提供的父实体"
    );
    assert_eq!(
        row.get::<_, String>("info_id"),
        HOTPIC_INFO,
        "落库 info_id 必须指向请求上下文提供的父实体"
    );
    db.execute("DELETE FROM x_hotpic WHERE id = $1", &[&id])
        .await
        .expect("cleanup hotpic failed");
}

async fn case_hotpic_config_missing(client: &Client, base: &str, auth: &str) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/hotpic/assemble/control/update/control/config"),
        Some(json!({})),
    )
    .await;
    assert_bad_request(&raw, "application/infoId");
}

async fn case_hotpic_config_with_parent(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/hotpic/assemble/control/update/control/config"),
        Some(json!({
            "name": "it-config",
            "application": HOTPIC_APP,
            "infoId": HOTPIC_INFO,
        })),
    )
    .await;
    assert_success(&raw, "hotpic config");
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["data"]["updated"], true, "body={}", raw.1);
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT application, info_id FROM x_hotpic WHERE title = 'it-config' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .expect("read hotpic config row failed");
    assert_eq!(row.get::<_, String>("application"), HOTPIC_APP);
    assert_eq!(row.get::<_, String>("info_id"), HOTPIC_INFO);
    db.execute("DELETE FROM x_hotpic WHERE title = 'it-config'", &[])
        .await
        .expect("cleanup hotpic config failed");
}

async fn case_hotpic_get_create_missing(client: &Client, base: &str, auth: &str) {
    let raw = send(
        client,
        auth,
        reqwest::Method::GET,
        &format!("{base}/api/hotpic_assemble_control/create/hotpic"),
        Some(json!({})),
    )
    .await;
    assert_bad_request(&raw, "application/infoId");
}

async fn case_portal_page_missing(client: &Client, base: &str, auth: &str) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/portal/assemble/designer/page/create"),
        Some(json!({ "name": "it-page-no-parent" })),
    )
    .await;
    assert_bad_request(&raw, "portalId");
}

async fn case_portal_page_with_parent(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/portal/assemble/designer/page"),
        Some(json!({
            "name": "it-page-with-parent",
            "category": "default",
            "content": {},
            "portalId": PORTAL_ID,
        })),
    )
    .await;
    assert_success(&raw, "portal page create");
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    let id = body["data"]["id"].as_str().expect("data.id").to_string();
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT portal_id, status FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .expect("read portal page failed");
    assert_eq!(
        row.get::<_, String>("portal_id"),
        PORTAL_ID,
        "落库 portal_id 必须指向请求上下文提供的父实体"
    );
    assert_eq!(
        row.get::<_, String>("status"),
        "active",
        "status 沿用同表既有写方默认值"
    );
    db.execute("DELETE FROM x_portal_page WHERE id = $1", &[&id])
        .await
        .expect("cleanup portal page failed");
}

fn assert_bad_request((status, raw): &(reqwest::StatusCode, String), param: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::BAD_REQUEST,
        "缺实体引用应为 400（旧实现直接 INSERT → E23502 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(raw).expect("body must be JSON");
    assert_eq!(body["type"], "error", "body={raw}");
    assert!(
        raw.contains(param),
        "400 文案必须点名缺失参数 {param}，body={raw}"
    );
}

fn assert_success((status, raw): &(reqwest::StatusCode, String), label: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "{label} 应为 200，实际 {status}，body={raw}"
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
