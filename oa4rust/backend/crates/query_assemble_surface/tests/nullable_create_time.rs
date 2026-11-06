// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C2：`query_assemble_surface` 中 `x_query_view.create_time` 的 5 处解码。
//!
//! 迁移证据：`029_create_missing_core_entity_tables.sql:362`
//! `CREATE TABLE x_query_view (... "create_time" TIMESTAMP, ...)`，无 NOT NULL；
//! 全仓无 `ALTER COLUMN create_time` 改类型或加约束。
//!
//! 其中 1 处 SQL 读取裸 TIMESTAMP（`view_flag_..._execute_v2_page_...`），
//! 另 4 处为 `to_char(...)`（NULL 仍返回 NULL）或 `::text`；解码统一改
//! `Option<String>` 兜底空串。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_query_surface";

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

fn find<'a>(items: &'a [Value], id: &str, label: &str) -> &'a Value {
    items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some(id))
        .unwrap_or_else(|| panic!("{label}: {id} 必须在结果中"))
}

#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn all_view_readers_decode_null_create_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_query_view (id, name, query_type, view_flag, query_flag, content, creator, \
         create_time) VALUES \
         ('o36-q-null', 'A_null', 'table', 'o36-v', 'o36-q', NULL, 'p1', NULL), \
         ('o36-q-time', 'B_time', 'table', 'o36-v', 'o36-q', NULL, 'p1', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_query_view");
    drop(c);

    // 1) to_char 路径：view_flag_flag_query_queryFlag（LIMIT 1，取 NULL 行）。
    let res = query_assemble_surface::view_flag_flag_query_queryFlag(
        Extension(pool.clone()),
        Path(("o36-v".to_string(), "o36-q".to_string())),
    )
    .await
    .expect("view_flag_flag_query_queryFlag 不得 500");
    assert_empty(
        &res.0.data.expect("缺 data"),
        "view_flag_flag_query_queryFlag",
    );

    // 2) 裸 TIMESTAMP 路径：execute_v2 分页。
    let res =
        query_assemble_surface::view_flag_flag_query_queryFlag_execute_v2_page_page_size_size(
            Extension(pool.clone()),
            Path(("o36-v".to_string(), "o36-q".to_string(), 1, 20)),
        )
        .await
        .expect("execute_v2_page 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.as_array().expect("data 必须是数组");
    assert_empty(
        find(arr, "o36-q-null", "execute_v2_page"),
        "execute_v2_page",
    );

    // 3) to_char 路径：view_list_query_queryFlag。
    let res = query_assemble_surface::view_list_query_queryFlag(
        Extension(pool.clone()),
        Path("o36-q".to_string()),
    )
    .await
    .expect("view_list_query_queryFlag 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.as_array().expect("data 必须是数组");
    assert_empty(
        find(arr, "o36-q-null", "view_list_query_queryFlag"),
        "view_list_query_queryFlag",
    );

    // 4) ::text 路径：view_id。
    let res =
        query_assemble_surface::view_id(Extension(pool.clone()), Path("o36-q-null".to_string()))
            .await
            .expect("view_id 不得 500");
    assert_empty(&res.0.data.expect("缺 data"), "view_id");

    // 5) ::text 路径：view_id_execute_v2 分页。
    let res = query_assemble_surface::view_id_execute_v2_page_page_size_size(
        Extension(pool.clone()),
        Path(("legacy".to_string(), "o36-q-null".to_string(), 1, 20)),
    )
    .await
    .expect("view_id_execute_v2_page 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.as_array().expect("data 必须是数组");
    assert_empty(
        find(arr, "o36-q-null", "view_id_execute_v2_page"),
        "view_id_execute_v2_page",
    );

    drop_scratch().await;
}
