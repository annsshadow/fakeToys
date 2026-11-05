// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O33：组织域列表的 keyset 分页（`create_time` 为 TIMESTAMP，非 TEXT）。
//!
//! 该表曾被 `COALESCE(create_time, '')` 的文本归一写法打挂
//! （`invalid input syntax for type timestamp: ""`，seeded-route-scan 抓到）。
//! 本回归同时验证：时间戳列的 NULL 归一（`'-infinity'::timestamp`）、
//! 跨页不漏不重、prev 窗口正确。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use organization_assemble_control::{
    organization_assemble_control_role_list_flag_next_count, role_list_flag_prev_count,
};

const SCRATCH: &str = "oa_par_org_ks";

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
        &format!("DROP DATABASE IF EXISTS {SCRATCH} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {SCRATCH}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pool_to(SCRATCH);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 库迁移");
    pool
}

fn ids_of(data: &serde_json::Value) -> Vec<String> {
    data.as_array()
        .expect("list data 必须是数组")
        .iter()
        .map(|r| {
            r.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

#[tokio::test]
async fn timestamp_keyset_walks_every_row_exactly_once() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    // create_time 为 TIMESTAMP；id 序（r1<r2<…）与时间序刻意错位，r1/r5 并列。
    // r7 的 create_time 为 NULL：O34 修复前该行解码会 500，修复后应落末位且
    // createTime 序列化为空串。
    c.execute(
        "INSERT INTO x_org_role (id, name, create_time) VALUES \
         ('r1', 'a', '2025-11-01 00:00:00'), \
         ('r2', 'b', '2024-01-01 00:00:00'), \
         ('r3', 'c', '2025-12-01 00:00:00'), \
         ('r4', 'd', '2024-12-01 00:00:00'), \
         ('r5', 'e', '2025-11-01 00:00:00'), \
         ('r6', 'f', '2023-06-01 00:00:00'), \
         ('r7', 'g', NULL)",
        &[],
    )
    .await
    .expect("seed x_org_role");
    drop(c);

    let mut cursor = "0".to_string();
    let mut collected: Vec<String> = Vec::new();
    for _ in 0..16 {
        let res = organization_assemble_control_role_list_flag_next_count(
            Extension(pool.clone()),
            Path((cursor.clone(), "2".to_string())),
        )
        .await
        .expect("role next 不应失败");
        let page = ids_of(res.0.data.as_ref().expect("list 缺 data"));
        if page.is_empty() {
            break;
        }
        cursor = page.last().cloned().expect("页非空");
        collected.extend(page);
    }

    println!("timestamp keyset 实测（role next）: {collected:?}");
    let expected: Vec<String> = ["r3", "r5", "r1", "r4", "r2", "r6", "r7"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        collected, expected,
        "create_time DESC 的分页必须不漏不重、并列按 id DESC、NULL 落末"
    );

    // O34：NULL create_time 行单独再取一页，断言 200 且 createTime 为空串
    // （修复前 `row.get::<String>` 解码 NULL 会 500）。
    let res = organization_assemble_control_role_list_flag_next_count(
        Extension(pool.clone()),
        Path(("r6".to_string(), "2".to_string())),
    )
    .await
    .expect("role next(NULL 行) 不应失败");
    let data = res.0.data.as_ref().expect("list 缺 data");
    assert_eq!(
        ids_of(data),
        vec!["r7".to_string()],
        "游标 r6 的下一页应只有 NULL create_time 的 r7"
    );
    assert_eq!(
        data[0].get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须解码为空串而不是 500"
    );

    let res = role_list_flag_prev_count(
        Extension(pool.clone()),
        Path(("r2".to_string(), "2".to_string())),
    )
    .await
    .expect("role prev 不应失败");
    let prev = ids_of(res.0.data.as_ref().expect("list 缺 data"));
    println!("timestamp keyset prev 实测（游标 r2）: {prev:?}");
    assert_eq!(
        prev,
        vec!["r1".to_string(), "r4".to_string()],
        "prev 必须取游标之前的最近两条，并按 create_time DESC 返回"
    );

    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {SCRATCH} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}
