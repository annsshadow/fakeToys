// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 8：`E42P10`（ON CONFLICT 指向不存在的唯一约束）。
//!
//! - attendance `statistic_do`：表 `x_attendance_statistic` 只有 `id` 主键，旧 SQL 却
//!   `ON CONFLICT (person_id, year, month)`，且 INSERT 未写 NOT NULL 的 `id`。
//!   改为：确定性 id + `ON CONFLICT (id)`；`create_time/update_time` 是 text，用 `NOW()::text`。
//! - calendar `update_control_config`：表 `cal_control_config` 只有 `id` 主键，
//!   旧 SQL 却 `ON CONFLICT (config_key)`，且 INSERT 未写 NOT NULL 的 `id`。
//!   改为：UPDATE-then-INSERT（单行幂等），`update_time` 用 `NOW()::text`。
//!
//! 每条都断言「写入后立即读回同一值」。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const PERSON_ID: &str = "it-att-person";
const YEAR: &str = "2026";
const MONTH: &str = "11";

pub async fn upsert_paths_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    case_attendance_post(&client, &base, &auth, &pool).await;
    case_attendance_get_variant(&client, &base, &auth).await;
    case_calendar_config(&client, &base, &auth, &pool).await;

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn case_attendance_post(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/attendance/assemble/control/statistic/do"),
        Some(json!({ "personId": PERSON_ID, "year": YEAR, "month": MONTH })),
    )
    .await;
    assert_success(&raw, "attendance POST 首次");
    let db = pg_client(pool).await;
    let row = db
        .query_one(
            "SELECT id, person_id, year, month, create_time FROM x_attendance_statistic \
             WHERE person_id = $1 AND year = $2 AND month = $3",
            &[&PERSON_ID, &YEAR, &MONTH],
        )
        .await
        .expect("first write must persist a row (write-then-read)");
    assert_eq!(row.get::<_, String>("person_id"), PERSON_ID);
    assert_eq!(row.get::<_, String>("year"), YEAR);
    assert_eq!(row.get::<_, String>("month"), MONTH);
    assert!(
        !row.get::<_, Option<String>>("create_time")
            .unwrap_or_default()
            .is_empty(),
        "create_time 必须落库（text 列）"
    );

    // 第二次同键写入：必须命中 upsert（仍只有一行，update_time 更新）。
    let raw = send(
        client,
        auth,
        reqwest::Method::POST,
        &format!("{base}/api/attendance/assemble/control/statistic/do"),
        Some(json!({ "personId": PERSON_ID, "year": YEAR, "month": MONTH })),
    )
    .await;
    assert_success(&raw, "attendance POST 二次");
    let count: i64 = db
        .query_one(
            "SELECT COUNT(*) FROM x_attendance_statistic WHERE person_id = $1 AND year = $2 AND month = $3",
            &[&PERSON_ID, &YEAR, &MONTH],
        )
        .await
        .expect("count failed")
        .get(0);
    assert_eq!(count, 1, "同 (person_id,year,month) 必须 upsert 为一行");
    let update_time = db
        .query_one(
            "SELECT update_time FROM x_attendance_statistic WHERE person_id = $1 AND year = $2 AND month = $3",
            &[&PERSON_ID, &YEAR, &MONTH],
        )
        .await
        .expect("read update_time failed")
        .get::<_, Option<String>>(0)
        .unwrap_or_default();
    assert!(!update_time.is_empty(), "二次写入必须更新 update_time");
}

async fn case_attendance_get_variant(client: &Client, base: &str, auth: &str) {
    // GET 与 POST 共用 handler（o2server 端该端点为 GET 无 body）。
    let raw = send(
        client,
        auth,
        reqwest::Method::GET,
        &format!("{base}/api/attendance/assemble/control/statistic/do"),
        Some(json!({})),
    )
    .await;
    assert_success(&raw, "attendance GET 变体");
}

async fn case_calendar_config(client: &Client, base: &str, auth: &str, pool: &Arc<TestPool>) {
    let raw = send(
        client,
        auth,
        reqwest::Method::GET,
        &format!("{base}/api/calendar_assemble_control/update/control/config"),
        Some(json!({ "enabled": false, "defaultTimeZone": "UTC+8", "allowSharing": false })),
    )
    .await;
    assert_success(&raw, "calendar 首次");
    let db = pg_client(pool).await;
    let stored = db
        .query_one(
            "SELECT config_value FROM cal_control_config WHERE config_key = 'global' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .expect("config write must be readable (write-then-read)")
        .get::<_, String>(0);
    let parsed: Value = serde_json::from_str(&stored).expect("config_value must be JSON");
    assert_eq!(parsed["enabled"], false, "stored={stored}");
    assert_eq!(parsed["allowSharing"], false, "stored={stored}");
    assert_eq!(parsed["defaultTimeZone"], "UTC+8", "stored={stored}");

    // 第二次换值：必须更新同一行，不新增第二行。
    let raw = send(
        client,
        auth,
        reqwest::Method::GET,
        &format!("{base}/api/calendar_assemble_control/update/control/config"),
        Some(json!({ "enabled": true, "defaultTimeZone": "UTC+8", "allowSharing": true })),
    )
    .await;
    assert_success(&raw, "calendar 二次");
    let count: i64 = db
        .query_one(
            "SELECT COUNT(*) FROM cal_control_config WHERE config_key = 'global'",
            &[],
        )
        .await
        .expect("count failed")
        .get(0);
    assert_eq!(count, 1, "全局配置必须单行幂等");
    let stored = db
        .query_one(
            "SELECT config_value FROM cal_control_config WHERE config_key = 'global'",
            &[],
        )
        .await
        .expect("read config failed")
        .get::<_, String>(0);
    let parsed: Value = serde_json::from_str(&stored).expect("config_value must be JSON");
    assert_eq!(parsed["enabled"], true, "stored={stored}");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute(
            "DELETE FROM x_attendance_statistic WHERE person_id = $1 AND year = $2 AND month = $3",
            &[&PERSON_ID, &YEAR, &MONTH],
        )
        .await;
    let _ = db
        .execute("DELETE FROM x_attendance_statistic WHERE id = '--'", &[])
        .await;
    let _ = db
        .execute(
            "DELETE FROM cal_control_config WHERE config_key = 'global'",
            &[],
        )
        .await;
}

fn assert_success((status, raw): &(reqwest::StatusCode, String), label: &str) {
    assert_eq!(
        status,
        &reqwest::StatusCode::OK,
        "{label} 应为 200（旧实现 ON CONFLICT 指向不存在约束 → E42P10 500），实际 {status}，body={raw}"
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
