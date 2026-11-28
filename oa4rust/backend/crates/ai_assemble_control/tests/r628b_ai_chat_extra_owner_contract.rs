// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R628b: ai_assemble_control chat_write_completion_extra 身份盲写 IDOR 回归。
//! 真父: extra 端点只 require_session 验会话存在, UPDATE x_ai_chat 无归属谓词,
//!       任意登录人凭 chat id 即可覆写他人对话扩展数据。
//! 修复: UPDATE 前经 x_ai_chat.conversation_id -> x_ai_conversation.user_id
//!       关联取归属, 调 shared::middleware::require_owner (本人/admin 放行)。

use ai_assemble_control::router;
use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
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
    pool
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r628b-alice','r628b-alice','alice','x'),('r628b-bob','r628b-bob','bob','x'),('r628b-admin','r628b-admin','admin','x') ON CONFLICT (id) DO NOTHING;").await.expect("seed persons");
    c.batch_execute("INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r628b-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;").await.expect("seed role");
    c.batch_execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r628b-admin','r628b-role','r628b-unit');").await.expect("seed admin role");
    c.batch_execute("INSERT INTO x_ai_conversation (id, title, user_id, create_time) VALUES ('r628b-conv','conv','r628b-bob','2026-01-01 00:00:00');").await.expect("seed conversation");
    c.batch_execute("INSERT INTO x_ai_chat (id, conversation_id, role, content, create_time, creator, extra) VALUES ('r628b-chat','r628b-conv','assistant','orig','2026-01-01 00:00:00','r628b-bob','{\"origin\":true}'::jsonb);").await.expect("seed chat");
}

async fn extra_field(pool: &Pool, id: &str) -> String {
    let c = pool.get().await.expect("field conn");
    let row = c
        .query_one(
            "SELECT extra::text AS v FROM x_ai_chat WHERE id = $1",
            &[&id],
        )
        .await
        .expect("field query");
    row.get::<_, String>("v")
}

async fn call(app: &Router, who: Option<&str>, body: Value) -> StatusCode {
    let req = Request::builder()
        .method("POST")
        .uri("/api/ai_assemble_control/chat/write/completion/extra")
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
async fn r628b_foreign_write_forbidden_owner_admin_ok_and_source_guard() {
    let pool = fresh_scratch("oa_r628b_extra").await;
    seed(&pool).await;
    let app = router(pool.clone());
    let before = extra_field(&pool, "r628b-chat").await;

    // (1) 真父漏洞臂: alice 凭 bob 的 chat id 覆写 extra 必须 403; DB 不得改。
    assert_eq!(
        call(
            &app,
            Some("r628b-alice"),
            json!({"id":"r628b-chat","extra":{"hacked":"alice"}})
        )
        .await,
        StatusCode::FORBIDDEN,
        "跨身份覆写他人 extra 必须 403"
    );
    assert_eq!(
        extra_field(&pool, "r628b-chat").await,
        before,
        "越权覆写不得改库"
    );

    // (2) 属主本人 200 且落库。
    assert_eq!(
        call(
            &app,
            Some("r628b-bob"),
            json!({"id":"r628b-chat","extra":{"owner":"bob"}})
        )
        .await,
        StatusCode::OK,
        "属主本人必须 200"
    );
    assert_ne!(
        extra_field(&pool, "r628b-chat").await,
        before,
        "属主覆写必须落库"
    );

    // (3) admin 跨人 200。
    assert_eq!(
        call(
            &app,
            Some("r628b-admin"),
            json!({"id":"r628b-chat","extra":{"by":"admin"}})
        )
        .await,
        StatusCode::OK,
        "admin 代改必须 200"
    );

    // (4) 匿名裸调 401。
    assert_eq!(
        call(&app, None, json!({"id":"r628b-chat","extra":{}})).await,
        StatusCode::UNAUTHORIZED,
        "匿名裸调必须 401"
    );

    // (5) 不存在的 chat id 404。
    assert_eq!(
        call(
            &app,
            Some("r628b-bob"),
            json!({"id":"r628b-missing","extra":{}})
        )
        .await,
        StatusCode::NOT_FOUND,
        "不存在 id 必须 404"
    );

    // (6) 源守卫: 修复必须经两表关联取归属并调 require_owner。
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    let start = src
        .find("pub async fn chat_write_completion_extra")
        .expect("handler fn");
    let body = &src[start..];
    assert!(
        body.contains("require_owner"),
        "取归属后必须调 require_owner"
    );
    assert!(
        body.contains("x_ai_conversation") && body.contains("JOIN"),
        "必须经两表关联取 user_id"
    );
}
