// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R612a: mind_core_entity delete endpoints identity-blind IDOR regression.
//! Parent: delete_mind/delete_folder drop session and ORM delete_by_id; alice can delete bob record by id.
//! Fixed: owner-scoped SQL DELETE with CREATOR_OWNER_PREDICATE; 0 rows -> ownership_error 403/404.

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use mind_core_entity::mind_core_entity_router;
use sea_orm::{Database, DatabaseConnection};
use tower::ServiceExt;

const SCRATCH_MIND: &str = "oa_r612a_mind_delete";
const SCRATCH_FOLDER: &str = "oa_r612a_folder_delete";

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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r612a-alice', 'r612a-alice', 'alice', 'x'), ('r612a-bob', 'r612a-bob', 'bob', 'x'), ('r612a-admin', 'r612a-admin', 'admin', 'x');"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r612a-role', 'admin', NULL, false);"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r612a-admin', 'r612a-role', 'r612a-unit');"#).await.expect("seed admin role");
    c.batch_execute(r#"INSERT INTO x_mind_folder_info (id, name, parent_id, order_number, description, creator) VALUES ('f-bob', 'bob-folder', NULL, 0, '', 'r612a-bob'), ('f-alice', 'alice-folder', NULL, 0, '', 'r612a-alice');"#).await.expect("seed folders");
    c.batch_execute(r#"INSERT INTO x_mind_base_info (id, name, folder_id, description, creator) VALUES ('m-bob', 'bob-mind', 'f-bob', '', 'r612a-bob'), ('m-alice', 'alice-mind', 'f-alice', '', 'r612a-alice');"#).await.expect("seed minds");
}

async fn count(pool: &Pool, table: &str, id: &str) -> i64 {
    let c = pool.get().await.expect("count conn");
    let row = c
        .query_one(
            &format!("SELECT COUNT(*)::bigint AS n FROM {table} WHERE id = $1"),
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

fn owner_of(id: &str) -> &'static str {
    if id.contains("alice") {
        "r612a-alice"
    } else {
        "r612a-bob"
    }
}

async fn assert_paths(
    app: &Router,
    pool: &Pool,
    uri: &str,
    table: &str,
    other_id: &str,
    own_id: &str,
) {
    assert_eq!(
        call(app, Some("r612a-alice"), uri).await,
        StatusCode::FORBIDDEN,
        "cross-user delete must 403"
    );
    assert_eq!(
        count(pool, table, other_id).await,
        1,
        "victim record must survive unauthorized delete"
    );
    assert_eq!(
        call(app, None, uri).await,
        StatusCode::UNAUTHORIZED,
        "no session must 401"
    );
    let own_uri = uri.replace(other_id, own_id);
    assert_eq!(
        call(app, Some(owner_of(own_id)), &own_uri).await,
        StatusCode::OK,
        "owner delete must 200"
    );
    assert_eq!(
        count(pool, table, own_id).await,
        0,
        "owner delete must persist"
    );
    assert_eq!(
        call(app, Some("r612a-admin"), uri).await,
        StatusCode::OK,
        "admin delete must 200"
    );
    assert_eq!(
        count(pool, table, other_id).await,
        0,
        "admin delete must persist"
    );
    let missing_uri = uri.replace(other_id, "does-not-exist");
    assert_eq!(
        call(app, Some("r612a-alice"), &missing_uri).await,
        StatusCode::NOT_FOUND,
        "missing id must 404"
    );
}

#[tokio::test]
async fn delete_mind_owner_guard() {
    let (pool, db) = fresh_scratch(SCRATCH_MIND).await;
    seed(&pool).await;
    let app = mind_core_entity_router(db);
    assert_paths(
        &app,
        &pool,
        "/api/mind/core/entity/mind/m-bob",
        "x_mind_base_info",
        "m-bob",
        "m-alice",
    )
    .await;
}

#[tokio::test]
async fn delete_folder_owner_guard() {
    let (pool, db) = fresh_scratch(SCRATCH_FOLDER).await;
    seed(&pool).await;
    let app = mind_core_entity_router(db);
    assert_paths(
        &app,
        &pool,
        "/api/mind/core/entity/folder/f-bob",
        "x_mind_folder_info",
        "f-bob",
        "f-alice",
    )
    .await;
}

#[tokio::test]
async fn source_guard_owner_predicate_and_no_orm_delete() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("source");
    assert!(
        src.contains("let session = require_session(session)?;"),
        "session value must be kept"
    );
    assert_eq!(
        src.matches("let actor = session.person_unique;").count(),
        4,
        "4 handlers (2 update + 2 delete) must take actor"
    );
    assert!(
        src.contains("DELETE FROM x_mind_base_info WHERE id = $1 AND ({CREATOR_OWNER_PREDICATE})"),
        "mind scoped delete"
    );
    assert!(
        src.contains(
            "DELETE FROM x_mind_folder_info WHERE id = $1 AND ({CREATOR_OWNER_PREDICATE})"
        ),
        "folder scoped delete"
    );
    assert_eq!(
        src.matches("ownership_error(&db.0,").count(),
        4,
        "all 4 writes distinguish 403/404"
    );
    assert_eq!(
        src.matches("::Entity::delete_by_id").count(),
        0,
        "no ORM naked delete"
    );
    assert!(
        src.contains("const CREATOR_OWNER_PREDICATE"),
        "owner predicate unchanged"
    );
}
