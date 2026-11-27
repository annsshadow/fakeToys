// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R616b: correlation_core_entity delete_by_id 身份盲软删修复回归。
//! 真父: delete_by_id 只 require_session 验会话存在, 按主键 find_by_id 后
//!       无条件置 deleted_at 软删; 任意登录人可删他人关联记录(身份盲)。
//! 关联表无 creator/owner 列, 不做逐人归属门(source_id 是任意对象 id, 非
//! person_unique); 定稿为 admin-only 门: 非 admin 一律 403, admin 200。
//! 行为变化: 原"本人凭自己 id 删除 200"在新语义下非 admin 变 403。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use correlation_core_entity::router;
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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r616b-alice','r616b-alice','alice','x'),('r616b-bob','r616b-bob','bob','x'),('r616b-admin','r616b-admin','admin','x') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r616b-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r616b-admin','r616b-role','r616b-unit');"#).await.expect("seed admin role");
    c.batch_execute(r#"INSERT INTO x_corr_c_correlation (id,source_type,source_id,target_type,target_id,weight,create_time,deleted_at) VALUES ('c-bob','person','r616b-bob','doc','d1',1,'2026-01-01 00:00:00',NULL),('c-alice','person','r616b-alice','doc','d2',1,'2026-01-01 00:00:00',NULL) ON CONFLICT (id) DO NOTHING;"#).await.expect("seed correlation");
}

async fn deleted_at_null(pool: &Pool, id: &str) -> bool {
    let c = pool.get().await.expect("field conn");
    let row = c
        .query_one(
            "SELECT deleted_at IS NULL AS v FROM x_corr_c_correlation WHERE id = $1",
            &[&id],
        )
        .await
        .expect("deleted_at query");
    row.get::<_, bool>("v")
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
async fn correlation_delete_requires_admin_and_preserves_db() {
    let (pool, db) = fresh_scratch("oa_r616b_admin_gate").await;
    seed(&pool).await;
    let app = router(db);
    let del = |id: &str| format!("/api/correlation/core/entity/delete/{id}");

    // 1) 跨用户: alice 凭 bob 关联 id 删除必须 403, 且 bob 记录未被软删
    assert_eq!(
        call(
            &app,
            Some("r616b-alice"),
            "DELETE",
            &del("c-bob"),
            json!({})
        )
        .await,
        StatusCode::FORBIDDEN,
        "跨用户 delete 必须 403"
    );
    assert!(
        deleted_at_null(&pool, "c-bob").await,
        "越权 delete 不得软删他人记录"
    );

    // 2) 裸请求无会话必须 401, 数据未删
    assert_eq!(
        call(&app, None, "DELETE", &del("c-bob"), json!({})).await,
        StatusCode::UNAUTHORIZED,
        "裸 delete 必须 401"
    );
    assert!(deleted_at_null(&pool, "c-bob").await, "裸 delete 不得改库");

    // 3) 非 admin 本人删自己关联也必须 403 (行为变化)
    assert_eq!(
        call(
            &app,
            Some("r616b-alice"),
            "DELETE",
            &del("c-alice"),
            json!({})
        )
        .await,
        StatusCode::FORBIDDEN,
        "非 admin 本人 delete 在新语义下必须 403"
    );
    assert!(
        deleted_at_null(&pool, "c-alice").await,
        "非 admin 本人 delete 不得软删"
    );

    // 4) admin 删除必须 200 且落库软删
    assert_eq!(
        call(
            &app,
            Some("r616b-admin"),
            "DELETE",
            &del("c-bob"),
            json!({})
        )
        .await,
        StatusCode::OK,
        "admin delete 必须 200"
    );
    assert!(
        !deleted_at_null(&pool, "c-bob").await,
        "admin delete 必须置 deleted_at 非空"
    );
}

#[tokio::test]
async fn admin_delete_missing_id_is_noop() {
    let (pool, db) = fresh_scratch("oa_r616b_missing").await;
    seed(&pool).await;
    let app = router(db);
    let uri = "/api/correlation/core/entity/delete/does-not-exist";
    // ActionResult::error 仍映射 200 OK(既有响应形状, 本轮不改)
    assert_eq!(
        call(&app, Some("r616b-admin"), "DELETE", uri, json!({})).await,
        StatusCode::OK,
        "admin 删不存在 id 仍返回 200(error body)"
    );
    assert!(
        deleted_at_null(&pool, "c-bob").await,
        "删不存在 id 不得影响其他记录"
    );
}
