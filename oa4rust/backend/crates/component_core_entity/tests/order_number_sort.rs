// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O32：`CPT_COMPONENT.order_number` 在实库里是 **TEXT**，`ORDER BY order_number ASC`
//! 走的是字典序（`'10' < '2'`），一旦组件序号跨过 9 顺序就错。
//!
//! 实库现状（探针实测）：`cpt_component` 0 行、`x_component` 仅 1 行且 order_number='1'，
//! 因此线上看不到错序——**现有数据掩盖了缺陷**。本测试在专用 scratch 库
//! `oa_par_cpt_ord_sort` 里造跨 9 的真实数据，直调 `component_list_all`，断言按数值升序。
//!
//! 期望口径与 O31 一致：先裁空格，按整数语法（可选正负号、至多 10 位数字）解析，
//! 且落在 i32 范围内才算可解析；NULL / 非数字 / 溢出统一排到末尾（NULLS LAST）。

use axum::extract::Extension;
use component_core_entity::component_list_all;
use deadpool_postgres::Pool;

const SCRATCH_DB: &str = "oa_par_cpt_ord_sort";

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
async fn text_order_number_sorts_numerically_across_nine() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute("DELETE FROM cpt_component", &[])
        .await
        .expect("清空 cpt_component");
    c.execute(
        "INSERT INTO cpt_component \
         (id, name, title, type, visible, order_number, path, icon_path) \
         VALUES \
         ('n2',     'n2',     '序号2',    'component', TRUE, '2',          'p2', 'i2'), \
         ('n10',    'n10',    '序号10',   'component', TRUE, '10',         'p10', 'i10'), \
         ('n3',     'n3',     '序号3',    'component', TRUE, '3',          'p3', 'i3'), \
         ('n-plus', 'n-plus', '带正号7',   'component', TRUE, '+7',         'p7', 'i7'), \
         ('n-space','n-space','空格5',     'component', TRUE, ' 5 ',        'p5', 'i5'), \
         ('n-null', 'n-null', '空序号',     'component', TRUE, NULL,         'pn', 'in'), \
         ('n-bad',  'n-bad',  '非数字',     'component', TRUE, 'abc',        'pb', 'ib'), \
         ('n-over', 'n-over', '溢出i32',   'component', TRUE, '2147483648', 'po', 'io')",
        &[],
    )
    .await
    .expect("seed cpt_component");
    drop(c);

    let res = component_list_all(Extension(pool.clone()))
        .await
        .expect("component_list_all 不应失败");
    let data = res.0.data.expect("list 缺 data");
    let actual = ids_of(&data);
    assert!(
        actual.len() >= 8,
        "规模下限：必须返回 8 行，实际 {}",
        actual.len()
    );
    let expected: Vec<String> = [
        "n2", "n3", "n-space", "n-plus", "n10", "n-bad", "n-null", "n-over",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    println!("实际顺序: {actual:?}");
    println!("期望顺序: {expected:?}");
    assert_eq!(actual, expected, "order_number 必须按数值升序");

    // 读面数值同样必须与 O31 口径一致。
    let num = |id: &str| -> Option<i64> {
        data.as_array()
            .unwrap()
            .iter()
            .find(|r| r.get("id").and_then(|v| v.as_str()) == Some(id))
            .and_then(|r| r.get("orderNumber").and_then(|v| v.as_i64()))
    };
    assert_eq!(num("n10"), Some(10), "TEXT '10' 解析为 10");
    assert_eq!(num("n-plus"), Some(7), "'+7' 解析为 7");
    assert_eq!(num("n-space"), Some(5), "' 5 ' 去空格后解析为 5");
    assert_eq!(num("n-bad"), None, "非数字不输出 orderNumber");
    assert_eq!(num("n-null"), None, "NULL 不输出 orderNumber");
    assert_eq!(num("n-over"), None, "超出 i32 不输出 orderNumber");

    drop_scratch().await;
}
