// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R622a: mind_save 身份盲写 IDOR 真修复回归契约。
//! 真父: mind_save 更新分支只 require_session 验会话存在, UPDATE WHERE id 无归属门,
//!       任意登录者可覆写他人脑图(含 shared 私有转公开)+以自己身份写受害者版本链。
//! 修复: 更新分支进事务后加 person_can_manage_mind(属主或 admin) 门禁。
//! 夹具修正 schema 两处独立缺陷(file_version INT4/i64 错配、folder_id NOT NULL 漏列),
//! 否则真父越权会被 schema 错误 500 回滚而掩盖身份盲写。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use mind_assemble_control::mind_assemble_control_router;
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
    // 隔离两处独立 schema 缺陷,使身份盲写逻辑缺陷可被真执行区分
    let c = pool.get().await.expect("fix conn");
    c.execute(
        "ALTER TABLE x_mind_version_info ALTER COLUMN file_version TYPE BIGINT",
        &[],
    )
    .await
    .expect("widen file_version");
    c.execute(
        "ALTER TABLE x_mind_version_info ALTER COLUMN folder_id SET DEFAULT ''",
        &[],
    )
    .await
    .expect("default folder_id");
    drop(c);
    pool
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r622a-alice','r622a-alice','alice','x'),('r622a-bob','r622a-bob','bob','x'),('r622a-admin','r622a-admin','admin','x') ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed persons");
    c.batch_execute("INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r622a-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed role");
    c.batch_execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r622a-admin','r622a-role','r622a-unit') ON CONFLICT DO NOTHING;")
        .await.expect("seed admin role");
    c.batch_execute("INSERT INTO x_mind (id, name, content, creator, shared, folder_id, create_time) VALUES ('r622a-victim-mind','original','original-content','r622a-bob',false,'','2026-01-01 00:00:00');")
        .await.expect("seed mind");
    c.batch_execute("INSERT INTO x_mind_version_info (id, mind_id, name, folder_id, creator, file_version, shared) VALUES ('r622a-v1','r622a-victim-mind','original','','r622a-bob',1,false);")
        .await.expect("seed version");
}

async fn field(pool: &Pool, table: &str, id: &str, col: &str) -> String {
    let c = pool.get().await.expect("field conn");
    let row = c
        .query_opt(
            &format!("SELECT {col}::text AS v FROM {table} WHERE id = $1"),
            &[&id],
        )
        .await
        .expect("field query");
    row.map(|r| r.get::<_, String>("v")).unwrap_or_default()
}

async fn version_creator_count(pool: &Pool, mind_id: &str, creator: &str) -> i64 {
    let c = pool.get().await.expect("version conn");
    let row = c.query_one(
        "SELECT COUNT(*)::bigint AS n FROM x_mind_version_info WHERE mind_id = $1 AND creator = $2",
        &[&mind_id, &creator],
    ).await.expect("version count query");
    row.get::<_, i64>("n")
}

async fn call(
    app: &Router,
    who: Option<&str>,
    method: &str,
    uri: &str,
    body: Value,
) -> (StatusCode, String) {
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
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("body");
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

const SAVE_URI: &str = "/api/mind/assemble/control/mind/save";
const SHARE_URI: &str = "/api/mind/assemble/control/mind/share/r622a-victim-mind";

#[tokio::test]
async fn r622a_mind_save_forged_write_is_idor_and_gated() {
    let pool = fresh_scratch("oa_r622a_idor").await;
    seed(&pool).await;
    let app = mind_assemble_control_router(pool.clone());

    let (st, _) = call(
        &app,
        None,
        "POST",
        SAVE_URI,
        json!({"id":"r622a-victim-mind","name":"x"}),
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "裸 mind_save 必须 401");

    let (st, body) = call(
        &app,
        Some("r622a-alice"),
        "PUT",
        SHARE_URI,
        json!({"person":"r622a-alice"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "mind_share 拒绝仍走 200 信封");
    assert!(
        body.contains("forbidden"),
        "兄弟 mind_share 必须拒绝非属主: {body}"
    );

    let (st, _) = call(
        &app,
        Some("r622a-alice"),
        "POST",
        SAVE_URI,
        json!({"id":"r622a-victim-mind","name":"hacked","content":"hacked-content","shared":true}),
    )
    .await;
    assert_eq!(
        st,
        StatusCode::FORBIDDEN,
        "非属主非admin伪造覆写必须403(真父此处200坐实IDOR)"
    );

    assert_eq!(
        field(&pool, "x_mind", "r622a-victim-mind", "name").await,
        "original",
        "越权不得改name"
    );
    assert_eq!(
        field(&pool, "x_mind", "r622a-victim-mind", "shared").await,
        "false",
        "越权不得把私有转公开"
    );
    assert_eq!(
        version_creator_count(&pool, "r622a-victim-mind", "r622a-alice").await,
        0,
        "越权不得写受害者版本链"
    );
}

#[tokio::test]
async fn r622a_mind_save_owner_and_admin_still_work_and_source_guard() {
    let pool = fresh_scratch("oa_r622a_owner").await;
    seed(&pool).await;
    let app = mind_assemble_control_router(pool.clone());

    let (st, _) = call(&app, Some("r622a-bob"), "POST", SAVE_URI,
        json!({"id":"r622a-victim-mind","name":"owner-edit","content":"owner-content","shared":false})).await;
    assert_eq!(st, StatusCode::OK, "属主本人必须200");
    assert_eq!(
        field(&pool, "x_mind", "r622a-victim-mind", "name").await,
        "owner-edit",
        "属主编辑必须落库"
    );

    let (st, _) = call(&app, Some("r622a-admin"), "POST", SAVE_URI,
        json!({"id":"r622a-victim-mind","name":"admin-edit","content":"admin-content","shared":false})).await;
    assert_eq!(st, StatusCode::OK, "admin代改必须200");
    assert_eq!(
        field(&pool, "x_mind", "r622a-victim-mind", "name").await,
        "admin-edit",
        "admin编辑必须落库"
    );

    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/u2.rs"))
        .expect("read u2.rs");
    let start = src.find("pub async fn mind_save").expect("mind_save fn");
    let tail = &src[start..];
    let end = tail
        .find("INSERT INTO x_mind (id, name, content, folder_id")
        .expect("insert-new branch");
    let body = &tail[..end];
    assert!(
        body.contains("person_can_manage_mind"),
        "mind_save更新分支必须调用person_can_manage_mind门"
    );
}
