// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O35：meeting 域 `create_time` 均为 **可空 TIMESTAMP**（006 migration：
//! `DEFAULT CURRENT_TIMESTAMP`，无 NOT NULL）。本 crate 统一用
//! `to_char(create_time, ...)` 读取，NULL 经 `to_char` 仍是 NULL；修复前
//! `row.get::<_, String>` 解码 NULL 会 panic → 500。
//!
//! 专用 scratch 库 `oa_o35_meeting_ac`，造「有值 + NULL」行后直调
//! `x_meeting_building` 与 `x_meeting` 两条读路径。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;

const SCRATCH_DB: &str = "oa_o35_meeting_ac";

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
async fn null_create_time_does_not_500_on_meeting_read_paths() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_meeting_building (id, name, create_time) VALUES \
         ('o35-bld-null', 'A_null', NULL), \
         ('o35-bld-time', 'B_time', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_meeting_building");
    // start_time/end_time 是 NOT NULL（006），只置 create_time 为 NULL。
    // 读路径已加归属谓词（本人 OR 管理员）：本用例聚焦 create_time 解码，
    // 种上 creator 让会话本人可读，避免被 NULL 归属 fail-closed 拦截。
    c.execute(
        "INSERT INTO x_meeting (id, title, start_time, end_time, create_time, creator) VALUES \
         ('o35-meeting-null', 'T', '2025-01-01 08:00:00', '2025-01-01 09:00:00', NULL, 'u-o35-owner')",
        &[],
    )
    .await
    .expect("seed x_meeting");
    drop(c);

    // 路径一：building_list（to_char on x_meeting_building.create_time）
    let res = meeting_assemble_control::building_list(Extension(pool.clone()))
        .await
        .expect("building_list 遇 NULL create_time 不得 500");
    let data = res.0.data.expect("building list 缺 data");
    let items = data.as_array().expect("data 必须是数组");
    let null_row = items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o35-bld-null"))
        .expect("NULL 行必须在列表中");
    assert_eq!(
        null_row.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "NULL create_time 必须归一为空串"
    );
    let timed_row = items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some("o35-bld-time"))
        .expect("有值行必须在列表中");
    assert!(
        timed_row
            .get("createTime")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.starts_with("2025-01-01")),
        "有值行的 createTime 格式不得被改动，实收: {:?}",
        timed_row.get("createTime")
    );

    // 路径二：meeting_id（to_char on x_meeting.create_time）
    let res = meeting_assemble_control::meeting_id(
        Extension(pool.clone()),
        Some(Extension(shared::testing::test_session("u-o35-owner"))),
        Path("o35-meeting-null".to_string()),
    )
    .await
    .expect("meeting_id 遇 NULL create_time 不得 500");
    let data = res.0.data.expect("meeting detail 缺 data");
    assert_eq!(
        data.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "x_meeting.create_time 为 NULL 时 createTime 必须为空串"
    );

    drop_scratch().await;
}
