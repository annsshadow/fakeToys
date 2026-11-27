// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R610a：message 已读标记两端点身份盲 IDOR 写回归。
//! 真父版 alice 凭 bob 裸 xid 翻 xconsumed=true；修复版按 xconsumer 归属门 403。

use axum::{body::Body, extract::Extension, http::Request, Router};
use deadpool_postgres::{Manager, Pool};
use message::routes::router;
use serde_json::Value;
use tower::ServiceExt;

const SCRATCH_SINGLE: &str = "oa_r610a_message_owner_single";
const SCRATCH_MARK: &str = "oa_r610a_message_owner_mark";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config = url.parse().expect("DATABASE_URL");
    cfg.dbname(dbname);
    let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(db: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(&format!("DROP DATABASE IF EXISTS {db} WITH (FORCE)"), &[])
        .await
        .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {db}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pool_to(db);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 迁移");
    pool
}

async fn drop_scratch(db: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(&format!("DROP DATABASE IF EXISTS {db} WITH (FORCE)"), &[])
            .await;
    }
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(concat!(
        "INSERT INTO x_msg_message (id, title, body, type, consumer, xconsumer, is_read, xid, xconsumed) VALUES ",
        "('m-bob','b','b','read','bob','bob',false,'b-xid',false), ",
        "('m-alice','a','a','read','alice','alice',false,'a-xid',false); ",
        "INSERT INTO auth_person (id, unique_id, name, password_hash) ",
        "VALUES ('r610a-admin','r610a-admin','r610a admin','x'); ",
        "INSERT INTO auth_role (id, name, deleted_at, disable) ",
        "VALUES ('r610a-role','admin',NULL,false); ",
        "INSERT INTO auth_person_role (person_id, role_id, unit_id) ",
        "VALUES ('r610a-admin','r610a-role','r610a-unit');"
    )).await.expect("seed");
}

fn session(person_unique: &str) -> shared::session::Session {
    shared::testing::test_session(person_unique)
}

async fn call(app: &Router, who: Option<&str>, uri: &str) -> (u16, Value) {
    let mut app = app.clone();
    if let Some(who) = who {
        app = app.layer(Extension(session(who)));
    }
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");
    let status = resp.status().as_u16();
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, json)
}

async fn consumed(pool: &Pool, xid: &str) -> Option<bool> {
    let c = pool.get().await.expect("conn");
    let row = c
        .query_opt(
            "SELECT xconsumed FROM x_msg_message WHERE xid = $1",
            &[&xid],
        )
        .await
        .expect("query");
    row.map(|r| r.get::<_, bool>("xconsumed"))
}

fn flag(v: &Value, key: &str) -> bool {
    v.get("data")
        .and_then(|d| d.get(key))
        .and_then(|b| b.as_bool())
        .unwrap_or(false)
}

async fn assert_paths(app: &Router, pool: &Pool, uri: &str, key: &str) {
    // 越权：alice 凭 bob xid 不得翻 bob.xconsumed
    let (st, _) = call(app, Some("alice"), uri).await;
    assert_eq!(st, 403, "越权必须 403");
    assert_eq!(
        consumed(pool, "b-xid").await,
        Some(false),
        "bob 消息不得被标记"
    );

    // 裸调：401
    let (st, _) = call(app, None, uri).await;
    assert_eq!(st, 401, "无会话必须 401");

    // 本人：200 + flag=true
    let own_uri = uri.replace("b-xid", "a-xid");
    let (st, v) = call(app, Some("alice"), &own_uri).await;
    assert_eq!(st, 200, "本人操作必须 200");
    assert!(flag(&v, key), "本人操作必须返回 true");
    assert_eq!(
        consumed(pool, "a-xid").await,
        Some(true),
        "本人消息必须被标记"
    );

    // admin 代操作：200
    let (st, v) = call(app, Some("r610a-admin"), uri).await;
    assert_eq!(st, 200, "admin 代操作必须 200");
    assert!(flag(&v, key), "admin 代操作必须返回 true");

    // 不存在 xid：200 + flag=false，不泄露存在性
    let missing_uri = uri.replace("b-xid", "no-such-xid");
    let (st, v) = call(app, Some("alice"), &missing_uri).await;
    assert_eq!(st, 200, "不存在记录必须 200 不泄露");
    assert!(!flag(&v, key), "不存在记录必须 false");
}

#[tokio::test]
async fn update_single_owner_guard() {
    let pool = fresh_scratch(SCRATCH_SINGLE).await;
    seed(&pool).await;
    let app = router(pool.clone());
    assert_paths(
        &app,
        &pool,
        "/api/message/consume/b-xid/type/read",
        "updated",
    )
    .await;
    drop_scratch(SCRATCH_SINGLE).await;
}

#[tokio::test]
async fn mark_read_owner_guard() {
    let pool = fresh_scratch(SCRATCH_MARK).await;
    seed(&pool).await;
    let app = router(pool.clone());
    assert_paths(&app, &pool, "/api/message/mark_read/b-xid", "markedRead").await;
    drop_scratch(SCRATCH_MARK).await;
}

#[tokio::test]
async fn source_guard_gate_present_and_update_sql_scoped() {
    let src = include_str!("../src/lib.rs");
    assert!(
        src.contains("let sess = require_session(session)?;"),
        "必须保留会话值"
    );
    assert_eq!(
        src.matches("SELECT xconsumer FROM x_msg_message WHERE xid = $1")
            .count(),
        2,
        "两端点必须按 xconsumer 查归属"
    );
    assert_eq!(
        src.matches("shared::middleware::require_owner(&pool, &sess, &owner).await?;")
            .count(),
        2,
        "两端点必须归属门"
    );
    assert_eq!(
        src.matches("SET xconsumed = true WHERE xid = $1 AND xconsumer = $2")
            .count(),
        2,
        "两端点 UPDATE 必须带 owner 兜底"
    );
    assert!(
        !src.contains("SET xconsumed = true WHERE xid = $1\""),
        "旧裸 UPDATE 不得残留"
    );
}
