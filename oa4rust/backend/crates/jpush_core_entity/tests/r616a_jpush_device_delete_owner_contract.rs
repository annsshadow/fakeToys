// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R616a: jpush_core_entity device_delete identity-blind hard-delete IDOR regression.
//! Parent: device_delete drops session and ORM delete_by_id; alice can hard-delete bob device by id.
//! Fixed: reuse device_read_allowed + device_get existence check (403 existing / 404 missing).

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use jpush_core_entity::jpush_core_entity_router;
use sea_orm::{Database, DatabaseConnection};
use tower::ServiceExt;

const SCRATCH: &str = "oa_r616a_jpush_delete";

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

async fn fresh_scratch(dbname: &str) -> (Pool, DatabaseConnection) {
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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r616a-alice', 'r616a-alice', 'alice', 'x'), ('r616a-bob', 'r616a-bob', 'bob', 'x'), ('r616a-admin', 'r616a-admin', 'admin', 'x');"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r616a-role', 'admin', NULL, false);"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r616a-admin', 'r616a-role', 'r616a-unit');"#).await.expect("seed admin role");
    c.batch_execute(r#"INSERT INTO x_jpush_device (id, user_id, platform, token) VALUES ('d-bob', 'r616a-bob', 'ios', 't1'), ('d-alice', 'r616a-alice', 'android', 't2');"#).await.expect("seed devices");
}

async fn count(pool: &Pool, id: &str) -> i64 {
    let c = pool.get().await.expect("count conn");
    let row = c
        .query_one(
            "SELECT COUNT(*)::bigint AS n FROM x_jpush_device WHERE id = $1",
            &[&id],
        )
        .await
        .expect("count query");
    row.get::<_, i64>("n")
}

async fn call(app: &Router, who: Option<&str>, uri: &str) -> StatusCode {
    let req = Request::builder()
        .method("DELETE")
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let resp = match who {
        Some(person) => {
            app.clone()
                .layer(Extension(shared::testing::test_session(person)))
                .oneshot(req)
                .await
        }
        None => app.clone().oneshot(req).await,
    }
    .expect("oneshot");
    resp.status()
}

#[tokio::test]
async fn device_delete_owner_guard() {
    let (pool, db) = fresh_scratch(SCRATCH).await;
    seed(&pool).await;
    let app = jpush_core_entity_router(db);
    let bob_uri = "/api/jpush/core/entity/device/d-bob";
    let alice_uri = "/api/jpush/core/entity/device/d-alice";
    // cross-user: alice -> bob must 403, victim survives
    assert_eq!(
        call(&app, Some("r616a-alice"), bob_uri).await,
        StatusCode::FORBIDDEN,
        "cross-user delete must 403"
    );
    assert_eq!(
        count(&pool, "d-bob").await,
        1,
        "victim must survive unauthorized delete"
    );
    // no session -> 401
    assert_eq!(
        call(&app, None, bob_uri).await,
        StatusCode::UNAUTHORIZED,
        "no session must 401"
    );
    // owner -> 200 and persisted
    assert_eq!(
        call(&app, Some("r616a-alice"), alice_uri).await,
        StatusCode::OK,
        "owner delete must 200"
    );
    assert_eq!(
        count(&pool, "d-alice").await,
        0,
        "owner delete must persist"
    );
    // admin -> 200 and persisted
    assert_eq!(
        call(&app, Some("r616a-admin"), bob_uri).await,
        StatusCode::OK,
        "admin delete must 200"
    );
    assert_eq!(count(&pool, "d-bob").await, 0, "admin delete must persist");
    // missing id -> 404
    assert_eq!(
        call(
            &app,
            Some("r616a-alice"),
            "/api/jpush/core/entity/device/no-such"
        )
        .await,
        StatusCode::NOT_FOUND,
        "missing id must 404"
    );
}

#[tokio::test]
async fn source_guard_owner_gate_present() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("source");
    // device_create keeps its bare require_session by design (guarded, unchanged).
    // device_get + device_delete must keep the session value (2 occurrences).
    assert_eq!(
        src.matches("let sess = require_session(session)?;").count(),
        2,
        "device_get + device_delete must keep session value"
    );
    let dd = src
        .split("pub async fn device_delete(")
        .nth(1)
        .expect("device_delete body");
    assert!(
        dd.contains("let sess = require_session(session)?;"),
        "device_delete keeps session"
    );
    assert!(
        dd.contains("device_read_allowed(&db.0, &id, &sess.person_unique)"),
        "owner gate"
    );
    assert!(
        dd.contains("Err(AppError::Forbidden)"),
        "403 for existing unauthorized"
    );
    assert!(dd.contains("Err(AppError::NotFound)"), "404 for missing");
    assert_eq!(
        src.matches("device_read_allowed(&db.0, &id, &sess.person_unique)")
            .count(),
        2,
        "device_get + device_delete must call device_read_allowed"
    );
    assert!(
        src.contains("const DEVICE_READ_OWNER_PREDICATE"),
        "owner predicate unchanged"
    );
    assert!(
        src.matches("Err(AppError::NotFound)").count() >= 2,
        "delete must distinguish 404 like device_get"
    );
}
