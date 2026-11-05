// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C3：`file::file_download` 的 `FILE_FILE.create_time` 解码。
//!
//! 迁移证据：`008_file_tables.sql:21` `create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP`
//! （可空，无 NOT NULL）；`011_normalize_schema.sql:21` 仅改名 `FILE_FILE → file_file`，
//! 无改类型/加约束。SQL 已 `::text`，解码改为 `Option<String>` 兜底空串。

use axum::extract::{Extension, Path};
use chrono::Duration;
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_file_misc";

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
async fn file_download_decodes_null_create_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, \
         mime_type, create_time) VALUES \
         ('o36-file-null', 'a.txt', 'p1', '', 'file', 'txt', 0, 'text/plain', NULL)",
        &[],
    )
    .await
    .expect("seed FILE_FILE");
    drop(c);

    let now = chrono::Local::now().naive_local();
    let session = shared::session::Session {
        token: "o36-token".to_string(),
        person_unique: "p1".to_string(),
        created_at: now,
        expires_at: now + Duration::hours(1),
    };

    let res = file::file_download(
        Extension(pool.clone()),
        Extension(session),
        Path("o36-file-null".to_string()),
    )
    .await
    .expect("file_download 不得 500");
    let obj: Value = res.0.data.expect("缺 data");
    assert_eq!(
        obj.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须归一为空串，实收: {:?}",
        obj.get("createTime")
    );

    drop_scratch().await;
}
