// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O24-3 批次 2：JSONB 列按 String 绑定的同根因族（4 条基线）。
//!
//! 共同根因：SQL 里 `$n::jsonb` / `jsonb_set(..., $2::jsonb, ...)` 会把参数类型推断为
//! JSONB，而 Rust 侧传入的是 `String`/`&str` → `ToSql` 失败（WrongType, postgres: Jsonb）。
//! 真实列类型经 `information_schema` 现场确认：
//!
//! - `x_meeting_room.equipment` 是 **JSONB**；
//! - `x_process_surface.content` 是 **TEXT**（`$1::jsonb` 只是在参数上多套了一层）；
//! - `x_query_table_data.data` 是 **TEXT**（`jsonb_set(...$2::jsonb...)` 同上）。
//!
//! 修法：把参数侧显式转成文本路径（`$n::text::jsonb` / `$1::text`），让绑定类型保持
//! String，写入仍产生合法 JSON。四条基线（3 个 handler，queryview 两条路由共用实现）：
//!
//! 1. `POST /api/meeting/core/entity/room/save/{id}`
//! 2. `POST /api/processplatform/assemble/surface/save/{id}`
//! 3. `POST /api/queryview/table/row/part/update/{tableFlag}/{id}`
//! 4. `POST /api/queryview/table/{flag}/row/{rid}/part/update`
#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

use crate::integration_tests::db::TestPool;
use crate::integration_tests::helpers::setup_test_server;

const ROOM_ID: &str = "it-jsonb-meeting-room";
const SURFACE_ID: &str = "it-jsonb-process-surface";
const TABLE_FLAG: &str = "it-jsonb-table";
const ROW_ID: &str = "it-jsonb-row";

pub async fn jsonb_param_binding_flow(pool: Arc<TestPool>) {
    let (addr, server, token) = setup_test_server(pool.clone())
        .await
        .expect("failed to start test server");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client");
    let base = format!("http://{}", addr);
    let auth = format!("Bearer {}", token);

    case_meeting_room_equipment(&client, &base, &auth, &pool).await;
    case_process_surface_content(&client, &base, &auth, &pool).await;
    case_queryview_part_update(&client, &base, &auth, &pool, false).await;
    case_queryview_part_update(&client, &base, &auth, &pool, true).await;

    server.abort();
    let _ = server.await;
}

/// `equipment` 真实类型 JSONB；payload 是数组，必须以 JSON 落库。
async fn case_meeting_room_equipment(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let equipment = json!([{"name": "投影仪"}, {"name": "白板"}]);
    let db = pg_client(pool).await;
    db.execute("DELETE FROM x_meeting_room WHERE id = $1", &[&ROOM_ID])
        .await
        .expect("delete meeting room failed");
    db.execute(
        "INSERT INTO x_meeting_room (id, name, equipment) VALUES ($1, 'it room', '{}'::jsonb)",
        &[&ROOM_ID],
    )
    .await
    .expect("insert meeting room failed");

    let payload = json!({
        "name": "it room updated",
        "buildingId": "it-building",
        "floor": "3",
        "capacity": 12,
        "equipment": equipment,
        "description": "it description",
        "orderNumber": 7,
    });
    let resp = client
        .post(format!(
            "{base}/api/meeting/core/entity/room/save/{ROOM_ID}"
        ))
        .header("Authorization", auth)
        .json(&payload)
        .send()
        .await
        .expect("save room request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("save room body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "会议室保存应为 200（旧实现 equipment 按 String 绑 JSONB → 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("save room body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["id"], ROOM_ID, "body={raw}");
    assert_eq!(body["data"]["saved"], true, "body={raw}");
    assert_eq!(body["data"]["name"], "it room updated", "body={raw}");

    let row = db
        .query_one(
            "SELECT name, equipment::text FROM x_meeting_room WHERE id = $1",
            &[&ROOM_ID],
        )
        .await
        .expect("read meeting room failed");
    assert_eq!(row.get::<_, String>("name"), "it room updated");
    let stored: Value = serde_json::from_str(&row.get::<_, String>("equipment")).expect("jsonb");
    assert_eq!(stored, equipment, "equipment 必须以 JSON 数组落库");
}

/// `content` 真实类型 TEXT；payload 原样写入并原样回显。
async fn case_process_surface_content(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
) {
    let payload = json!({"name": "it surface", "elements": [{"id": "e1"}], "width": 1200});
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_process_surface WHERE id = $1",
        &[&SURFACE_ID],
    )
    .await
    .expect("delete process surface failed");
    db.execute(
        "INSERT INTO x_process_surface (id, name, content) VALUES ($1, 'it surface', '{}')",
        &[&SURFACE_ID],
    )
    .await
    .expect("insert process surface failed");

    let resp = client
        .post(format!(
            "{base}/api/processplatform/assemble/surface/save/{SURFACE_ID}"
        ))
        .header("Authorization", auth)
        .json(&payload)
        .send()
        .await
        .expect("save surface request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("save surface body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "表面保存应为 200（旧实现 $1::jsonb 把参数推断为 JSONB → 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("save surface body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["id"], SURFACE_ID, "body={raw}");
    assert_eq!(body["data"]["content"], payload, "body={raw}");

    let stored: String = db
        .query_one(
            "SELECT content FROM x_process_surface WHERE id = $1",
            &[&SURFACE_ID],
        )
        .await
        .expect("read process surface failed")
        .get(0);
    let stored: Value = serde_json::from_str(&stored).expect("content 必须是合法 JSON");
    assert_eq!(stored, payload);
}

/// `data` 真实类型 TEXT；`jsonb_set(..., $2::jsonb, ...)` 把参数推断为 JSONB。
/// 两条路由共用同一个 handler，`legacy_shape=true` 走旧形态路径。
async fn case_queryview_part_update(
    client: &Client,
    base: &str,
    auth: &str,
    pool: &Arc<TestPool>,
    legacy_shape: bool,
) {
    let payload = json!({"k": "v", "n": 1});
    let expected = json!({"k": "v", "n": 1});
    let db = pg_client(pool).await;
    db.execute(
        "DELETE FROM x_query_table_data WHERE id = $1 AND table_flag = $2",
        &[&ROW_ID, &TABLE_FLAG],
    )
    .await
    .expect("delete table row failed");
    db.execute(
        "INSERT INTO x_query_table_data (id, table_flag, data) VALUES ($1, $2, '{}')",
        &[&ROW_ID, &TABLE_FLAG],
    )
    .await
    .expect("insert table row failed");

    let url = if legacy_shape {
        format!("{base}/api/queryview/table/{TABLE_FLAG}/row/{ROW_ID}/part/update")
    } else {
        format!("{base}/api/queryview/table/row/part/update/{TABLE_FLAG}/{ROW_ID}")
    };
    let resp = client
        .post(url)
        .header("Authorization", auth)
        .json(&payload)
        .send()
        .await
        .expect("part update request failed");
    let status = resp.status();
    let raw = resp.text().await.expect("part update body");
    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "行局部更新应为 200（旧实现 $2::jsonb 把参数推断为 JSONB → 500），实际 {status}，body={raw}"
    );
    let body: Value = serde_json::from_str(&raw).expect("part update body must be JSON");
    assert_eq!(body["type"], "success", "body={raw}");
    assert_eq!(body["data"]["id"], ROW_ID, "body={raw}");
    assert_eq!(body["data"]["tableFlag"], TABLE_FLAG, "body={raw}");
    assert_eq!(body["data"]["saved"], 1, "body={raw}");

    let stored: String = db
        .query_one(
            "SELECT (data::jsonb -> '/part')::text FROM x_query_table_data \
             WHERE id = $1 AND table_flag = $2",
            &[&ROW_ID, &TABLE_FLAG],
        )
        .await
        .expect("read table row failed")
        .get(0);
    let stored: Value = serde_json::from_str(&stored).expect("part 必须是合法 JSON");
    assert_eq!(stored, expected, "局部写入应在 /part 下保留 JSON");
}

async fn pg_client(pool: &Arc<TestPool>) -> deadpool_postgres::Object {
    pool.as_pg()
        .expect("requires PostgreSQL pool")
        .get()
        .await
        .expect("failed to get pool client")
}
