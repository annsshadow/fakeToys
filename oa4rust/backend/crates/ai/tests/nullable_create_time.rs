// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O36-C3：`ai` crate 中 3 处 `create_time` 解码。
//!
//! 迁移证据：
//! - `026_create_ai_app_conversation.sql:18` `x_ai_conversation.create_time TIMESTAMP WITHOUT TIME ZONE`（可空）；
//! - `034_create_x_schema_tables.sql:9` `x_ai_clue.create_time TIMESTAMPTZ`（可空）；
//! - `034_create_x_schema_tables.sql:30` `x_ai_completion.create_time TIMESTAMPTZ`（可空）。
//!
//! 三处 SQL 均已 `::text`，解码改为 `Option<String>` 兜底空串。

use axum::extract::{Extension, Path};
use deadpool_postgres::Pool;
use serde_json::Value;

const SCRATCH_DB: &str = "oa_o36_ai_misc";

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

fn assert_empty(obj: &Value, label: &str) {
    assert_eq!(
        obj.get("createTime").and_then(|v| v.as_str()),
        Some(""),
        "{label}: NULL create_time 必须归一为空串，实收: {:?}",
        obj.get("createTime")
    );
}

fn find<'a>(items: &'a [Value], id: &str, label: &str) -> &'a Value {
    items
        .iter()
        .find(|r| r.get("id").and_then(|v| v.as_str()) == Some(id))
        .unwrap_or_else(|| panic!("{label}: {id} 必须在结果中"))
}

#[tokio::test]
async fn ai_readers_decode_null_create_time() {
    let pool = fresh_scratch().await;

    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_ai_conversation (id, title, user_id, create_time) VALUES \
         ('o36-conv-null', 'A_null', 'p1', NULL), \
         ('o36-conv-time', 'B_time', 'p1', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_ai_conversation");
    c.execute(
        "INSERT INTO x_ai_clue (id, title, person, create_time) VALUES \
         ('o36-clue-null', 'A_null', 'p1', NULL), \
         ('o36-clue-time', 'B_time', 'p1', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_ai_clue");
    c.execute(
        "INSERT INTO x_ai_completion (id, \"clueId\", name, create_time) VALUES \
         ('o36-comp-null', 'o36-clue-null', 'A_null', NULL), \
         ('o36-comp-time', 'o36-clue-null', 'B_time', '2025-01-01 08:00:00')",
        &[],
    )
    .await
    .expect("seed x_ai_completion");
    drop(c);

    // 1) conversation_list。
    let res = ai::conversation_list(Extension(pool.clone()))
        .await
        .expect("conversation_list 不得 500");
    let arr = res
        .0
        .data
        .expect("缺 data")
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert_empty(
        find(&arr, "o36-conv-null", "conversation_list"),
        "conversation_list",
    );

    // 2) chat_list_paging。
    let res = ai::chat_list_paging(Extension(pool.clone()), Path((1, 20)))
        .await
        .expect("chat_list_paging 不得 500");
    let arr = res
        .0
        .data
        .expect("缺 data")
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert_empty(
        find(&arr, "o36-clue-null", "chat_list_paging"),
        "chat_list_paging",
    );

    // 3) chat_list_completion_paging。
    let res = ai::chat_list_completion_paging(
        Extension(pool.clone()),
        Path(("o36-clue-null".to_string(), 1, 20)),
    )
    .await
    .expect("chat_list_completion_paging 不得 500");
    let arr = res
        .0
        .data
        .expect("缺 data")
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert_empty(
        find(&arr, "o36-comp-null", "chat_list_completion_paging"),
        "chat_list_completion_paging",
    );
    // 顺带发现：同表 `generateType` 为可空 VARCHAR（034:10），原解码非 Option
    // 会在 NULL 行 panic；本轮一并兜底并断言。`clueId`（034:6）虽同样可空，
    // 但该端点按 `"clueId" = $1` 过滤，NULL 行走不到解码，故不改（台账记为
    // 「走不到解码」）。
    let comp = find(&arr, "o36-comp-null", "chat_list_completion_paging");
    assert_eq!(
        comp.get("generateType").and_then(|v| v.as_str()),
        Some(""),
        "NULL generateType 必须归一为空串，实收: {:?}",
        comp.get("generateType")
    );

    drop_scratch().await;
}
