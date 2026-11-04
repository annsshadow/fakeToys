// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 3：查询设计器 permission 族的 `$1` 复用冲突（4 条基线）。
//!
//! 共同根因：`set_permission_generic` 把同一个 `predicate`（`flag = $1 OR id = $1`）
//! 既用于守卫 SELECT（只绑 flag），又用于 UPDATE（绑 `[permission, flag]`）。
//! UPDATE 的 `$1` 已被 permission 占用，谓词里的 `$1` 实际指向 permission，
//! 于是 2 个绑定只对应 1 个占位符 → `Parameters` 失配 → 500。
//!
//! 修法：仅在 UPDATE 里把谓词的 `$1` 改为 `$2`（`replace`，编译期常量谓词，无注入面）。
//! 四条基线：
//!
//! 1. `POST /api/query/assemble/designer/query/{flag}/permission`
//! 2. `POST /api/query/assemble/designer/stat/{id}/permission`
//! 3. `POST /api/query/assemble/designer/table/{flag}/permission`
//! 4. `POST /api/query/assemble/designer/view/{id}/permission`
//!
//! 种子行 owner 留空 → 走 `require_admin_gate`，用 admin token 通过。
#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const QUERY_FLAG: &str = "it-perm-query-flag";
const QUERY_ID: &str = "it-perm-query-id";
const STAT_ID: &str = "it-perm-stat-id";
const TABLE_FLAG: &str = "it-perm-table-flag";
const TABLE_ID: &str = "it-perm-table-id";
const VIEW_ID: &str = "it-perm-view-id";
const VIEW_FLAG: &str = "it-perm-view-flag";

pub async fn query_permission_binding_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    case_query(&client, &base, &auth, &pool).await;
    case_stat(&client, &base, &auth, &pool).await;
    case_table(&client, &base, &auth, &pool).await;
    case_view(&client, &base, &auth, &pool).await;

    server.abort();
    let _ = server.await;
}

async fn case_query(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_query_design WHERE id = $1 OR flag = $2",
        &[&QUERY_ID, &QUERY_FLAG],
    )
    .await
    .expect("delete x_query_design failed");
    db.execute(
        "INSERT INTO x_query_design (id, flag, permission, update_time) VALUES ($1, $2, NULL, NULL)",
        &[&QUERY_ID, &QUERY_FLAG],
    )
    .await
    .expect("insert x_query_design failed");

    let body = json!({"permissionList": [{"type": "person", "name": "it-person"}]});
    let raw = post_permission(
        client,
        auth,
        &format!("{base}/api/query/assemble/designer/query/{QUERY_FLAG}/permission"),
        &body,
    )
    .await;
    assert_success(&raw, QUERY_FLAG);
    assert_permission_stored(&db, "x_query_design", "flag", QUERY_FLAG, &body).await;
}

async fn case_stat(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute("DELETE FROM x_query_stat WHERE id = $1", &[&STAT_ID])
        .await
        .expect("delete x_query_stat failed");
    db.execute(
        "INSERT INTO x_query_stat (id, permission, update_time) VALUES ($1, NULL, NULL)",
        &[&STAT_ID],
    )
    .await
    .expect("insert x_query_stat failed");

    let body = json!({"permissionList": [{"type": "unit", "name": "it-unit"}]});
    let raw = post_permission(
        client,
        auth,
        &format!("{base}/api/query/assemble/designer/stat/{STAT_ID}/permission"),
        &body,
    )
    .await;
    assert_success(&raw, STAT_ID);
    assert_permission_stored(&db, "x_query_stat", "id", STAT_ID, &body).await;
}

async fn case_table(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_query_table WHERE id = $1 OR table_flag = $2",
        &[&TABLE_ID, &TABLE_FLAG],
    )
    .await
    .expect("delete x_query_table failed");
    db.execute(
        "INSERT INTO x_query_table (id, table_flag, permission, update_time) VALUES ($1, $2, NULL, NULL)",
        &[&TABLE_ID, &TABLE_FLAG],
    )
    .await
    .expect("insert x_query_table failed");

    let body = json!({"permissionList": [{"type": "group", "name": "it-group"}]});
    let raw = post_permission(
        client,
        auth,
        &format!("{base}/api/query/assemble/designer/table/{TABLE_FLAG}/permission"),
        &body,
    )
    .await;
    assert_success(&raw, TABLE_FLAG);
    assert_permission_stored(&db, "x_query_table", "table_flag", TABLE_FLAG, &body).await;
}

async fn case_view(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_query_view WHERE id = $1 OR view_flag = $2",
        &[&VIEW_ID, &VIEW_FLAG],
    )
    .await
    .expect("delete x_query_view failed");
    db.execute(
        "INSERT INTO x_query_view (id, name, query_type, view_flag, permission, update_time) \
         VALUES ($1, 'it view', 'view', $2, NULL, NULL)",
        &[&VIEW_ID, &VIEW_FLAG],
    )
    .await
    .expect("insert x_query_view failed");

    let body = json!({"permissionList": [{"type": "person", "name": "it-viewer"}]});
    let raw = post_permission(
        client,
        auth,
        &format!("{base}/api/query/assemble/designer/view/{VIEW_ID}/permission"),
        &body,
    )
    .await;
    assert_success(&raw, VIEW_ID);
    assert_permission_stored(&db, "x_query_view", "id", VIEW_ID, &body).await;
}

async fn post_permission(
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
        .expect("permission request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("permission body");
    (status, raw)
}

fn assert_success((status, raw): &(reqwest::StatusCode, String), expected_id: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "权限写入应为 200（旧实现谓词 $1 与 permission 绑定冲突 → 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(raw).expect("permission body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["id"], expected_id, "body={raw}");
    assert_eq!(body["data"]["permissionSet"], true, "body={raw}");
}

async fn assert_permission_stored(
    db: &deadpool_postgres::Object,
    table: &str,
    key_column: &str,
    key: &str,
    body: &Value,
) {
    // 表名/列名是测试内编译期常量，非用户输入。
    // update_time 在不同表是 TEXT / TIMESTAMP，统一 ::text 读，避免解码类型分叉。
    let sql = format!(
        "SELECT permission, update_time::text AS update_time FROM {table} WHERE {key_column} = $1"
    );
    let row = db
        .query_one(sql.as_str(), &[&key])
        .await
        .expect("read permission row failed");
    let expected = serde_json::to_string(body.get("permissionList").expect("permissionList"))
        .expect("serialize expected permission");
    assert_eq!(
        row.get::<_, Option<String>>("permission"),
        Some(expected),
        "{table}.{key_column}={key} 的 permission 必须落库"
    );
    assert!(
        row.get::<_, Option<String>>("update_time")
            .map(|s| !s.is_empty())
            .unwrap_or(false),
        "{table} 的 update_time 必须被写入"
    );
}

async fn pg_client(pool: &Arc<TestPool>) -> deadpool_postgres::Object {
    pool.as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client")
}
