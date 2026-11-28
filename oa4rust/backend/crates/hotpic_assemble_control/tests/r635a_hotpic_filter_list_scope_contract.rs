// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R635a: hotpic_assemble_control 用户热图分页列表身份盲读真修复回归。
//! 真父: handler 无 session 提取、SQL 无可见范围谓词 -> 任意登录人可读全表含他人私有行。
//! 修复: handler 加 Option<Extension<Session>> + require_session 前置；读口径 = 本人 OR
//!       全局行(application 与 info_id 皆空) OR admin。GET/PUT 共用同一 handler，路由零改。
//! 通过 router oneshot 调用，契约在修复前后两签名下均可编译（真父 RED 为断言失败而非编译失败）。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
};
use deadpool_postgres::{Manager, Pool};
use serde_json::Value;
use shared::testing::test_session;
use tower::ServiceExt;

const ALICE: &str = "r635a-alice";
const BOB: &str = "r635a-bob";
const ADMIN: &str = "r635a-admin";
const BOB_TITLE: &str = "R635A_BOB_PRIVATE_TITLE";
const ALICE_TITLE: &str = "R635A_ALICE_PRIVATE_TITLE";
const GLOBAL_TITLE: &str = "R635A_GLOBAL_TITLE";

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

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute(
        "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES          ('r635a-alice','r635a-alice','alice','x'),('r635a-bob','r635a-bob','bob','x'),          ('r635a-admin','r635a-admin','admin','x') ON CONFLICT (id) DO NOTHING",
    ).await.expect("seed persons");
    c.batch_execute(
        "INSERT INTO auth_role (id, name, deleted_at, disable)          VALUES ('r635a-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING",
    ).await.expect("seed role");
    c.batch_execute(
        "INSERT INTO auth_person_role (person_id, role_id, unit_id)          VALUES ('r635a-admin','r635a-role','r635a-unit') ON CONFLICT DO NOTHING",
    ).await.expect("seed admin role");
    // 三行：bob 私有键行、alice 私有键行、部署级全局空键行
    c.batch_execute(
        "INSERT INTO x_hotpic (id, application, info_id, title, image_url, creator, deleted_at) VALUES          ('r635a-bob-priv','news','bob-info','R635A_BOB_PRIVATE_TITLE','R635A_BOB_URL','r635a-bob',NULL),          ('r635a-alice-priv','news','alice-info','R635A_ALICE_PRIVATE_TITLE','R635A_ALICE_URL','r635a-alice',NULL),          ('r635a-global','','','R635A_GLOBAL_TITLE','R635A_GLOBAL_URL','system',NULL)          ON CONFLICT (id) DO NOTHING",
    ).await.expect("seed hotpic");
}

async fn send(pool: &Pool, actor: Option<&str>) -> (StatusCode, Value) {
    let mut app = hotpic_assemble_control::router(pool.clone());
    if let Some(person) = actor {
        app = app.layer(Extension(test_session(person)));
    }
    let req = Request::builder()
        .uri("/api/hotpic/assemble/control/user/hotpic/filter/list/page/1/count/200")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap_or_default();
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

fn titles(v: &Value) -> Vec<String> {
    v.get("data")
        .and_then(|d| d.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.get("title").and_then(|t| t.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

#[tokio::test]
async fn r635a_hotpic_filter_list_scope_gate() {
    let pool = fresh_scratch("r635a_scope_test").await;
    seed(&pool).await;

    // 反臂：匿名直调 handler（router 无 session 层）应 401
    let (anon_status, _) = send(&pool, None).await;
    assert_eq!(StatusCode::UNAUTHORIZED, anon_status, "匿名必须 401");

    // 本人视图：仅本人私有 + 全局，不得见 bob 私有行
    let (alice_status, alice_body) = send(&pool, Some(ALICE)).await;
    assert_eq!(StatusCode::OK, alice_status, "alice 本人应 200");
    let a = titles(&alice_body);
    assert!(a.iter().any(|t| t == ALICE_TITLE), "alice 应见本人私有行");
    assert!(a.iter().any(|t| t == GLOBAL_TITLE), "alice 应见全局行");
    assert!(!a.iter().any(|t| t == BOB_TITLE), "alice 不得见 bob 私有行");

    // 跨身份：bob 同样不得见 alice 私有行
    let (bob_status, bob_body) = send(&pool, Some(BOB)).await;
    assert_eq!(StatusCode::OK, bob_status, "bob 本人应 200");
    let b = titles(&bob_body);
    assert!(b.iter().any(|t| t == BOB_TITLE), "bob 应见本人私有行");
    assert!(
        !b.iter().any(|t| t == ALICE_TITLE),
        "bob 不得见 alice 私有行"
    );

    // admin：可见全部三行
    let (admin_status, admin_body) = send(&pool, Some(ADMIN)).await;
    assert_eq!(StatusCode::OK, admin_status, "admin 应 200");
    let m = titles(&admin_body);
    assert!(m.iter().any(|t| t == BOB_TITLE), "admin 应见 bob 私有行");
    assert!(
        m.iter().any(|t| t == ALICE_TITLE),
        "admin 应见 alice 私有行"
    );
    assert!(m.iter().any(|t| t == GLOBAL_TITLE), "admin 应见全局行");
}

#[test]
fn r635a_hotpic_filter_source_guard() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read src/lib.rs");
    let sig = "pub async fn user_hotpic_filter_list_page_page_count_count(";
    let start = src.find(sig).expect("target handler exists");
    let rest = &src[start + sig.len()..];
    let end = rest.find("pub async fn").unwrap_or(rest.len());
    let body = &rest[..end];
    assert!(
        body.contains("Option<Extension<shared::session::Session>>"),
        "handler 必须提取会话"
    );
    assert!(
        body.contains("require_session(session)?"),
        "handler 必须在首个 DB 访问前 require_session"
    );
    assert!(body.contains("creator = $3"), "读口径须含本人 creator = $3");
    assert!(
        body.contains("application = '' AND info_id = ''"),
        "读口径须含全局空键行"
    );
    assert!(body.contains("r.name = 'admin'"), "读口径须含 admin EXISTS");
}
