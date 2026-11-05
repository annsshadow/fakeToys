// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O29：`cal_calendar.is_public` 是可空 BOOLEAN，读面用非 Option `row.get::<_, bool>`
//! 解码——只要出现一行 `is_public IS NULL`，tokio-postgres 的 `Row::get` 就会
//! **panic**（FromSql for bool 不接受 NULL），请求线程崩溃而不是返回业务响应。
//!
//! 本测试在专用 scratch 库 `oa_par_cal_panic` 里造两行：
//! - `probe-null`：`is_public IS NULL` + `description/source IS NULL`，`status='OPEN'`；
//! - `probe-public`：`is_public=true` + `description/source IS NULL`。
//!
//! 然后直调三个读 handler，断言「不 panic + NULL 按 false 解码 + 部分响应不丢数据」。

use axum::extract::{Extension, Path};
use calendar::{calendar_get, calendar_list_my, calendar_list_public};
use deadpool_postgres::Pool;

/// 探针专用 scratch 库（每轮重建，跑完 DROP）。
const SCRATCH_DB: &str = "oa_par_cal_panic";

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
async fn null_is_public_row_does_not_panic_read_handlers() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO cal_calendar \
         (id, name, type, target, color, description, source, createor, is_public, status) \
         VALUES ('probe-null', 'Probe Null', 'PERSON', '', '', NULL, NULL, 'probe', NULL, 'OPEN')",
        &[],
    )
    .await
    .expect("seed probe-null");
    c.execute(
        "INSERT INTO cal_calendar \
         (id, name, type, target, color, description, source, createor, is_public, status) \
         VALUES ('probe-public', 'Probe Public', 'UNIT', '', '', NULL, NULL, 'probe', true, 'OPEN')",
        &[],
    )
    .await
    .expect("seed probe-public");
    drop(c);

    // list_my：包含 NULL 行，改前在这里 panic。
    let my = calendar_list_my(Extension(pool.clone()))
        .await
        .expect("calendar_list_my 不应失败/panic");
    let data = my.0.data.expect("list_my 缺 data");
    let items: Vec<&calendar::CalendarItem> = data
        .my_calendars
        .iter()
        .chain(data.unit_calendars.iter())
        .collect();
    // 规模下限：两行都必须被读到（person 一支 + unit 一支）。
    assert!(
        items.len() >= 2,
        "list_my 只返回 {} 行，少于种子行数 2",
        items.len()
    );
    let null_row = items
        .iter()
        .find(|i| i.id == "probe-null")
        .expect("list_my 丢了 is_public IS NULL 的行");
    assert!(
        !null_row.is_public,
        "is_public IS NULL 必须按 false 解码，实际为 true"
    );
    assert_eq!(
        null_row.description, None,
        "description IS NULL 应解为 None"
    );

    // list_public：只收 is_public = true，且不因 NULL 行报错。
    let public = calendar_list_public(Extension(pool.clone()))
        .await
        .expect("calendar_list_public 不应失败/panic");
    let public_items = public.0.data.expect("list_public 缺 data");
    assert!(
        public_items.iter().any(|i| i.id == "probe-public"),
        "公开日历未出现在 list_public"
    );
    assert!(
        !public_items.iter().any(|i| i.id == "probe-null"),
        "is_public IS NULL 的行不得出现在公开列表"
    );

    // calendar_get：公开行可读，NULL 行按私有处理 → 无会话 401（不 panic）。
    let public_get = calendar_get(
        Extension(pool.clone()),
        None,
        Path("probe-public".to_string()),
    )
    .await
    .expect("公开日历 calendar_get 应成功");
    assert_eq!(public_get.0.data.expect("get 缺 data").description, None);

    let null_get = calendar_get(
        Extension(pool.clone()),
        None,
        Path("probe-null".to_string()),
    )
    .await;
    assert!(
        null_get.is_err(),
        "is_public IS NULL 应视为私有，无会话必须拒绝"
    );

    drop_scratch().await;
}
