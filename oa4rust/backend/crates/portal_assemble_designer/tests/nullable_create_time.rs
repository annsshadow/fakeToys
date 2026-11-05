// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C1：`portal_assemble_designer` 中 5 处 `x_portal_page.create_time` 解码，
//! 另含同表 `update_time` 同族（029:291 同为可空 TIMESTAMP）。
//!
//! 迁移证据：`029_create_missing_core_entity_tables.sql:290-291`，两列均无 NOT NULL，
//! 全仓无改类型/加约束语句。修复前两处列表 SQL 读取裸 TIMESTAMP，NULL 行 panic → 500；
//! 修复为 `::text AS ...` + `Option<String>`。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_portal_designer";

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

fn assert_null_times(obj: &Value, label: &str) {
    assert_eq!(
        obj.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "{label}: NULL create_time 必须归一为空串，实收: {:?}",
        obj.get("createTime")
    );
    assert_eq!(
        obj.get("updateTime").and_then(|v| v.as_str()),
        Some(""),
        "{label}: NULL update_time 必须归一为空串，实收: {:?}",
        obj.get("updateTime")
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
async fn all_page_readers_decode_null_times() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_portal_page (id, portal_id, name, status, category, content, creator, \
         create_time, update_time, deleted_at) VALUES \
         ('o36-pad-null', 'o36-portal', 'A_null', 'published', 'o36-cat', NULL, 'p1', NULL, NULL, NULL), \
         ('o36-pad-time', 'o36-portal', 'B_time', 'published', 'o36-cat', NULL, 'p1', \
          '2025-01-01 08:00:00', '2025-01-02 08:00:00', NULL)",
        &[],
    )
    .await
    .expect("seed x_portal_page");
    drop(c);

    // 1) list_pages_by_category。
    let res = portal_assemble_designer::list_pages_by_category(
        Extension(pool.clone()),
        Path("o36-cat".to_string()),
    )
    .await
    .expect("list_pages_by_category 不得 500");
    let data = res.0.data.expect("缺 data");
    let items = data.get("data").expect("缺 data.data");
    let arr = items.as_array().expect("data.data 必须是数组");
    assert_null_times(
        find(arr, "o36-pad-null", "list_pages_by_category"),
        "list_pages_by_category",
    );

    // 2) get_page。
    let res = portal_assemble_designer::get_page(
        Extension(pool.clone()),
        Path("o36-pad-null".to_string()),
    )
    .await
    .expect("get_page 不得 500");
    assert_null_times(&res.0.data.expect("缺 data"), "get_page");

    // 3) page_list_portal_portalId。
    let res = portal_assemble_designer::page_list_portal_portalId(
        Extension(pool.clone()),
        Path("o36-portal".to_string()),
    )
    .await
    .expect("page_list_portal_portalId 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.get("data").expect("缺 data.data");
    let items = arr.as_array().expect("data.data 必须是数组");
    assert_null_times(
        find(items, "o36-pad-null", "page_list_portal_portalId"),
        "page_list_portal_portalId",
    );

    // 4) page_id。
    let res = portal_assemble_designer::page_id(
        Extension(pool.clone()),
        Path("o36-pad-null".to_string()),
    )
    .await
    .expect("page_id 不得 500");
    assert_null_times(&res.0.data.expect("缺 data"), "page_id");

    // 5) page_list。
    let res = portal_assemble_designer::page_list(Extension(pool.clone()))
        .await
        .expect("page_list 不得 500");
    let data = res.0.data.expect("缺 data");
    let arr = data.get("data").expect("缺 data.data");
    let items = arr.as_array().expect("data.data 必须是数组");
    assert_null_times(find(items, "o36-pad-null", "page_list"), "page_list");

    drop_scratch().await;
}
