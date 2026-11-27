// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R602 契约：日历事件列表两裸读端点（calendar SQL 版 + calendar_core_entity ORM 版）
//! 的私有日历归属门禁。临时 PG scratch 库 seed alice 私有日历 + 唯一私密标记事件 + bob/admin。
//! 真父版本无 session 提取器也无归属查询，本契约定点 RED；修复后 GREEN。

use axum::{body::Body, extract::Extension, http::{Request, StatusCode}, Router};
use calendar::calendar_router;
use calendar_core_entity::calendar_core_entity_router;
use deadpool_postgres::Pool;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use shared::session::Session;
use tower::ServiceExt;

const SQL_EVENT: &str = "/api/calendar/event/list/alice-private";
const ORM_EVENT: &str = "/api/calendar/core/entity/event/list/alice-private";
const PUBLIC_EVENT: &str = "/api/calendar/event/list/public-cal";
const CANARY: &str = "r602-secret-event-canary";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("DATABASE_URL 解析失败");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin 连接");
    c.execute(&format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"), &[])
        .await.expect("drop scratch");
    c.execute(&format!("CREATE DATABASE {dbname}"), &[]).await.expect("create scratch");
    drop(c);
    let pool = pool_to(dbname);
    shared::migrate::run_migrations(&pool).await.expect("scratch 迁移");
    pool
}

async fn drop_scratch(dbname: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c.execute(&format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"), &[]).await;
    }
}

fn session_for(person: &str) -> Session {
    serde_json::from_value(serde_json::json!({
        "token": "r602-probe",
        "person_unique": person,
        "created_at": "2026-01-01T00:00:00",
        "expires_at": "2099-01-01T00:00:00"
    })).expect("构造测试 Session")
}

async fn call(app: Router, who: Option<&str>, uri: &str) -> (StatusCode, Value) {
    let mut app = app;
    if let Some(person) = who {
        app = app.layer(Extension(session_for(person)));
    }
    let resp = app.oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap()).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

fn event_titles(v: &Value) -> Vec<String> {
    let mut titles = Vec::new();
    // SQL 版返回 {"data":[...]}，ORM 版返回 {"data":{"data":[...]}}，两种形状都兼容。
    let arr = v.get("data")
        .and_then(|d| d.as_array().or_else(|| d.get("data").and_then(|d| d.as_array())));
    if let Some(arr) = arr {
        for item in arr {
            if let Some(t) = item.get("title").and_then(|t| t.as_str()) {
                titles.push(t.to_string());
            }
        }
    }
    titles
}

async fn seed_common(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO cal_calendar (id, name, type, target, color, description, source, createor, is_public, status, create_time) VALUES \
         ('alice-private', 'Alice Private', 'PERSON', '', '', NULL, NULL, 'alice', false, 'OPEN', '2026-01-01 00:00:00'), \
         ('public-cal', 'Public Cal', 'UNIT', '', '', NULL, NULL, 'bob', true, 'OPEN', '2026-01-02 00:00:00')",
        &[],
    ).await.expect("seed cal_calendar");
    c.execute(
        "INSERT INTO x_cal_calendar (id, name, type, target, color, description, source, createor, is_public, status, create_time) VALUES \
         ('alice-private', 'Alice Private', 'PERSON', '', '', NULL, NULL, 'alice', false, 'OPEN', '2026-01-01 00:00:00'), \
         ('public-cal', 'Public Cal', 'UNIT', '', '', NULL, NULL, 'bob', true, 'OPEN', '2026-01-02 00:00:00')",
        &[],
    ).await.expect("seed x_cal_calendar");
    c.execute(
        "INSERT INTO cal_event (id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor) VALUES \
         ('secret-1', 'alice-private', $1, NULL, NULL, '2026-01-01 10:00:00', '2026-01-01 11:00:00', false, 'PRIVATE', 'OPEN', 'alice'), \
         ('public-1', 'public-cal', 'public-event', NULL, NULL, '2026-01-02 10:00:00', '2026-01-02 11:00:00', false, 'PUBLIC', 'OPEN', 'bob')",
        &[&CANARY],
    ).await.expect("seed cal_event");
    c.execute(
        "INSERT INTO x_cal_event (id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor) VALUES \
         ('secret-1', 'alice-private', $1, NULL, NULL, '2026-01-01 10:00:00', '2026-01-01 11:00:00', false, 'PRIVATE', 'OPEN', 'alice'), \
         ('public-1', 'public-cal', 'public-event', NULL, NULL, '2026-01-02 10:00:00', '2026-01-02 11:00:00', false, 'PUBLIC', 'OPEN', 'bob')",
        &[&CANARY],
    ).await.expect("seed x_cal_event");
    // admin 身份：scratch 迁移已注入 person_unique=admin 且绑定 name=admin 角色，
    // 直接复用（shared::middleware::is_admin 按 unique_id + role.name=admin 判定）。
    drop(c);
}

fn orm_db_url(dbname: &str) -> String {
    let base = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    let mut u = url::Url::parse(&base).expect("DATABASE_URL 解析失败");
    u.set_path(&format!("/{dbname}"));
    u.to_string()
}

#[tokio::test]
async fn sql_event_list_private_requires_owner_or_admin() {
    const DB: &str = "oa_r602_sql";
    let pool = fresh_scratch(DB).await;
    seed_common(&pool).await;
    let app = calendar_router(pool.clone());
    let (st, _) = call(app.clone(), None, SQL_EVENT).await;
    assert!(st == StatusCode::UNAUTHORIZED, "SQL 版私有日历事件无会话须 401，实得 {st}");
    let (st, _) = call(app.clone(), Some("bob"), SQL_EVENT).await;
    assert!(st == StatusCode::FORBIDDEN, "SQL 版 bob 读 alice 私有事件须 403，实得 {st}");
    let (st, v) = call(app.clone(), Some("alice"), SQL_EVENT).await;
    assert_eq!(st, StatusCode::OK, "SQL 版 alice 本人可读");
    assert!(event_titles(&v).iter().any(|t| t == CANARY), "alice 应见唯一私密标记事件");
    let (st, v) = call(app.clone(), Some("admin"), SQL_EVENT).await;
    assert_eq!(st, StatusCode::OK, "SQL 版 admin 可读");
    assert!(event_titles(&v).iter().any(|t| t == CANARY), "admin 应见私密标记事件");
    let (st, v) = call(app, Some("bob"), PUBLIC_EVENT).await;
    assert_eq!(st, StatusCode::OK, "公开日历事件登录用户可读");
    assert!(event_titles(&v).iter().any(|t| t == "public-event"), "公开事件可见");
    drop_scratch(DB).await;
}

#[tokio::test]
async fn orm_event_list_private_requires_owner_or_admin() {
    const DB: &str = "oa_r602_orm";
    let pool = fresh_scratch(DB).await;
    seed_common(&pool).await;
    let db: DatabaseConnection = sea_orm::Database::connect(orm_db_url(DB)).await.expect("sea-orm 连接");
    let app = calendar_core_entity_router(db);
    let (st, _) = call(app.clone(), None, ORM_EVENT).await;
    assert!(st == StatusCode::UNAUTHORIZED, "ORM 版私有日历事件无会话须 401，实得 {st}");
    let (st, _) = call(app.clone(), Some("bob"), ORM_EVENT).await;
    assert!(st == StatusCode::FORBIDDEN, "ORM 版 bob 读 alice 私有事件须 403，实得 {st}");
    let (st, v) = call(app.clone(), Some("alice"), ORM_EVENT).await;
    assert_eq!(st, StatusCode::OK, "ORM 版 alice 本人可读");
    assert!(event_titles(&v).iter().any(|t| t == CANARY), "alice 应见唯一私密标记事件");
    let (st, v) = call(app.clone(), Some("admin"), ORM_EVENT).await;
    assert_eq!(st, StatusCode::OK, "ORM 版 admin 可读");
    assert!(event_titles(&v).iter().any(|t| t == CANARY), "admin 应见私密标记事件");
    let (st, v) = call(app, Some("bob"), "/api/calendar/core/entity/event/list/public-cal").await;
    assert_eq!(st, StatusCode::OK, "ORM 版公开日历事件登录用户可读");
    assert!(event_titles(&v).iter().any(|t| t == "public-event"), "公开事件可见");
    drop_scratch(DB).await;
}

#[tokio::test]
async fn source_guard_gate_present_and_event_query_unchanged() {
    let sql_src = std::fs::read_to_string("crates/calendar/src/lib.rs").expect("读 SQL 源码");
    let core_src = std::fs::read_to_string("crates/calendar_core_entity/src/lib.rs").expect("读 ORM 源码");
    assert!(sql_src.contains("require_owner(&pool, &sess, &createor)"), "SQL 版归属门须在");
    assert!(sql_src.contains("FROM CAL_EVENT WHERE calendar_id = $1 AND status = 'OPEN' ORDER BY start_time ASC"), "SQL 事件查询零改");
    assert!(core_src.contains("require_owner(&db.0, &owner, &actor)"), "ORM 版归属门须在");
    assert!(core_src.contains("cal_event::Column::Status.eq(\"OPEN\")"), "ORM 事件查询零改");
}
