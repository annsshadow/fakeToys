// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C1：`x_portal_page.create_time` / `update_time` 是**可空 TIMESTAMP**
//! （migration 029 `CREATE TABLE x_portal_page`，两列均无 NOT NULL；全仓无
//! `ALTER COLUMN ... SET NOT NULL` 或改类型语句）。
//!
//! 本 crate 的 `page_list` 曾直接 `row.get::<_, String>("create_time")` 解码
//! 裸 TIMESTAMP：NULL 行会 panic → 500，非 NULL 行也会因 TIMESTAMP→String
//! 类型不匹配而 panic。修复为 SQL 侧 `::text` + 解码侧 `Option<String>`。
//!
//! 专用 scratch 库 `oa_o36_portal`，造「NULL 双列 + 有值双列」两行后直调 handler。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_portal";

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

fn find<'a>(items: &'a [Value], id: &str) -> &'a Value {
    items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some(id))
        .unwrap_or_else(|| panic!("{id} 必须在结果中"))
}

#[tokio::test]
async fn page_list_decodes_null_create_time_and_update_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_portal_page (id, portal_id, name, status, category, content, creator, \
         create_time, update_time) VALUES \
         ('o36-pp-null', 'o36-portal', 'A_null', 'published', 'o36-cat', NULL, 'p1', NULL, NULL), \
         ('o36-pp-time', 'o36-portal', 'B_time', 'published', 'o36-cat', NULL, 'p1', \
          '2025-01-01 08:00:00', '2025-01-02 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_portal_page");
    drop(c);

    // 修复前：NULL 行 `row.get::<_, String>` panic → 500。
    let res = portal::page_list(Extension(pool.clone()))
        .await
        .expect("NULL create_time 行不得 500");
    let data = res.0.data.expect("page_list 缺 data");
    let items = data.as_array().expect("data 必须是数组");

    let null_row = find(items, "o36-pp-null");
    assert_eq!(
        null_row.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须归一为空串"
    );
    assert_eq!(
        null_row.get("updateTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL update_time 必须归一为空串"
    );

    let timed_row = find(items, "o36-pp-time");
    assert!(
        timed_row
            .get("createTime")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.starts_with("2025-01-01")),
        "有值行 createTime 不得被改动，实收: {:?}",
        timed_row.get("createTime")
    );
    assert!(
        timed_row
            .get("updateTime")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.starts_with("2025-01-02")),
        "有值行 updateTime 不得被改动，实收: {:?}",
        timed_row.get("updateTime")
    );

    drop_scratch().await;
}

#[tokio::test]
async fn get_page_decodes_null_create_time_and_update_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_portal_page (id, portal_id, name, status, category, content, creator, \
         create_time, update_time) VALUES \
         ('o36-pp-single-null', 'o36-portal', 'A_null', 'published', 'o36-cat', NULL, 'p1', NULL, NULL)",
        &[],
    )
    .await
    .expect("seed x_portal_page");
    drop(c);

    let res = portal::get_page(
        Extension(pool.clone()),
        Path("o36-pp-single-null".to_string()),
    )
    .await
    .expect("单条详情不得 500");
    let obj = res.0.data.expect("get_page 缺 data");
    let null_row = &obj;

    assert_eq!(
        null_row.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须归一为空串"
    );
    assert_eq!(
        null_row.get("updateTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL update_time 必须归一为空串"
    );

    drop_scratch().await;
}
