// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O32 同型缺陷：`corr_c_correlation.order_number` 实列是 **TEXT**（探针实测），
//! 两个列表 handler 却 ① `ORDER BY order_number`（字典序，跨 9 错序）
//! ② `row.get::<_, Option<i32>>` 解码（命中带值行直接 panic）。
//!
//! 本测试在 scratch 库 `oa_par_corr_ord_sort` 里造 '2'/'10'/'3'/NULL/'abc'，
//! 直调两个读 handler：改前 panic，改后按数值升序且不可解析值不输出 orderNumber。

use axum::extract::Extension;
use correlation::routes::{list_cms_correlations, list_process_platform_correlations};
use deadpool_postgres::Pool;

const SCRATCH_DB: &str = "oa_par_corr_ord_sort";

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

fn rows_of(data: &serde_json::Value) -> Vec<(String, Option<i64>)> {
    data.as_array()
        .expect("list data 必须是数组")
        .iter()
        .map(|r| {
            (
                r.get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                r.get("orderNumber").and_then(|v| v.as_i64()),
            )
        })
        .collect()
}

#[tokio::test]
async fn text_order_number_sorts_numerically_in_both_lists() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM corr_c_correlation", &[])
        .await
        .expect("清空");
    c.execute(
        "INSERT INTO corr_c_correlation \
         (id, from_bundle, target_bundle, person, site, order_number, from_type) VALUES \
         ('cms-2',   'b1', 't1', 'p', 's', '2',   'cms'), \
         ('cms-10',  'b2', 't2', 'p', 's', '10',  'cms'), \
         ('cms-3',   'b3', 't3', 'p', 's', '3',   'cms'), \
         ('cms-null','b4', 't4', 'p', 's', NULL,  'cms'), \
         ('cms-bad', 'b5', 't5', 'p', 's', 'abc', 'cms'), \
         ('pp-2',    'b6', 't6', 'p', 's', '2',   'processPlatform'), \
         ('pp-10',   'b7', 't7', 'p', 's', '10',  'processPlatform'), \
         ('pp-3',    'b8', 't8', 'p', 's', '3',   'processPlatform')",
        &[],
    )
    .await
    .expect("seed corr_c_correlation");
    drop(c);

    let cms = list_cms_correlations(Extension(pool.clone()))
        .await
        .expect("cms 列表不应失败");
    let cms_data = cms.0.data.expect("cms 缺 data");
    let cms_rows = rows_of(&cms_data);
    assert!(cms_rows.len() >= 5, "规模下限：cms 必须 5 行");
    let cms_expected: Vec<(String, Option<i64>)> = vec![
        ("cms-2".into(), Some(2)),
        ("cms-3".into(), Some(3)),
        ("cms-10".into(), Some(10)),
        ("cms-bad".into(), None),
        ("cms-null".into(), None),
    ];
    println!("cms 实际: {cms_rows:?}");
    assert_eq!(cms_rows, cms_expected, "cms 列表必须按数值升序");

    let pp = list_process_platform_correlations(Extension(pool.clone()))
        .await
        .expect("processPlatform 列表不应失败");
    let pp_data = pp.0.data.expect("pp 缺 data");
    let pp_rows = rows_of(&pp_data);
    assert!(pp_rows.len() >= 3, "规模下限：processPlatform 必须 3 行");
    let pp_expected: Vec<(String, Option<i64>)> = vec![
        ("pp-2".into(), Some(2)),
        ("pp-3".into(), Some(3)),
        ("pp-10".into(), Some(10)),
    ];
    println!("pp 实际: {pp_rows:?}");
    assert_eq!(pp_rows, pp_expected, "processPlatform 列表必须按数值升序");

    drop_scratch().await;
}
