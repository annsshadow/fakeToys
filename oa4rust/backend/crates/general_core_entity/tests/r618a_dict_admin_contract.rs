// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R618a: general_core_entity dict six write endpoints admin gate regression.
//! Six dict/dict_item write handlers only required a session (identity discarded),
//! so any authenticated user could create/update/delete global dictionary entries.
//! Fixed: reuse is_admin_db (403 for non-admin, 200 for admin, 401 for anonymous).

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use general_core_entity::general_core_entity_router;
use sea_orm::{Database, DatabaseConnection};
use serde_json::{json, Value};
use tower::ServiceExt;

const SCRATCH: &str = "oa_r618a_dict_admin";

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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r618a-alice', 'r618a-alice', 'alice', 'x'), ('r618a-admin', 'r618a-admin', 'admin', 'x');"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r618a-role', 'admin', NULL, false);"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r618a-admin', 'r618a-role', 'r618a-unit');"#).await.expect("seed admin role");
    c.batch_execute(r#"INSERT INTO x_general_application_dict (id, name, application) VALUES ('d-1', 'seed-name', 'seed-app');"#).await.expect("seed dict");
    c.batch_execute(r#"INSERT INTO x_general_application_dict_item (id, dict_id, name, value) VALUES ('di-1', 'd-1', 'seed-item', 'seed-value');"#).await.expect("seed dict item");
}

async fn snapshot(pool: &Pool) -> (i64, Option<String>, i64, Option<String>) {
    let c = pool.get().await.expect("snap conn");
    let d: i64 = c
        .query_one(
            "SELECT COUNT(*)::bigint AS n FROM x_general_application_dict",
            &[],
        )
        .await
        .unwrap()
        .get("n");
    let dn: Option<String> = c
        .query_opt(
            "SELECT name FROM x_general_application_dict WHERE id = 'd-1'",
            &[],
        )
        .await
        .unwrap()
        .and_then(|r| r.get("name"));
    let i: i64 = c
        .query_one(
            "SELECT COUNT(*)::bigint AS n FROM x_general_application_dict_item",
            &[],
        )
        .await
        .unwrap()
        .get("n");
    let iv: Option<String> = c
        .query_opt(
            "SELECT value FROM x_general_application_dict_item WHERE id = 'di-1'",
            &[],
        )
        .await
        .unwrap()
        .and_then(|r| r.get("value"));
    (d, dn, i, iv)
}

async fn call(app: &Router, who: Option<&str>, uri: &str, body: Value) -> StatusCode {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
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
async fn dict_writes_require_admin_and_preserve_db() {
    let (pool, db) = fresh_scratch(SCRATCH).await;
    seed(&pool).await;
    let app = general_core_entity_router(db);
    let before = snapshot(&pool).await;
    assert_eq!(
        before,
        (1, Some("seed-name".into()), 1, Some("seed-value".into()))
    );

    let endpoints: Vec<(&str, Value)> = vec![
        (
            "/api/general/dict/create",
            json!({"name":"x","application":"app"}),
        ),
        (
            "/api/general/dict/update/d-1",
            json!({"name":"x","application":"app"}),
        ),
        ("/api/general/dict/delete/d-1", json!({})),
        (
            "/api/general/dict/item/create",
            json!({"dictId":"d-1","name":"x","value":"v"}),
        ),
        (
            "/api/general/dict/item/update/di-1",
            json!({"dictId":"d-1","name":"x","value":"v"}),
        ),
        ("/api/general/dict/item/delete/di-1", json!({})),
    ];

    for (uri, body) in &endpoints {
        assert_eq!(
            call(&app, Some("r618a-alice"), uri, body.clone()).await,
            StatusCode::FORBIDDEN,
            "non-admin must 403: {uri}"
        );
        assert_eq!(
            snapshot(&pool).await,
            before,
            "non-admin must not mutate DB: {uri}"
        );
        assert_eq!(
            call(&app, None, uri, body.clone()).await,
            StatusCode::UNAUTHORIZED,
            "anonymous must 401: {uri}"
        );
        assert_eq!(
            snapshot(&pool).await,
            before,
            "anonymous must not mutate DB: {uri}"
        );
    }

    for (uri, body) in &endpoints {
        assert_eq!(
            call(&app, Some("r618a-admin"), uri, body.clone()).await,
            StatusCode::OK,
            "admin must 200: {uri}"
        );
    }
    let after = snapshot(&pool).await;
    assert_eq!(after.0, 1, "dict count back to 1 after create+delete");
    assert_eq!(after.1, None, "original d-1 must be gone");
    assert_eq!(after.2, 1, "item count back to 1 after create+delete");
    assert_eq!(after.3, None, "original di-1 must be gone");
}

#[tokio::test]
async fn source_guard_admin_gates_present() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("source");
    for fname in [
        "dict_create",
        "dict_update",
        "dict_delete",
        "dict_item_create",
        "dict_item_update",
        "dict_item_delete",
    ] {
        let body = src
            .split(&format!("pub async fn {fname}("))
            .nth(1)
            .unwrap_or_else(|| panic!("missing {fname}"))
            .split("\n}")
            .next()
            .unwrap_or("");
        assert!(
            body.contains("let actor = require_session(session)?;"),
            "{fname} keeps actor"
        );
        assert!(
            body.contains("is_admin_db(&db.0, &actor.person_unique)"),
            "{fname} has admin gate"
        );
        assert!(
            body.contains("return Err(AppError::Forbidden);"),
            "{fname} 403 for non-admin"
        );
    }
    assert_eq!(
        src.matches("let actor = require_session(session)?;")
            .count(),
        6,
        "exactly six dict write guards"
    );
    let g = src
        .split("pub async fn dict_get(")
        .nth(1)
        .expect("dict_get")
        .split("\n}")
        .next()
        .unwrap_or("");
    assert!(!g.contains("actor"), "dict_get read endpoint unchanged");
}
