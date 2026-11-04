// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 11：`E42601`（`document_filter_paging_manager` 的 tail 被塞进 WHERE）。
//!
//! `document_filter_paging_manager_u3` 传给 `list_from_table_filtered_legacy` 的
//! where_clause 是**纯 tail**（`ORDER BY create_time DESC LIMIT 200`）。旧拆分只认
//! 带前导空格的 `" ORDER BY "`，漏匹配后数据查询拼成
//! `SELECT * FROM x_cms_data_document WHERE ORDER BY ...` → 语法错误 500。
//!
//! 同时回归 `document_filter_paging_u3`（filter + tail 的常规形态），确保拆分改动
//! 没有把原本正常的调用打坏。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const DOC_ID: &str = "it-doc-filter";

pub async fn document_filter_paths_flow(pool: Arc<TestPool>) {
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

    // 纯 tail（旧实现 500 E42601）：manager 过滤
    let raw = send(
        &client,
        &auth,
        reqwest::Method::POST,
        &format!("{base}/api/document/filter/list/1/size/50/manager"),
    )
    .await;
    assert_contains_doc(&raw, "document filter manager");

    // filter + tail（旧实现本来就正常）：常规分页
    let raw = send(
        &client,
        &auth,
        reqwest::Method::PUT,
        &format!("{base}/api/document/filter/list/1/size/50"),
    )
    .await;
    assert_contains_doc(&raw, "document filter paging");

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_cms_data_document WHERE id = $1", &[&DOC_ID])
        .await;
    db.execute(
        "INSERT INTO x_cms_data_document (id, title, content, author_id, status, creator) \
         VALUES ($1, 'O24-3 filter doc', '', 'it-admin', 'published', 'it-admin')",
        &[&DOC_ID],
    )
    .await
    .expect("seed document failed");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_cms_data_document WHERE id = $1", &[&DOC_ID])
        .await;
}

fn assert_contains_doc((status, raw): &(reqwest::StatusCode, String), label: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "{label} 应为 200（纯 tail 拼进 WHERE → E42601 语法错误 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    let data = body["data"]
        .as_array()
        .unwrap_or_else(|| panic!("data 不是数组: {raw}"));
    let hit = data
        .iter()
        .any(|row| row.get("id").and_then(Value::as_str) == Some(DOC_ID));
    assert!(hit, "{label} 必须返回种子文档 {DOC_ID}，body={raw}");
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
        .json(&serde_json::json!({}))
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
