// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O30：`x_general_assemble_excel_result` / `x_general_assemble_general_file`
//! 的 `create_time` 是 **TIMESTAMPTZ**（可空），四个读 handler 却用
//! `row.get::<_, Option<String>>("create_time")` 解码——tokio-postgres 的
//! `FromSql for String` 不接受 timestamptz，只要命中一行就直接 panic。
//! 同时 `size` 是 **BIGINT 可空**，handler 用非 Option 的 `row.get::<_, i64>("size")`
//! 解码，`size IS NULL` 同样 panic。
//!
//! 本测试在专用 scratch 库 `oa_par_general_ts` 里造两行（时间列有值、size 为 NULL），
//! 直调四个 handler，断言「不 panic + createTime 有值 + size NULL 降级为 0」。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use general_assemble_control::{
    excel_result_flag, generalfile_download_flag_flag, generalfile_flag_flag,
    generalfile_flag_flag_binary_base64,
};

/// 探针专用 scratch 库（每轮重建，跑完 DROP）。
const SCRATCH_DB: &str = "oa_par_general_ts";

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
async fn timestamptz_and_null_size_do_not_panic_read_handlers() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_general_assemble_excel_result \
         (id, flag, result, creator, create_time) \
         VALUES ('probe-res', 'res-flag', 'ok', 'probe', NOW())",
        &[],
    )
    .await
    .expect("seed excel_result");
    c.execute(
        "INSERT INTO x_general_assemble_general_file \
         (id, name, flag, content, size, creator, create_time) \
         VALUES ('probe-file', 'probe.txt', 'file-flag', 'payload', NULL, 'probe', NOW())",
        &[],
    )
    .await
    .expect("seed general_file");
    drop(c);

    // 1) excel_result_flag：create_time TIMESTAMPTZ，改前 panic。
    let res = excel_result_flag(Extension(pool.clone()), Path("res-flag".to_string()))
        .await
        .expect("excel_result_flag 不应失败/panic");
    let data = res.0.data.expect("excel_result_flag 缺 data");
    let create_time = data
        .get("createTime")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    assert!(
        !create_time.is_empty(),
        "createTime 不得为空（TIMESTAMPTZ 必须能解码）"
    );

    // 2) 三个 general_file 读 handler：create_time TIMESTAMPTZ + size IS NULL。
    let download =
        generalfile_download_flag_flag(Extension(pool.clone()), Path("file-flag".to_string()))
            .await
            .expect("generalfile_download_flag_flag 不应失败/panic");
    let data = download.0.data.expect("download 缺 data");
    assert_eq!(
        data.get("size").and_then(|v| v.as_str()),
        Some("0"),
        "size IS NULL 必须降级为 0"
    );
    assert!(
        data.get("createTime")
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty()),
        "download 的 createTime 不得为空"
    );

    let view = generalfile_flag_flag(Extension(pool.clone()), Path("file-flag".to_string()))
        .await
        .expect("generalfile_flag_flag 不应失败/panic");
    assert!(view.0.data.is_some(), "generalfile_flag_flag 必须返回 data");

    let binary =
        generalfile_flag_flag_binary_base64(Extension(pool.clone()), Path("file-flag".to_string()))
            .await
            .expect("generalfile_flag_flag_binary_base64 不应失败/panic");
    assert!(
        binary.0.data.is_some(),
        "generalfile_flag_flag_binary_base64 必须返回 data"
    );

    drop_scratch().await;
}
