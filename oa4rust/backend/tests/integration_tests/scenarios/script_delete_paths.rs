// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 10：`E42804`（text 字面量写 timestamp 列 / COALESCE 类型不匹配，3 条）。
//!
//! - `x_portal_script.deleted_at` 与 `PP_E_SCRIPT.deleted_at` 实列是
//!   `timestamp without time zone`，旧 DELETE 写 `to_char(...)`（text）→ datatype mismatch；
//! - `x_attachment.create_time` 是 timestamp，旧 mergeitem 用 `COALESCE(create_time, id)`
//!   （timestamp vs varchar）→ datatype mismatch，改为 `COALESCE(create_time, TIMESTAMP 'epoch')`。
//!
//! 每条都断言「写入后立即读回同一值」。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const PORTAL_SCRIPT_ID: &str = "it-pscript";
const PP_SCRIPT_ID: &str = "it-pscript-pp";
const ATTACH_OLD: &str = "it-attach-old";
const ATTACH_NEW: &str = "it-attach-new";

pub async fn script_delete_paths_flow(pool: Arc<TestPool>) {
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

    case_portal_delete(&client, &base, &auth, &pool).await;
    case_pp_delete(&client, &base, &auth, &pool).await;
    case_merge_item(&client, &base, &auth, &pool).await;

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    cleanup_with(&db).await;
    db.execute(
        "INSERT INTO x_portal_script (id, name, flag, category, content, creator, create_time, update_time) \
         VALUES ($1, 'it-script', 'it-script', 'x', 'body', 'it', '2026-01-01 10:00:00', '2026-01-01 10:00:00')",
        &[&PORTAL_SCRIPT_ID],
    )
    .await
    .expect("seed x_portal_script failed");
    db.execute(
        "INSERT INTO pp_e_script (id, xid, xname, xcode) VALUES ($1, $1, 'it-script', 'body')",
        &[&PP_SCRIPT_ID],
    )
    .await
    .expect("seed pp_e_script failed");
    for (id, created) in [
        (ATTACH_OLD, "'2026-01-01 10:00:00'"),
        (ATTACH_NEW, "'2026-01-02 10:00:00'"),
    ] {
        // create_time 是 timestamp 列，字面量必须显式进 SQL（绑定 &str 会 ToSql WrongType）。
        db.execute(
            &format!(
                "INSERT INTO x_attachment (id, work_id, workcompleted_id, name, content, create_time) \
                 VALUES ($1, 'it-work', 'it-wc', 'same.txt', 'c', {created})"
            ),
            &[&id],
        )
        .await
        .expect("seed x_attachment failed");
    }
}

async fn case_portal_delete(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::DELETE,
        &format!("{base}/api/portal/assemble/designer/script/{PORTAL_SCRIPT_ID}"),
    )
    .await;
    assert_success(&raw, "portal script DELETE");
    let db = pg_client(pool).await;
    let deleted: Option<String> = db
        .query_one(
            "SELECT deleted_at::text FROM x_portal_script WHERE id = $1",
            &[&PORTAL_SCRIPT_ID],
        )
        .await
        .expect("write-then-read failed")
        .get(0);
    assert!(deleted.is_some(), "软删后 deleted_at 必须非空");
}

async fn case_pp_delete(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::DELETE,
        &format!("{base}/api/processplatform/assemble/designer/script/{PP_SCRIPT_ID}"),
    )
    .await;
    assert_success(&raw, "processplatform script DELETE");
    let db = pg_client(pool).await;
    let deleted: Option<String> = db
        .query_one(
            "SELECT deleted_at::text FROM pp_e_script WHERE xid = $1",
            &[&PP_SCRIPT_ID],
        )
        .await
        .expect("write-then-read failed")
        .get(0);
    assert!(deleted.is_some(), "软删后 deleted_at 必须非空");
}

async fn case_merge_item(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/processplatform/service/processing/touch/mergeitem"),
    )
    .await;
    assert_success(&raw, "touch/mergeitem");
    let body: Value = serde_json::from_str(&raw.1).expect("body must be JSON");
    assert_eq!(body["data"]["merged"], 1, "去重后应删 1 行，body={}", raw.1);

    let db = pg_client(pool).await;
    let rows = db
        .query(
            "SELECT id FROM x_attachment WHERE work_id = 'it-work' AND deleted_at IS NULL",
            &[],
        )
        .await
        .expect("write-then-read failed");
    assert_eq!(
        rows.len(),
        1,
        "同一 (name, work_id, workcompleted_id) 只能留 1 行"
    );
    let kept: String = rows[0].get(0);
    assert_eq!(kept, ATTACH_OLD, "保留最早创建的那行（既有语义）");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    cleanup_with(&db).await;
}

async fn cleanup_with(db: &deadpool_postgres::Object) {
    let _ = db
        .execute(
            "DELETE FROM x_portal_script WHERE id = $1",
            &[&PORTAL_SCRIPT_ID],
        )
        .await;
    let _ = db
        .execute("DELETE FROM pp_e_script WHERE xid = $1", &[&PP_SCRIPT_ID])
        .await;
    let _ = db
        .execute("DELETE FROM x_attachment WHERE work_id = 'it-work'", &[])
        .await;
}

fn assert_success((status, raw): &(reqwest::StatusCode, String), label: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "{label} 应为 200（旧实现 text→timestamp 或 COALESCE 类型不匹配 → E42804 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
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
