// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 4：INSERT 未写必填主键/规范列的遗留给定族（3 条 E23502）。
//!
//! `information_schema` 现场确认的 NOT NULL 列：
//!
//! - `x_console_message.id`（旧 SQL 只写遗留列 `xid`）
//! - `x_cms_viewrecord.id`（旧 SQL 完全没写主键）
//! - `x_msg_message.id/title/type/consumer/is_read`（旧 SQL 只写 `x*` 遗留列）
//!
//! 三条路由都在 `execute` 处 E23502 → 500。修法是补规范列（主键由服务端 uuid 生成），
//! 遗留列继续同值写入，避免破坏读旧列的一方。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const CONSOLE_TOKEN: &str = "it-console-token";
const DOC_ID: &str = "it-cms-doc";
const PERSON_ID: &str = "it-cms-person";

pub async fn insert_required_id_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    case_console_send_message(&client, &base, &auth, &pool).await;
    case_message_custom_create(&client, &base, &auth, &pool).await;
    case_document_view_record(&client, &base, &auth, &pool).await;

    server.abort();
    let _ = server.await;
}

async fn case_console_send_message(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_console_message WHERE xtoken = $1",
        &[&CONSOLE_TOKEN],
    )
    .await
    .expect("delete console message failed");

    let raw = post(
        client,
        auth,
        &format!("{base}/api/console/send/message"),
        &json!({"token": CONSOLE_TOKEN, "message": "it-console-message"}),
    )
    .await;
    assert_eq!(raw.0, reqwest::StatusCode::OK, "body={}", raw.1);
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={}", raw.1);
    assert_eq!(body["data"]["sent"], true, "body={}", raw.1);
    assert_eq!(body["data"]["token"], CONSOLE_TOKEN, "body={}", raw.1);

    let row = db
        .query_one(
            "SELECT id, xmessage FROM x_console_message WHERE xtoken = $1",
            &[&CONSOLE_TOKEN],
        )
        .await
        .expect("read console message failed");
    assert!(
        !row.get::<_, String>("id").is_empty(),
        "x_console_message.id 必须非空（旧实现 E23502）"
    );
    assert_eq!(
        row.get::<_, Option<String>>("xmessage").as_deref(),
        Some("it-console-message")
    );
}

async fn case_message_custom_create(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let payload = json!({
        "title": "it-message-title",
        "body": "it-message-body",
        "type": "it-message-type",
        "consumer": "it-consumer",
    });
    let raw = post(
        client,
        auth,
        &format!("{base}/api/message/custom/create"),
        &payload,
    )
    .await;
    assert_eq!(raw.0, reqwest::StatusCode::OK, "body={}", raw.1);
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={}", raw.1);
    assert_eq!(body["data"]["created"], true, "body={}", raw.1);
    let id = body["data"]["id"].as_str().expect("id must be string");
    assert!(!id.is_empty(), "body={}", raw.1);

    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT title, type, consumer, is_read, xid, xconsumed \
             FROM x_msg_message WHERE id = $1",
            &[&id],
        )
        .await
        .expect("read custom message failed");
    assert_eq!(row.get::<_, String>("title"), "it-message-title");
    assert_eq!(row.get::<_, String>("type"), "it-message-type");
    assert_eq!(row.get::<_, String>("consumer"), "it-consumer");
    assert!(!row.get::<_, bool>("is_read"));
    assert_eq!(row.get::<_, Option<String>>("xid").as_deref(), Some(id));
    assert!(!row.get::<_, bool>("xconsumed"));
}

async fn case_document_view_record(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_cms_viewrecord WHERE doc_id = $1 AND person_id = $2",
        &[&DOC_ID, &PERSON_ID],
    )
    .await
    .expect("delete view record failed");

    let raw = post(
        client,
        auth,
        &format!("{base}/api/document/cipher/{DOC_ID}/persist/view/record"),
        &json!({"personId": PERSON_ID}),
    )
    .await;
    assert_eq!(raw.0, reqwest::StatusCode::OK, "body={}", raw.1);
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={}", raw.1);
    assert_eq!(body["data"]["docId"], DOC_ID, "body={}", raw.1);
    assert_eq!(body["data"]["personId"], PERSON_ID, "body={}", raw.1);

    let row = db
        .query_one(
            "SELECT id FROM x_cms_viewrecord WHERE doc_id = $1 AND person_id = $2",
            &[&DOC_ID, &PERSON_ID],
        )
        .await
        .expect("read view record failed");
    assert!(
        !row.get::<_, String>("id").is_empty(),
        "x_cms_viewrecord.id 必须非空（旧实现 E23502）"
    );
}

async fn post(
    client: &Client,
    auth: &str,
    url: &str,
    body: &Value,
) -> (reqwest::StatusCode, String) {
    let resp = client
        .post(url)
        .header("Authorization", auth)
        .json(body)
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
