// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R628a: file folder_update/folder_remove identity-blind write IDOR regression.
//! Parent: require_session drops session and UPDATE has no person predicate;
//! alice can rename / soft-delete bob folder by id. Fixed: capture session,
//! SELECT person, require_owner before the write.

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use serde_json::{json, Value};
use tower::ServiceExt;

const UPDATE_SCRATCH: &str = "oa_r628a_file_update";
const REMOVE_SCRATCH: &str = "oa_r628a_file_remove";
const BOB_FOLDER: &str = "r628a-f-bob";
const ALICE_FOLDER: &str = "r628a-f-alice";

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
        &format!("DROP DATABASE IF EXISTS {} WITH (FORCE)", dbname),
        &[],
    )
    .await
    .expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {}", dbname), &[])
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
    c.batch_execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r628a-alice', 'r628a-alice', 'alice', 'x'), ('r628a-bob', 'r628a-bob', 'bob', 'x'), ('r628a-admin', 'r628a-admin', 'admin', 'x');")
        .await.expect("seed persons");
    c.batch_execute("INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r628a-role', 'admin', NULL, false);")
        .await.expect("seed role");
    c.batch_execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r628a-admin', 'r628a-role', 'r628a-unit');")
        .await.expect("seed admin role");
    c.batch_execute("INSERT INTO FILE_FOLDER (id, name, person) VALUES ('r628a-f-bob', 'bob-original', 'r628a-bob'), ('r628a-f-alice', 'alice-original', 'r628a-alice');")
        .await.expect("seed folders");
}

async fn folder_name(pool: &Pool, id: &str) -> String {
    let c = pool.get().await.expect("query conn");
    let row = c
        .query_one("SELECT name FROM FILE_FOLDER WHERE id = $1", &[&id])
        .await
        .expect("name query");
    row.get::<_, String>("name")
}

async fn folder_deleted(pool: &Pool, id: &str) -> bool {
    let c = pool.get().await.expect("query conn");
    let row = c
        .query_one(
            "SELECT deleted_at IS NOT NULL AS d FROM FILE_FOLDER WHERE id = $1",
            &[&id],
        )
        .await
        .expect("deleted query");
    row.get::<_, bool>("d")
}

async fn call(app: &Router, who: Option<&str>, uri: &str, body: Value) -> StatusCode {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
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
async fn folder_update_owner_guard() {
    let pool = fresh_scratch(UPDATE_SCRATCH).await;
    seed(&pool).await;
    let app = file::router(pool.clone());
    let uri = "/api/file/folder/update";
    // cross-user rename must be 403 and MUST NOT persist
    assert_eq!(
        call(
            &app,
            Some("r628a-alice"),
            uri,
            json!({"id": BOB_FOLDER, "name": "HackedByAlice"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "alice renaming bob folder must 403"
    );
    assert_eq!(
        folder_name(&pool, BOB_FOLDER).await,
        "bob-original",
        "victim name unchanged"
    );
    // anonymous -> 401
    assert_eq!(
        call(&app, None, uri, json!({"id": BOB_FOLDER, "name": "Anon"})).await,
        StatusCode::UNAUTHORIZED,
        "no session must 401"
    );
    // owner -> 200 and persisted
    assert_eq!(
        call(
            &app,
            Some("r628a-bob"),
            uri,
            json!({"id": BOB_FOLDER, "name": "BobRenamed"})
        )
        .await,
        StatusCode::OK,
        "owner rename must 200"
    );
    assert_eq!(
        folder_name(&pool, BOB_FOLDER).await,
        "BobRenamed",
        "owner rename persisted"
    );
    // admin cross-user -> 200
    assert_eq!(
        call(
            &app,
            Some("r628a-admin"),
            uri,
            json!({"id": ALICE_FOLDER, "name": "AdminRenamed"})
        )
        .await,
        StatusCode::OK,
        "admin rename must 200"
    );
}

#[tokio::test]
async fn folder_remove_owner_guard() {
    let pool = fresh_scratch(REMOVE_SCRATCH).await;
    seed(&pool).await;
    let app = file::router(pool.clone());
    let uri = "/api/file/folder/remove";
    // cross-user soft-delete must be 403 and MUST NOT persist
    assert_eq!(
        call(&app, Some("r628a-alice"), uri, json!({"id": BOB_FOLDER})).await,
        StatusCode::FORBIDDEN,
        "alice removing bob folder must 403"
    );
    assert!(
        !folder_deleted(&pool, BOB_FOLDER).await,
        "victim not deleted"
    );
    // anonymous -> 401
    assert_eq!(
        call(&app, None, uri, json!({"id": BOB_FOLDER})).await,
        StatusCode::UNAUTHORIZED,
        "no session must 401"
    );
    // owner -> 200 and persisted
    assert_eq!(
        call(&app, Some("r628a-bob"), uri, json!({"id": BOB_FOLDER})).await,
        StatusCode::OK,
        "owner remove must 200"
    );
    assert!(
        folder_deleted(&pool, BOB_FOLDER).await,
        "owner remove persisted"
    );
    // admin cross-user -> 200
    assert_eq!(
        call(&app, Some("r628a-admin"), uri, json!({"id": ALICE_FOLDER})).await,
        StatusCode::OK,
        "admin remove must 200"
    );
}
