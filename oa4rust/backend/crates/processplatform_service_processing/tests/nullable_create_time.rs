// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C3：`processplatform_service_processing::work_list` 的 `x_work.create_time` 解码。
//!
//! 迁移证据：`020_create_processplatform_tables.sql:11` `create_time TIMESTAMP DEFAULT
//! CURRENT_TIMESTAMP`（可空，无 NOT NULL）；全仓无改类型/加约束。SQL 已
//! `to_char(create_time, ...)`（NULL 仍返回 NULL），解码改为 `Option<String>` 兜底空串。

use axum::extract::{Extension, Query};
use deadpool_postgres::Pool;
use serde_json::Value;
use std::collections::HashMap;

const SCRATCH_DB: &str = "oa_o36_pp_work_misc";

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
async fn work_list_decodes_null_create_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_work (id, title, process, creator, create_time) VALUES \
         ('o36-work-null', 'A_null', 'o36-proc', 'p1', NULL), \
         ('o36-work-time', 'B_time', 'o36-proc', 'p1', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_work");
    drop(c);

    let res = processplatform_service_processing::work_list(
        Extension(pool.clone()),
        Query(HashMap::new()),
    )
    .await
    .expect("work_list 不得 500");
    let arr: Vec<Value> = res
        .0
        .data
        .expect("缺 data")
        .as_array()
        .cloned()
        .unwrap_or_default();
    let null_row = arr
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o36-work-null"))
        .expect("NULL 行必须在列表中");
    assert_eq!(
        null_row.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须归一为空串，实收: {:?}",
        null_row.get("createTime")
    );

    drop_scratch().await;
}
