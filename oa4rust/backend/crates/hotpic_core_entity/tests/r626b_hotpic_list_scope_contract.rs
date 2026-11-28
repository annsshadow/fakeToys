// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R626b: hotpic list 绕过 visible_scope 身份盲读真修复回归契约。
//! 真父: GET /api/hotpic/core/entity/list 仅 db 无 session, 裸 Entity::find() 全表
//!       limit20 回传 base64, 任意视图可读他人热图。
//! 修复: list 加 Option<Extension<Session>> + require_session 门(匿名401)置于 DB 访问前,
//!       可见范围 = 本人创建 creator=me OR admin OR 全局行(application 与 info_id 皆空)。

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use deadpool_postgres::{Manager, Pool};
use hotpic_core_entity::hotpic_core_entity_router;
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

fn orm_db_url(dbname: &str) -> String {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    match url.rsplit_once('/') {
        Some((prefix, _)) => format!("{prefix}/{dbname}"),
        None => url,
    }
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r626b-alice','r626b-alice','alice','x'),('r626b-bob','r626b-bob','bob','x'),('r626b-admin','r626b-admin','admin','x') ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed persons");
    c.batch_execute("INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r626b-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed role");
    c.batch_execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r626b-admin','r626b-role','r626b-unit') ON CONFLICT DO NOTHING;")
        .await.expect("seed admin role");
    c.batch_execute("INSERT INTO x_hotpic (id, application, info_id, title, base64, creator, create_time, deleted_at) VALUES ('h-bob','app-bob','view-bob','bob-secret','BOB_BASE64_SECRET','r626b-bob','2026-01-03 00:00:00',NULL),('h-alice','app-alice','view-alice','alice-own','ALICE_BASE64','r626b-alice','2026-01-02 00:00:00',NULL),('h-global','','','global','GLOBAL_BASE64',NULL,'2026-01-01 00:00:00',NULL) ON CONFLICT (id) DO NOTHING;")
        .await.expect("seed hotpic");
}

async fn call(app: &axum::Router, who: Option<&str>, uri: &str) -> (StatusCode, String) {
    let req = Request::builder()
        .method("GET")
        .uri(uri)
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

fn ids(body: &str) -> Vec<String> {
    let v: Value = serde_json::from_str(body).expect("json body");
    v.get("data")
        .and_then(|d| d.get("data"))
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default()
        .iter()
        .filter_map(|r| r.get("id").and_then(|x| x.as_str()).map(|s| s.to_string()))
        .collect()
}

const LIST_URI: &str = "/api/hotpic/core/entity/list";

#[tokio::test]
async fn r626b_list_non_admin_sees_only_self_and_global() {
    let pool = fresh_scratch("oa_r626b_scope").await;
    seed(&pool).await;
    let db = sea_orm::Database::connect(orm_db_url("oa_r626b_scope"))
        .await
        .expect("sea_orm db");
    let app = hotpic_core_entity_router(db);

    let (st, body) = call(&app, Some("r626b-alice"), LIST_URI).await;
    assert_eq!(st, StatusCode::OK, "list 应成功");
    let got = ids(&body);
    assert!(got.contains(&"h-alice".to_string()), "本人热图必须可见");
    assert!(got.contains(&"h-global".to_string()), "全局热图必须可见");
    assert!(!got.contains(&"h-bob".to_string()), "他人视图热图不得可见");
    assert!(!body.contains("BOB_BASE64_SECRET"), "不得回传他人 base64");
}

#[tokio::test]
async fn r626b_list_anonymous_401_and_admin_sees_all() {
    let pool = fresh_scratch("oa_r626b_scope_admin").await;
    seed(&pool).await;
    let db = sea_orm::Database::connect(orm_db_url("oa_r626b_scope_admin"))
        .await
        .expect("sea_orm db");
    let app = hotpic_core_entity_router(db);

    let (st, _) = call(&app, None, LIST_URI).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "匿名裸调必须 401");

    let (st, body) = call(&app, Some("r626b-admin"), LIST_URI).await;
    assert_eq!(st, StatusCode::OK, "admin list 应成功");
    let got = ids(&body);
    assert!(got.contains(&"h-alice".to_string()), "admin 可见全体");
    assert!(got.contains(&"h-bob".to_string()), "admin 可见全体");
    assert!(got.contains(&"h-global".to_string()), "admin 可见全局");
}

#[test]
fn r626b_list_source_guard() {
    let source = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    let start = source.find("pub async fn list(").expect("list signature");
    let rest = &source[start..];
    let open = rest.find('{').expect("open brace");
    let mut depth = 0i32;
    let mut end = 0usize;
    for (i, ch) in rest[open..].char_indices() {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                end = open + i + 1;
                break;
            }
        }
    }
    let body = &rest[..end];
    assert!(
        body.contains("session: Option<Extension<shared::session::Session>>"),
        "list 必须接收会话"
    );
    assert!(
        body.contains("require_session(session)"),
        "list 必须有会话门"
    );
    assert!(body.contains("r.name = 'admin'"), "list 必须有 admin 判定");
    assert!(
        body.contains("application = '' AND info_id = ''"),
        "list 必须纳入全局行"
    );
}
