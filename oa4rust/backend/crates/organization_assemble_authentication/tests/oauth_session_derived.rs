// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! OAuth 授权码必须按会话签发：请求体 `personId` 不得决定落库归属。
//!
//! 缺陷复现：`oauth_generate_code` 曾把 body 的 `personId` 直接写进
//! `x_org_oauth_code.person_id`，任何登录用户都能用别人的 `personId` 生成授权码并落库。

use axum::extract::{Extension, Json};
use deadpool_postgres::{Manager, Pool};
use serde_json::json;

const SCRATCH: &str = "oa_oauth_session_derived";

fn pool_for(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(2).build().unwrap()
}

#[tokio::test]
async fn oauth_generate_code_person_id_comes_from_session() {
    let admin = pool_for("postgres");
    let conn = admin.get().await.expect("admin 连接");
    let _ = conn
        .execute(
            &format!("DROP DATABASE IF EXISTS {SCRATCH} WITH (FORCE)"),
            &[],
        )
        .await;
    conn.execute(&format!("CREATE DATABASE {SCRATCH}"), &[])
        .await
        .expect("create scratch");
    drop(conn);

    let pool = pool_for(SCRATCH);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 库迁移");

    // 攻击者会话提交受害者的 personId：落库归属必须是会话本人。
    let _ = organization_assemble_authentication::oauth_generate_code(
        Extension(pool.clone()),
        Some(Extension(shared::testing::test_session("audit-user"))),
        Json(json!({"clientId": "repro-oauth-client", "personId": "gate-victim"})),
    )
    .await
    .expect("handler");

    let conn = pool.get().await.expect("scratch 连接");
    let row = conn
        .query_one(
            "SELECT person_id FROM x_org_oauth_code WHERE client = $1 \
              ORDER BY created_at DESC LIMIT 1",
            &[&"repro-oauth-client"],
        )
        .await
        .expect("查询刚落的授权码");
    assert_eq!(
        row.get::<_, Option<String>>("person_id").as_deref(),
        Some("audit-user"),
        "body personId 不得决定授权码归属"
    );
}
