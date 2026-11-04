// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 14：`column:equipment`（`x_meeting_room.equipment` 是 jsonb，
//! SeaORM 实体却声明为 `Option<String>` → `Entity::find` 解码失败，GET 500）。
//!
//! 覆盖三条路径：`GET /room/list`、`GET /room/{id}`（读），以及
//! `POST /room/create`（写，含写后必读：回库 `equipment::text` 与写入值一致）。

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const ROOM_ID: &str = "it-room-jsonb";

pub async fn meeting_room_jsonb_flow(pool: Arc<TestPool>) {
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

    // 读路径 1：列表
    let body = get_json(
        &client,
        &auth,
        &format!("{base}/api/meeting/core/entity/room/list"),
    )
    .await;
    // 该 handler 的 payload 是 `{count, data}`，ActionResult 再包一层 `data`。
    let data = body["data"]["data"].as_array().expect("data.data 应为数组");
    assert!(
        data.iter().any(|r| r["id"] == ROOM_ID),
        "room/list 必须包含种子会议室，body={body}"
    );

    // 读路径 2：单条，且 jsonb 能解出原值
    let body = get_json(
        &client,
        &auth,
        &format!("{base}/api/meeting/core/entity/room/{ROOM_ID}"),
    )
    .await;
    assert_eq!(body["data"]["id"], ROOM_ID, "body={body}");
    assert_eq!(
        body["data"]["equipment"],
        json!([{"name": "projector"}]),
        "equipment 应原样返回 jsonb 内容，body={body}"
    );

    // 写路径：create 后立即读回 jsonb 同一值
    let resp = client
        .post(format!("{base}/api/meeting/core/entity/room/create"))
        .header("Authorization", &auth)
        .json(&json!({
            "name": "it-room-created",
            "equipment": [{"name": "tv"}],
        }))
        .send()
        .await
        .expect("request failed");
    assert_eq!(
        resp.status(),
        reqwest::StatusCode::OK,
        "room/create 应 200，body={}",
        resp.text().await.unwrap_or_default()
    );
    let db = pg_client(&pool).await;
    let stored: Option<String> = db
        .query_one(
            "SELECT equipment::text FROM x_meeting_room WHERE name = 'it-room-created'",
            &[],
        )
        .await
        .expect("write-then-read failed")
        .get(0);
    let stored: Value =
        serde_json::from_str(&stored.expect("创建后应有行")).expect("equipment 应为合法 JSON");
    assert_eq!(
        stored,
        json!([{"name": "tv"}]),
        "写入的 equipment 必须原样落库"
    );

    cleanup(&pool).await;
    server.abort();
    let _ = server.await;
}

async fn seed(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute("DELETE FROM x_meeting_room WHERE id = $1", &[&ROOM_ID])
        .await;
    db.execute(
        "INSERT INTO x_meeting_room (id, name, equipment, create_time) \
         VALUES ($1, 'it-room-jsonb', '[{\"name\":\"projector\"}]'::jsonb, '2026-01-01 10:00:00')",
        &[&ROOM_ID],
    )
    .await
    .expect("seed x_meeting_room failed");
}

async fn cleanup(pool: &Arc<TestPool>) {
    let db = pg_client(pool).await;
    let _ = db
        .execute(
            "DELETE FROM x_meeting_room WHERE id = $1 OR name = 'it-room-created'",
            &[&ROOM_ID],
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
        "应 200（jsonb 按 String 解码 → 500），实际 {status}，body={raw}"
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
