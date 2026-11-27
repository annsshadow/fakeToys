// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R607a 契约：attendance detail 的 recive / reciveSingle 两裸写端点必须校验目标记录归属人。
//! 核心用例：alice 凭 bob 明细 id 越权接收必须 403 且 bob.received 不变（真父必 RED）；
//! 本人成功、admin 代操作成功、不存在 id 保持 received=0、裸请求 401。

use std::path::PathBuf;

use attendance_assemble_control::attendance_assemble_control_router;
use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::session::Session;
use tower::ServiceExt;

const RECIVE: &str = "/api/attendance/assemble/control/attendancedetail/recive";
const RECIVE_SINGLE: &str = "/api/attendance/assemble/control/attendancedetail/reciveSingle";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust_test".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
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
    let pool = pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch 迁移");
    pool
}

async fn drop_scratch(dbname: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.batch_execute(concat!(
        "INSERT INTO x_attendance_detail (id, person_id, date, status, received) VALUES ",
        "('d-alice', 'alice', '2026-01-01', 'normal', false), ",
        "('d-bob', 'bob', '2026-01-02', 'normal', false);",
        "INSERT INTO auth_person (id, unique_id, name, password_hash) ",
        "VALUES ('r607a-admin', 'r607a-admin', 'r607a admin', 'x');",
        "INSERT INTO auth_role (id, name, deleted_at, disable) ",
        "VALUES ('r607a-admin-role', 'admin', NULL, false);",
        "INSERT INTO auth_person_role (person_id, role_id, unit_id) ",
        "VALUES ('r607a-admin', 'r607a-admin-role', 'r607a-unit');"
    ))
    .await
    .expect("seed");
    drop(c);
}

fn session(person: &str) -> Session {
    shared::testing::test_session(person)
}

async fn call(app: Router, who: Option<&str>, uri: &str, id: &str) -> (StatusCode, Value) {
    let mut app = app;
    if let Some(person) = who {
        app = app.layer(Extension(session(person)));
    }
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(json!({"id": id}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

async fn received(pool: &Pool, id: &str) -> bool {
    let c = pool.get().await.expect("连接");
    let row = c
        .query_one(
            "SELECT received FROM x_attendance_detail WHERE id = $1",
            &[&id],
        )
        .await
        .expect("查询 received");
    row.get::<_, bool>("received")
}

fn received_count(v: &Value) -> i64 {
    v.get("data")
        .and_then(|d| d.get("received").or(Some(d)))
        .and_then(|d| d.as_i64())
        .unwrap_or(-1)
}

async fn run_endpoint_checks(uri: &str, db: &str) {
    let pool = fresh_scratch(db).await;
    seed(&pool).await;
    let app = || attendance_assemble_control_router(pool.clone());

    // 越权：alice 凭 bob 的 id 调端点 → 403，bob.received 保持 false。
    let (st, _) = call(app(), Some("alice"), uri, "d-bob").await;
    assert_eq!(st, StatusCode::FORBIDDEN, "越权必须 403");
    assert!(!received(&pool, "d-bob").await, "越权不得改 bob.received");

    // 裸请求 → 401。
    let (st, _) = call(app(), None, uri, "d-alice").await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "无会话必须 401");

    // 本人合法：alice 收本人记录 → 200，received=1。
    let (st, v) = call(app(), Some("alice"), uri, "d-alice").await;
    assert_eq!(st, StatusCode::OK, "本人合法接收必须 200");
    assert!(
        received(&pool, "d-alice").await,
        "合法接收应置 received=true"
    );
    assert_eq!(received_count(&v), 1, "响应 received 计数应为 1");

    // admin 代操作 bob → 200。
    let (st, _) = call(app(), Some("r607a-admin"), uri, "d-bob").await;
    assert_eq!(st, StatusCode::OK, "管理员代操作必须 200");
    assert!(
        received(&pool, "d-bob").await,
        "管理员代操作应置 received=true"
    );

    // 不存在的 id → 200 + received=0（维持原语义）。
    let (st, v) = call(app(), Some("alice"), uri, "missing-id").await;
    assert_eq!(st, StatusCode::OK, "不存在 id 维持原成功语义");
    assert_eq!(received_count(&v), 0, "不存在 id received 必须为 0");

    drop_scratch(db).await;
}

#[tokio::test]
async fn recive_endpoint_requires_owner_or_admin() {
    run_endpoint_checks(RECIVE, "oa_r607a_recive").await;
}

#[tokio::test]
async fn recive_single_endpoint_requires_owner_or_admin() {
    run_endpoint_checks(RECIVE_SINGLE, "oa_r607a_recive_single").await;
}

#[test]
fn source_guard_gate_present_and_update_sql_unchanged() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/lib.rs");
    let src = std::fs::read_to_string(&path).expect("读取 lib.rs");
    assert_eq!(
        src.matches("let sess = require_session(session)?;").count() >= 2,
        true,
        "两端点必须取回 sess"
    );
    assert_eq!(
        src.matches("shared::middleware::require_owner(&pool, &sess, &person_id).await?;")
            .count()
            >= 2,
        true,
        "两端点必须有归属门"
    );
    assert!(
        src.contains("SELECT person_id FROM x_attendance_detail WHERE id = $1"),
        "必须按 id 查归属人"
    );
    assert_eq!(
        src.matches(
            "UPDATE x_attendance_detail SET received = true, update_time = NOW() WHERE id = $1"
        )
        .count(),
        2,
        "UPDATE SQL 零改"
    );
}
