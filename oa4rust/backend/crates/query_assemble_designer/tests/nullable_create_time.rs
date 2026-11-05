// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C2：`query_assemble_designer` 中 `x_query_view.create_time` 的 4 处解码。
//!
//! 迁移证据：`029_create_missing_core_entity_tables.sql:362`，`create_time TIMESTAMP`
//! 无 NOT NULL；全仓无改类型/加约束语句。
//!
//! 3 处 `to_char(create_time, ...)`（NULL 仍返回 NULL）、1 处 `::text`；解码统一改
//! `Option<String>` 兜底空串。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_query_designer";

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

fn assert_empty(obj: &Value, label: &str) {
    assert_eq!(
        obj.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "{label}: NULL create_time 必须归一为空串，实收: {:?}",
        obj.get("createTime")
    );
}

#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn all_view_readers_decode_null_create_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_query_view (id, name, query_type, view_flag, query_flag, content, creator, \
         create_time) VALUES \
         ('o36-qd-a', 'A_null', 'table', 'o36-v', 'o36-qd', NULL, 'p1', NULL), \
         ('o36-qd-b', 'B_time', 'table', 'o36-v', 'o36-qd', NULL, 'p1', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_query_view");
    drop(c);

    // 1) to_char 路径：view_list_query_flag。
    let res = query_assemble_designer::view_list_query_flag(
        Extension(pool.clone()),
        Path("o36-qd".to_string()),
    )
    .await
    .expect("view_list_query_flag 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.as_array().expect("data 必须是数组");
    let null_row = arr
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o36-qd-a"))
        .expect("NULL 行必须在列表中");
    assert_empty(null_row, "view_list_query_flag");

    // 2) to_char 路径：view_list_id_next_count（游标落在 a，取 b）。
    let res = query_assemble_designer::view_list_id_next_count(
        Extension(pool.clone()),
        Path(("o36-qd-a".to_string(), 20_i64)),
    )
    .await
    .expect("view_list_id_next_count 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.as_array().expect("data 必须是数组");
    let b_row = arr
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o36-qd-b"))
        .expect("b 行必须在 next 结果中");
    assert!(
        b_row
            .get("createTime")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.starts_with("2025-01-01")),
        "有值行 createTime 不得被改动，实收: {:?}",
        b_row.get("createTime")
    );

    // 3) to_char 路径：view_list_id_prev_count（游标落在 b，取 a=NULL）。
    let res = query_assemble_designer::view_list_id_prev_count(
        Extension(pool.clone()),
        Path(("o36-qd-b".to_string(), 20_i64)),
    )
    .await
    .expect("view_list_id_prev_count 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.as_array().expect("data 必须是数组");
    let a_row = arr
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o36-qd-a"))
        .expect("a 行必须在 prev 结果中");
    assert_empty(a_row, "view_list_id_prev_count");

    // 4) ::text 路径：view_id。
    let res =
        query_assemble_designer::view_id(Extension(pool.clone()), Path("o36-qd-a".to_string()))
            .await
            .expect("view_id 不得 500");
    assert_empty(&res.0.data.expect("缺 data"), "view_id");

    drop_scratch().await;
}
