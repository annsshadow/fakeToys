// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O32：`x_component.order_number` 在实库里是 **TEXT**，`ORDER BY order_number ASC LIMIT 50`
//! 走字典序。表内一旦超过 9 个组件，**前 50 行的窗口就会选错行**（例如 '6'..'9' 被排到
//! 50 名以后而整块漏掉）——这不是「顺序不好看」，是列表页直接少数据。
//!
//! 本测试在 scratch 库 `oa_par_xcomp_ord_sort` 里造 60 行（order_number '1'..'60'），
//! 断言 `list_all` 返回的前 50 行恰好是数值 1..=50。既有实库只有 1 行（order_number='1'），
//! **现有数据完全掩盖了这个缺陷**。

use axum::extract::Extension;
use component::list_all;
use deadpool_postgres::Pool;

const SCRATCH_DB: &str = "oa_par_xcomp_ord_sort";

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
async fn text_order_number_limit_window_selects_numerically_first_50() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    // 051 迁移会给 x_component 种一行 order_number='1' 的 test-id，先清空保证窗口规模精确。
    c.execute("DELETE FROM x_component", &[])
        .await
        .expect("清空 x_component");
    c.execute(
        "INSERT INTO x_component \
         (id, name, title, type, visible, order_number, path, icon_path) \
         SELECT 'c' || g, 'c' || g, '组件' || g, 'component', TRUE, g::text, 'p', 'i' \
         FROM generate_series(1, 60) AS g",
        &[],
    )
    .await
    .expect("seed x_component 60 行");
    drop(c);

    let res = list_all(Extension(pool.clone()))
        .await
        .expect("list_all 不应失败");
    let data = res.0.data.expect("list 缺 data");
    let rows = data.as_array().expect("list data 必须是数组");
    assert_eq!(rows.len(), 50, "规模下限：LIMIT 50 必须返回 50 行");
    let actual: Vec<i64> = rows
        .iter()
        .map(|r| r.get("orderNumber").and_then(|v| v.as_i64()).unwrap_or(-1))
        .collect();
    let expected: Vec<i64> = (1..=50).collect();
    println!("实际前 50: {actual:?}");
    println!("期望前 50: {expected:?}");
    assert_eq!(
        actual, expected,
        "LIMIT 50 的窗口必须按数值取最小的 50 个 order_number"
    );

    drop_scratch().await;
}
