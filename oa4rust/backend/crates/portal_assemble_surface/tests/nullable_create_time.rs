// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C1：`portal_assemble_surface` 中 9 处 `x_portal_page.create_time` 解码。
//!
//! 迁移证据：`backend/migrations/029_create_missing_core_entity_tables.sql:290`
//! `CREATE TABLE x_portal_page (... "create_time" TIMESTAMP, ...)`，无 NOT NULL；
//! 全仓无 `ALTER COLUMN create_time` 改类型/加约束。
//!
//! 修复前三处市值 SQL 读取裸 TIMESTAMP（NULL 或非 NULL 都会因 TIMESTAMP→String
//! 类型不匹配 panic）。修复为 SQL 侧 `::text AS create_time` + 解码侧
//! `Option<String>`。本测试一次性覆盖 9 个读取入口。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_portal_surface";

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

fn assert_null_create_time(obj: &Value, label: &str) {
    assert_eq!(
        obj.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "{label}: NULL create_time 必须归一为空串，实收: {:?}",
        obj.get("createTime")
    );
}

#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn all_page_readers_decode_null_create_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_portal_page (id, portal_id, name, status, category, content, mobile_content, \
         flag, portal_flag, creator, create_time) VALUES \
         ('o36-pas-null', 'o36-portal', 'A_null', 'published', 'o36-cat', NULL, NULL, \
          'o36-f1', 'o36-pf1', 'p1', NULL), \
         ('o36-pas-time', 'o36-portal', 'B_time', 'published', 'o36-cat', NULL, NULL, \
          'o36-f2', 'o36-pf2', 'p1', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_portal_page");
    drop(c);

    // 1) page_list_portal_portal：按 portal_id 列的列表。
    let res = portal_assemble_surface::page_list_portal_portal(
        Extension(pool.clone()),
        Path("o36-portal".to_string()),
    )
    .await
    .expect("page_list_portal_portal 不得 500");
    let items = res
        .0
        .data
        .expect("缺 data")
        .as_array()
        .cloned()
        .unwrap_or_default();
    let null_row = items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o36-pas-null"))
        .expect("NULL 行必须在列表中");
    assert_null_create_time(null_row, "page_list_portal_portal");

    // 2/3) v2 flag + portalFlag（桌面 / 移动）。
    let res = portal_assemble_surface::page_v2_flag_portal_portalFlag(
        Extension(pool.clone()),
        Path(("o36-f1".to_string(), "o36-pf1".to_string())),
    )
    .await
    .expect("page_v2_flag_portal_portalFlag 不得 500");
    assert_null_create_time(
        &res.0.data.expect("缺 data"),
        "page_v2_flag_portal_portalFlag",
    );

    let res = portal_assemble_surface::page_v2_flag_portal_portalFlag_mobile(
        Extension(pool.clone()),
        Path(("o36-f1".to_string(), "o36-pf1".to_string())),
    )
    .await
    .expect("page_v2_flag_portal_portalFlag_mobile 不得 500");
    assert_null_create_time(
        &res.0.data.expect("缺 data"),
        "page_v2_flag_portal_portalFlag_mobile",
    );

    // 4/5) v2 按 id（桌面 / 移动）。
    let res = portal_assemble_surface::page_v2_id(
        Extension(pool.clone()),
        Path("o36-pas-null".to_string()),
    )
    .await
    .expect("page_v2_id 不得 500");
    assert_null_create_time(&res.0.data.expect("缺 data"), "page_v2_id");

    let res = portal_assemble_surface::page_v2_id_mobile(
        Extension(pool.clone()),
        Path("o36-pas-null".to_string()),
    )
    .await
    .expect("page_v2_id_mobile 不得 500");
    assert_null_create_time(&res.0.data.expect("缺 data"), "page_v2_id_mobile");

    // 6/7) flag + portal_id（桌面 / 移动）。
    let res = portal_assemble_surface::page_flag_portal_portalFlag(
        Extension(pool.clone()),
        Path(("o36-f1".to_string(), "o36-portal".to_string())),
    )
    .await
    .expect("page_flag_portal_portalFlag 不得 500");
    assert_null_create_time(&res.0.data.expect("缺 data"), "page_flag_portal_portalFlag");

    let res = portal_assemble_surface::page_flag_portal_portalFlag_mobile(
        Extension(pool.clone()),
        Path(("o36-f1".to_string(), "o36-portal".to_string())),
    )
    .await
    .expect("page_flag_portal_portalFlag_mobile 不得 500");
    assert_null_create_time(
        &res.0.data.expect("缺 data"),
        "page_flag_portal_portalFlag_mobile",
    );

    // 8) page_id（两段路径）。
    let res = portal_assemble_surface::page_id(
        Extension(pool.clone()),
        Path(("legacy".to_string(), "o36-pas-null".to_string())),
    )
    .await
    .expect("page_id 不得 500");
    assert_null_create_time(&res.0.data.expect("缺 data"), "page_id");

    // 9) page_id_mobile。
    let res = portal_assemble_surface::page_id_mobile(
        Extension(pool.clone()),
        Path("o36-pas-null".to_string()),
    )
    .await
    .expect("page_id_mobile 不得 500");
    assert_null_create_time(&res.0.data.expect("缺 data"), "page_id_mobile");

    drop_scratch().await;
}
