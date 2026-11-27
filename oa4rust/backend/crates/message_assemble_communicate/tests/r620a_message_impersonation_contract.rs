// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R620a: message_assemble_communicate 写端点身份绑定回归。
//! 真父: connector_create 的 body.person 直接落 x_message_instant.person,
//!       任意登录人可伪造他人身份发消息; ws_create 的 body.sender 同理。
//! 修复: person/sender 一律取会话人, body 不再自证; 身份门在首个 DB 写前。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use message_assemble_communicate::router;
use serde_json::{json, Value};
use shared::testing::test_session;
use tower::ServiceExt;

fn pg_pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config = url.parse().expect("DATABASE_URL");
    cfg.dbname(dbname);
    let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pg_pool_to("postgres");
    let c = admin.get().await.expect("admin conn");
    c.execute(
        &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {dbname}"), &[])
        .await
        .expect("create scratch");
    drop(c);
    let pool = pg_pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch migration");
    pool
}

async fn call(app: &Router, path: &str, who: Option<&str>, body: Value) -> (StatusCode, String) {
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = match who {
        Some(p) => {
            app.clone()
                .layer(Extension(test_session(p)))
                .oneshot(req)
                .await
        }
        None => app.clone().oneshot(req).await,
    }
    .expect("oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("body");
    (status, String::from_utf8_lossy(&bytes).to_string())
}

async fn scalar(
    pool: &Pool,
    sql: &str,
    params: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
) -> String {
    let c = pool.get().await.expect("scalar conn");
    let row = c.query_opt(sql, params).await.expect("scalar query");
    row.and_then(|r| r.get::<_, Option<String>>(0))
        .unwrap_or_default()
}

#[tokio::test]
async fn r620a_message_identity_is_bound_to_session() {
    let dbname = "r620a_message_contract";
    let pool = fresh_scratch(dbname).await;
    let app = router(pool.clone());
    let path = "/api/message/assemble/communicate/connector";

    // 裸请求: 无会话必须 401。
    let (st, _) = call(&app, path, None, json!({"person":"r620a-bob","body":"x"})).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "裸请求 401");

    // 真父 RED: alice 以 body.person 伪造 bob 发消息 -> 200 且 DB 落 bob 名下。
    // 修复后: 200 但 person 恒为会话人 alice, bob 名下不得出现。
    let (st, _) = call(
        &app,
        path,
        Some("r620a-alice"),
        json!({"person":"r620a-bob","type":"t","title":"ti","body":"b"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "alice 本人发消息 200");
    let bob = scalar(
        &pool,
        "SELECT person FROM x_message_instant WHERE body=$1 LIMIT 1",
        &[&"\"b\""],
    )
    .await;
    assert_eq!(bob, "r620a-alice", "person 必须绑定会话人 alice, got {bob}");
    let forged = scalar(
        &pool,
        "SELECT COUNT(*)::text FROM x_message_instant WHERE person=$1",
        &[&"r620a-bob"],
    )
    .await;
    assert_eq!(forged, "0", "bob 名下不得被伪造写入, got {forged}");
}

#[test]
fn r620a_message_source_guard() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    assert!(
        src.contains("let person = sess.person_unique.clone()"),
        "connector person 必须取会话身份"
    );
    assert!(
        src.contains("let sender = sess.person_unique.clone()"),
        "ws sender 必须取会话身份"
    );
}
