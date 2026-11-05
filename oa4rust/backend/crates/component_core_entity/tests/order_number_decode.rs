// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O31：`CPT_COMPONENT.order_number` 在实库里是 **TEXT**（可空），
//! `component_list_all` / `component_get` 却用 `row.get::<_, Option<i32>>` 解码。
//! tokio-postgres 的 `FromSql for i32` 不接受 text，命中一行带值的记录就 panic。
//!
//! 本测试在专用 scratch 库 `oa_par_component_ord` 里造两行（`order_number='10'`
//! 与 `order_number IS NULL`），直调两个读 handler，断言「不 panic + 数值解析正确
//! + NULL 不输出 orderNumber」。

use axum::extract::{Extension, Path};
use component_core_entity::{component_get, component_list_all};
use deadpool_postgres::Pool;

const SCRATCH_DB: &str = "oa_par_component_ord";

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
async fn text_order_number_does_not_panic_read_handlers() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO cpt_component \
         (id, name, title, type, visible, order_number, path, icon_path) \
         VALUES \
         ('probe-num', 'probe-num', '数值排序', 'component', TRUE, '10', 'p1', 'i1'), \
         ('probe-null', 'probe-null', '空排序', 'component', FALSE, NULL, 'p2', 'i2')",
        &[],
    )
    .await
    .expect("seed cpt_component");
    drop(c);

    // 1) component_list_all：命中两行，改前在第一行就 panic。
    let res = component_list_all(Extension(pool.clone()))
        .await
        .expect("component_list_all 不应失败/panic");
    let data = res.0.data.expect("list 缺 data");
    let rows = data.as_array().expect("list data 必须是数组");
    assert!(
        rows.len() >= 2,
        "规模下限：必须返回两行，实际 {}",
        rows.len()
    );

    let num = rows
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("probe-num"))
        .expect("必须包含 probe-num");
    assert_eq!(
        num.get("orderNumber").and_then(|v| v.as_i64()),
        Some(10),
        "TEXT '10' 必须解析成数字 10"
    );

    let null = rows
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("probe-null"))
        .expect("必须包含 probe-null");
    assert!(
        null.get("orderNumber").is_none(),
        "order_number IS NULL 时不得输出 orderNumber"
    );

    // 2) component_get：单行路径同样不能 panic。
    let one = component_get(Extension(pool.clone()), Path("probe-num".to_string()))
        .await
        .expect("component_get 不应失败/panic");
    let data = one.0.data.expect("get 缺 data");
    assert_eq!(
        data.get("orderNumber").and_then(|v| v.as_i64()),
        Some(10),
        "component_get 的 orderNumber 必须是 10"
    );

    drop_scratch().await;
}
