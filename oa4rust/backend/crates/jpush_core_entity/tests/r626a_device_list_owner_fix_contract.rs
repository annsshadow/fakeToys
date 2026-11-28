// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R626a: jpush device_list identity-blind read (P1) real-fix regression.
//! Parent: GET /api/jpush/core/entity/device/list takes only db; bare Entity::find()
//!         .limit(20) returns every user's userId+token to any logged-in caller.
//! Fix: device_list takes Option<Extension<Session>> + require_session gate before DB,
//!       scoped SQL reuses the owner predicate so non-admin sees only own devices,
//!       admin sees all (aligned with device_get/device_delete).

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use jpush_core_entity::jpush_core_entity_router;
use sea_orm::Database;
use serde_json::Value;
use tower::ServiceExt;

fn pg_pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config = url.parse().expect("DATABASE_URL");
    cfg.dbname(dbname);
    let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

fn orm_url_to(dbname: &str) -> String {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    match url.rsplit_once('/') {
        Some((prefix, _)) => format!("{prefix}/{dbname}"),
        None => url,
    }
}

async fn fresh_scratch(dbname: &str) -> (Pool, sea_orm::DatabaseConnection) {
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
    let db = Database::connect(orm_url_to(dbname))
        .await
        .expect("sea_orm conn");
    (pool, db)
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r626a-alice', 'r626a-alice', 'alice', 'x'), ('r626a-bob', 'r626a-bob', 'bob', 'x'), ('r626a-admin', 'r626a-admin', 'admin', 'x') ON CONFLICT (id) DO NOTHING;"#)
        .await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r626a-role', 'admin', NULL, false) ON CONFLICT (id) DO NOTHING;"#)
        .await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r626a-admin', 'r626a-role', 'r626a-unit') ON CONFLICT DO NOTHING;"#)
        .await.expect("seed admin role");
    c.batch_execute(r#"INSERT INTO x_jpush_device (id, user_id, platform, token) VALUES ('d-bob', 'r626a-bob', 'ios', 'BOB_PUSH_TOKEN'), ('d-alice', 'r626a-alice', 'android', 'ALICE_PUSH_TOKEN') ON CONFLICT (id) DO NOTHING;"#)
        .await.expect("seed devices");
}

async fn call(app: &Router, who: Option<&str>) -> (StatusCode, String) {
    let req = Request::builder()
        .method("GET")
        .uri("/api/jpush/core/entity/device/list")
        .body(Body::empty())
        .unwrap();
    let resp = match who {
        Some(p) => {
            app.clone()
                .layer(Extension(shared::testing::test_session(p)))
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
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

fn devices(body: &str) -> Vec<Value> {
    let v: Value = serde_json::from_str(body).expect("json body");
    v.get("data")
        .and_then(|d| d.get("data"))
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default()
}

fn ids(body: &str) -> Vec<String> {
    devices(body)
        .iter()
        .filter_map(|r| r.get("id").and_then(|x| x.as_str()).map(|s| s.to_string()))
        .collect()
}

#[tokio::test]
async fn device_list_non_admin_sees_only_self() {
    let (pool, db) = fresh_scratch("oa_r626a_jpush_list_self").await;
    seed(&pool).await;
    let app = jpush_core_entity_router(db);
    let (status, body) = call(&app, Some("r626a-alice")).await;
    assert_eq!(status, StatusCode::OK, "alice list must 200");
    let got = ids(&body);
    assert!(
        got.contains(&"d-alice".to_string()),
        "owner must see own device"
    );
    assert!(
        !got.contains(&"d-bob".to_string()),
        "non-admin must NOT see bob device"
    );
    assert!(
        !body.contains("BOB_PUSH_TOKEN"),
        "non-admin must NOT see bob token"
    );
    assert!(
        !body.contains("r626a-bob"),
        "non-admin must NOT see bob userId"
    );
}

#[tokio::test]
async fn device_list_anonymous_rejected() {
    let (pool, db) = fresh_scratch("oa_r626a_jpush_list_anon").await;
    seed(&pool).await;
    let app = jpush_core_entity_router(db);
    let (status, body) = call(&app, None).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "anonymous list must 401, body={body}"
    );
}

#[tokio::test]
async fn device_list_admin_sees_all() {
    let (pool, db) = fresh_scratch("oa_r626a_jpush_list_admin").await;
    seed(&pool).await;
    let app = jpush_core_entity_router(db);
    let (status, body) = call(&app, Some("r626a-admin")).await;
    assert_eq!(status, StatusCode::OK, "admin list must 200");
    let got = ids(&body);
    assert!(
        got.contains(&"d-alice".to_string()) && got.contains(&"d-bob".to_string()),
        "admin must see all devices, got={got:?}"
    );
}

#[test]
fn device_list_source_guard() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read src/lib.rs");
    let start = src
        .find("pub async fn device_list")
        .expect("device_list fn");
    let end = src.find("/// 获取推送设备详情").expect("device_get doc");
    let f = &src[start..end];
    assert!(
        f.contains("Option<Extension<shared::session::Session>>"),
        "device_list must accept session"
    );
    assert!(
        f.contains("require_session(session)"),
        "device_list must gate session before DB access"
    );
    assert!(
        f.contains("DEVICE_LIST_OWNER_PREDICATE"),
        "device_list must use scoped owner predicate"
    );
    assert!(
        f.contains("Statement::from_sql_and_values"),
        "device_list must use scoped SQL, not bare ORM find"
    );
    assert!(
        !f.contains("Entity::find()"),
        "device_list must not use bare Entity::find()"
    );
    assert!(
        src.contains("pub async fn device_get") && src.contains("pub async fn device_delete"),
        "sibling owner gates must remain"
    );
}
