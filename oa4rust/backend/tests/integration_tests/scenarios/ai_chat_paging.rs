// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 13：`E42P18`（ai chat 分页的占位符编号错位）。
//!
//! 绑定顺序是 `$1=clue_id, $2=offset, $3=size`，旧 SQL 却写成 `LIMIT $4 OFFSET $3`：
//! `$2` 从未被引用 → PG 无法推断其类型（`E42P18`），`$4` 也不存在。
//! 正确写法与同文件 `x_ai_conversation` 分页一致：`LIMIT $3 OFFSET $2`。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const CLUE_ID: &str = "it-clue-paging";
const CHAT_OLD: &str = "it-chat-old";
const CHAT_NEW: &str = "it-chat-new";

pub async fn ai_chat_paging_flow(pool: Arc<TestPool>) {
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

    // page=1 size=2：两条都在，按 create_time 倒序。
    let body = get_json(
        &client,
        &auth,
        &format!("{base}/api/ai_assemble_control/chat/list/completion/{CLUE_ID}/paging/1/size/2"),
    )
    .await;
    let data = body["data"].as_array().expect("data 应为数组");
    assert_eq!(data.len(), 2, "body={body}");
    assert_eq!(data[0]["id"], CHAT_NEW, "第一页第一行应是最新一条");
    assert_eq!(data[1]["id"], CHAT_OLD, "body={body}");

    // page=2 size=1：分页窗口真的生效（offset=1）。
    let body = get_json(
        &client,
        &auth,
        &format!("{base}/api/ai_assemble_control/chat/list/completion/{CLUE_ID}/paging/2/size/1"),
    )
    .await;
    let data = body["data"].as_array().expect("data 应为数组");
    assert_eq!(data.len(), 1, "body={body}");
    assert_eq!(data[0]["id"], CHAT_OLD, "第二页应是较旧的一条");

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute(
            "DELETE FROM x_ai_chat WHERE conversation_id = $1",
            &[&CLUE_ID],
        )
        .await;
    db.execute(
        "INSERT INTO x_ai_chat (id, conversation_id, role, content, create_time) \
         VALUES ($1, $2, 'user', 'old', '2026-01-01 10:00:00')",
        &[&CHAT_OLD, &CLUE_ID],
    )
    .await
    .expect("seed old chat failed");
    db.execute(
        "INSERT INTO x_ai_chat (id, conversation_id, role, content, create_time) \
         VALUES ($1, $2, 'assistant', 'new', '2026-01-02 10:00:00')",
        &[&CHAT_NEW, &CLUE_ID],
    )
    .await
    .expect("seed new chat failed");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute(
            "DELETE FROM x_ai_chat WHERE conversation_id = $1",
            &[&CLUE_ID],
        )
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
        "应 200（旧实现 $2 未引用 → E42P18 500），实际 {status}，body={raw}"
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
