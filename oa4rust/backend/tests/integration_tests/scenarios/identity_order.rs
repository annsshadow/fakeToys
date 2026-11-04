// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 15：`wrong-type`（`x_org_identity.order_number` 是 bigint，代码绑定 i32）。
//!
//! `identity_flag_order_before_followFlag` 重排时用 `(index + 1) as i32` 绑定
//! `order_number = $1` → tokio-postgres `WrongType` → 500。改绑 i64。
//! 断言含写后必读：调用后回库逐行检查 `order_number` 就是新顺序。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const UNIT_ID: &str = "it-unit-order";
const ID_A: &str = "it-ident-a";
const ID_B: &str = "it-ident-b";

pub async fn identity_order_flow(pool: Arc<TestPool>) {
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

    // flag=B、followFlag=(0)：把 B 移到顺序末尾 → [A, B]。
    let resp = client
        .get(format!(
            "{base}/api/organization/assemble/control/identity/{ID_B}/order/before/(0)"
        ))
        .header("Authorization", &auth)
        .send()
        .await
        .expect("request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "应 200（order_number 是 bigint，绑定 i32 → WrongType 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["success"], true, "body={raw}");

    // 写后必读：顺序为 A=1, B=2。
    let db = pg_client(&pool).await;
    for (id, expected) in [(ID_A, 1_i64), (ID_B, 2_i64)] {
        let actual: Option<i64> = db
            .query_one(
                "SELECT order_number FROM x_org_identity WHERE id = $1",
                &[&id],
            )
            .await
            .expect("write-then-read failed")
            .get(0);
        assert_eq!(
            actual,
            Some(expected),
            "{id} 的 order_number 应为 {expected}"
        );
    }

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    cleanup_with(&db).await;
    for (id, created) in [
        (ID_A, "'2026-01-01 10:00:00'"),
        (ID_B, "'2026-01-02 10:00:00'"),
    ] {
        db.execute(
            &format!(
                "INSERT INTO x_org_identity \
                 (id, name, unit_id, person_id, type, major, creator, create_time) \
                 VALUES ($1, $1, $2, 'it-person', 'identity', false, 'it-admin', {created})"
            ),
            &[&id, &UNIT_ID],
        )
        .await
        .expect("seed x_org_identity failed");
    }
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    cleanup_with(&db).await;
}

async fn cleanup_with(db: &deadpool_postgres::Object) {
    let _ = db
        .execute("DELETE FROM x_org_identity WHERE unit_id = $1", &[&UNIT_ID])
        .await;
}

async fn pg_client(pool: &Arc<TestPool>) -> deadpool_postgres::Object {
    pool.as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client")
}
