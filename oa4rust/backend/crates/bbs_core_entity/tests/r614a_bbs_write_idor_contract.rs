// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R614a: bbs_core_entity 写端点身份盲越权修复回归。
//! 真父: forum/section/subject 的 update/delete 只验会话存在, 任意登录人
//!       可改删他人板块/主题(管理面与作者归属均失守)。
//! 修复: forum/section 全写→admin门; subject update/delete→作者或admin门;
//!       create_subject 原本已用 resolve_author 零改。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use bbs_core_entity::bbs_core_entity_router;
use deadpool_postgres::{Manager, Pool};
use sea_orm::{Database, DatabaseConnection};
use serde_json::{json, Value};
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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r614a-alice','r614a-alice','alice','x'),('r614a-bob','r614a-bob','bob','x'),('r614a-admin','r614a-admin','admin','x') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r614a-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r614a-admin','r614a-role','r614a-unit');"#).await.expect("seed admin role");
    c.batch_execute(r#"INSERT INTO bbs_forum_info (id,name,description,order_number,disable,create_time) VALUES ('f1','forum-one','d',0,false,'2026-01-01 00:00:00');"#).await.expect("seed forum");
    c.batch_execute(r#"INSERT INTO bbs_section_info (id,forum_id,name,description,order_number,disable,create_time) VALUES ('s1','f1','sec-one','d',0,false,'2026-01-01 00:00:00');"#).await.expect("seed section");
    c.batch_execute(r#"INSERT INTO bbs_subject_info (id,title,author_id,section_id,content,reply_count,view_count,is_top,disable,create_time) VALUES ('sub1','sub-one','r614a-alice','s1','',0,0,false,false,'2026-01-01 00:00:00');"#).await.expect("seed subject");
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

async fn field(pool: &Pool, table: &str, id: &str, col: &str) -> String {
    let c = pool.get().await.expect("field conn");
    let row = c
        .query_one(
            &format!("SELECT {col}::text AS v FROM {table} WHERE id = $1"),
            &[&id],
        )
        .await
        .expect("field query");
    row.get::<_, String>("v")
}

async fn call(app: &Router, who: Option<&str>, method: &str, uri: &str, body: Value) -> StatusCode {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
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
    resp.status()
}

#[tokio::test]
async fn forum_writes_require_admin_and_preserve_db() {
    let (pool, db) = fresh_scratch("oa_r614a_forum").await;
    seed(&pool).await;
    let app = bbs_core_entity_router(db);
    let base = "/api/bbs/core/entity/forum";
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "POST",
            base,
            json!({"name":"hacked"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "create_forum 非admin必须403"
    );
    assert_eq!(
        count(&pool, "bbs_forum_info", "f1").await,
        1,
        "越权create不得改库"
    );
    assert_eq!(
        call(&app, None, "POST", base, json!({"name":"x"})).await,
        StatusCode::UNAUTHORIZED,
        "裸create必须401"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "POST",
            base,
            json!({"name":"admin-made","description":"d"})
        )
        .await,
        StatusCode::OK,
        "admin create必须200"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-bob"),
            "POST",
            &format!("{base}/f1"),
            json!({"name":"hacked"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "update_forum 非admin必须403"
    );
    assert_eq!(
        field(&pool, "bbs_forum_info", "f1", "name").await,
        "forum-one",
        "越权update不得篡改name"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "POST",
            &format!("{base}/f1"),
            json!({"name":"admin-edit"})
        )
        .await,
        StatusCode::OK,
        "admin update必须200"
    );
    assert_eq!(
        field(&pool, "bbs_forum_info", "f1", "name").await,
        "admin-edit",
        "admin update必须落库"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "DELETE",
            &format!("{base}/f1"),
            json!({})
        )
        .await,
        StatusCode::FORBIDDEN,
        "delete_forum 非admin必须403"
    );
    assert_eq!(
        count(&pool, "bbs_forum_info", "f1").await,
        1,
        "越权delete记录必须仍在"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "DELETE",
            &format!("{base}/f1"),
            json!({})
        )
        .await,
        StatusCode::OK,
        "admin delete必须200"
    );
    assert_eq!(
        count(&pool, "bbs_forum_info", "f1").await,
        0,
        "admin delete必须落库"
    );
}

#[tokio::test]
async fn section_writes_require_admin_and_preserve_db() {
    let (pool, db) = fresh_scratch("oa_r614a_section").await;
    seed(&pool).await;
    let app = bbs_core_entity_router(db);
    let base = "/api/bbs/core/entity/section";
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "POST",
            base,
            json!({"name":"x","forumId":"f1"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "create_section 非admin必须403"
    );
    assert_eq!(
        count(&pool, "bbs_section_info", "s1").await,
        1,
        "越权create不得改库"
    );
    assert_eq!(
        call(&app, None, "POST", base, json!({"name":"x"})).await,
        StatusCode::UNAUTHORIZED,
        "裸create必须401"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "POST",
            base,
            json!({"name":"admin-sec","forumId":"f1"})
        )
        .await,
        StatusCode::OK,
        "admin create必须200"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-bob"),
            "POST",
            &format!("{base}/s1"),
            json!({"name":"hacked"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "update_section 非admin必须403"
    );
    assert_eq!(
        field(&pool, "bbs_section_info", "s1", "name").await,
        "sec-one",
        "越权update不得篡改"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "POST",
            &format!("{base}/s1"),
            json!({"name":"admin-edit"})
        )
        .await,
        StatusCode::OK,
        "admin update必须200"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "DELETE",
            &format!("{base}/s1"),
            json!({})
        )
        .await,
        StatusCode::FORBIDDEN,
        "delete_section 非admin必须403"
    );
    assert_eq!(
        count(&pool, "bbs_section_info", "s1").await,
        1,
        "越权delete记录必须仍在"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "DELETE",
            &format!("{base}/s1"),
            json!({})
        )
        .await,
        StatusCode::OK,
        "admin delete必须200"
    );
    assert_eq!(
        count(&pool, "bbs_section_info", "s1").await,
        0,
        "admin delete必须落库"
    );
}

#[tokio::test]
async fn subject_writes_require_author_or_admin_and_preserve_db() {
    let (pool, db) = fresh_scratch("oa_r614a_subject").await;
    seed(&pool).await;
    let app = bbs_core_entity_router(db);
    let base = "/api/bbs/core/entity/subject";
    assert_eq!(
        call(
            &app,
            Some("r614a-bob"),
            "POST",
            &format!("{base}/sub1"),
            json!({"title":"hacked"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "update_subject 非作者必须403"
    );
    assert_eq!(
        field(&pool, "bbs_subject_info", "sub1", "title").await,
        "sub-one",
        "越权update不得篡改title"
    );
    assert_eq!(
        call(
            &app,
            None,
            "POST",
            &format!("{base}/sub1"),
            json!({"title":"x"})
        )
        .await,
        StatusCode::UNAUTHORIZED,
        "裸update必须401"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "POST",
            &format!("{base}/sub1"),
            json!({"title":"author-edit"})
        )
        .await,
        StatusCode::OK,
        "作者update必须200"
    );
    assert_eq!(
        field(&pool, "bbs_subject_info", "sub1", "title").await,
        "author-edit",
        "作者update必须落库"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-admin"),
            "POST",
            &format!("{base}/sub1"),
            json!({"title":"admin-edit"})
        )
        .await,
        StatusCode::OK,
        "admin update必须200"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-bob"),
            "DELETE",
            &format!("{base}/sub1"),
            json!({})
        )
        .await,
        StatusCode::FORBIDDEN,
        "delete_subject 非作者必须403"
    );
    assert_eq!(
        count(&pool, "bbs_subject_info", "sub1").await,
        1,
        "越权delete记录必须仍在"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "DELETE",
            &format!("{base}/sub1"),
            json!({})
        )
        .await,
        StatusCode::OK,
        "作者delete必须200"
    );
    assert_eq!(
        count(&pool, "bbs_subject_info", "sub1").await,
        0,
        "作者delete必须落库"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "POST",
            base,
            json!({"title":"t","sectionId":"s1"})
        )
        .await,
        StatusCode::OK,
        "create_subject本人必须200"
    );
    assert_eq!(
        call(
            &app,
            Some("r614a-alice"),
            "POST",
            base,
            json!({"title":"t2","sectionId":"s1","authorId":"r614a-bob"})
        )
        .await,
        StatusCode::FORBIDDEN,
        "create_subject冒名必须403"
    );
}

#[test]
fn source_guard_admin_and_author_gates_present() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    let admin_calls = src
        .matches("is_admin_db(&db.0, &sess.person_unique)")
        .count();
    assert!(
        admin_calls >= 8,
        "forum/section 六写 + subject 两写必须收 admin 门, got {admin_calls}"
    );
    assert!(
        src.contains("m.author_id != sess.person_unique"),
        "update_subject 必须有作者门"
    );
    assert!(
        src.contains("model.author_id != sess.person_unique"),
        "delete_subject 必须有作者门"
    );
    assert!(
        src.contains("resolve_author(&db.0, &sess, &payload)"),
        "create_subject 的 resolve_author 必须零改保留"
    );
    let sess_kept = src.matches("let sess = require_session(session)?;").count();
    assert!(sess_kept >= 8, "八写端点必须保留会话值, got {sess_kept}");
}
