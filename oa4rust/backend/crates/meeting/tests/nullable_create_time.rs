// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O35：`x_meeting_building.create_time` 是 **可空 TIMESTAMP**（006 migration：
//! `DEFAULT CURRENT_TIMESTAMP`，无 NOT NULL）。本 crate 走 `create_time::text`
//! 读取，NULL 列值经 `::text` 仍是 NULL；修复前 `row.get::<_, String>` 解码 NULL
//! 会 panic → 500，修复后应落空串且不影响有值行。
//!
//! 专用 scratch 库 `oa_o35_meeting`，造「有值 + NULL」两行后直调 handler。

use axum::extract::Extension;
use deadpool_postgres::Pool;

const SCRATCH_DB: &str = "oa_o35_meeting";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(mgr)
        .max_size(4)
        .build()
        .unwrap()
}

async fn fresh_scratch() -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {SCRATCH_DB} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {SCRATCH_DB}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pool_to(SCRATCH_DB);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 库迁移");
    pool
}

async fn drop_scratch() {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {SCRATCH_DB} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

#[tokio::test]
async fn building_list_decodes_null_create_time_as_empty_string() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_meeting_building (id, name, create_time) VALUES \
         ('o35-bld-null', 'A_null', NULL), \
         ('o35-bld-time', 'B_time', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_meeting_building");
    drop(c);

    // 修复前：NULL 行解码 panic → 500。修复后：200 且该行 createTime 为空串。
    let res = meeting::building_list(Extension(pool.clone()))
        .await
        .expect("NULL create_time 行不得 500");
    let data = res.0.data.expect("building list 缺 data");
    let items = data.as_array().expect("data 必须是数组");

    let null_row = items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o35-bld-null"))
        .expect("NULL 行必须在列表中");
    assert_eq!(
        null_row.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须归一为空串"
    );

    let timed_row = items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o35-bld-time"))
        .expect("有值行必须在列表中");
    assert!(
        timed_row
            .get("createTime")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.starts_with("2025-01-01")),
        "有值行的 createTime 格式不得被改动，实收: {:?}",
        timed_row.get("createTime")
    );

    drop_scratch().await;
}
