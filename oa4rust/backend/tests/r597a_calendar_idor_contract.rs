// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R597a 契约：日历 list/my 两实例（calendar SQL 版 + calendar_core_entity ORM 版）
//! 会话缺失必须 401；登录用户不得读到他人私有日历。
//! 用 router + tower oneshot 直调，签名在真父/修复两版稳定。
//! 两版归属口径差异据实：SQL 版 createor/target；ORM 版 is_public/createor/admin。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use calendar::calendar_router;
use calendar_core_entity::calendar_core_entity_router;
use deadpool_postgres::Pool;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use shared::session::Session;
use tower::ServiceExt;

const SQL_MY: &str = "/api/calendar/calendar/list/my";
const ORM_MY: &str = "/api/calendar/core/entity/calendar/list/my";
const SQL_PUBLIC: &str = "/api/calendar/calendar/list/public";

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
    c.execute(&format!("CREATE DATABASE {dbname}"), &[])
        .await.expect("create scratch");
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
        "token": "r597a-probe",
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

// 两版响应字段命名不同：SQL 版蛇形 my_calendars/unit_calendars，ORM 版驼峰。
// 统一遍历 data 下所有数组收集 id，避免绑定具体键名。
fn all_ids(v: &Value) -> Vec<String> {
    let mut ids = Vec::new();
    if let Some(obj) = v.get("data").and_then(|d| d.as_object()) {
        for arr in obj.values().filter_map(|x| x.as_array()) {
            for item in arr {
                if let Some(id) = item.get("id").and_then(|i| i.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }
    }
    ids
}

async fn seed_sql(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO cal_calendar \
         (id, name, type, target, color, description, source, createor, is_public, status, create_time) \
         VALUES \
         ('alice-private', 'Alice Private', 'PERSON', '', '', NULL, NULL, 'alice', false, 'OPEN', '2026-01-01 00:00:00'), \
         ('bob-private', 'Bob Private', 'PERSON', '', '', NULL, NULL, 'bob', false, 'OPEN', '2026-01-02 00:00:00'), \
         ('alice-target', 'Shared To Alice', 'PERSON', 'alice', '', NULL, NULL, 'bob', false, 'OPEN', '2026-01-03 00:00:00'), \
         ('public-cal', 'Public Cal', 'UNIT', '', '', NULL, NULL, 'bob', true, 'OPEN', '2026-01-04 00:00:00')",
        &[],
    ).await.expect("seed cal_calendar");
}

async fn seed_orm(pool: &Pool) {
    let c = pool.get().await.expect("scratch 连接");
    c.execute(
        "INSERT INTO x_cal_calendar \
         (id, name, type, target, color, description, source, createor, is_public, status, create_time) \
         VALUES \
         ('alice-private', 'Alice Private', 'PERSON', '', '', NULL, NULL, 'alice', false, 'OPEN', '2026-01-01 00:00:00'), \
         ('bob-private', 'Bob Private', 'PERSON', '', '', NULL, NULL, 'bob', false, 'OPEN', '2026-01-02 00:00:00'), \
         ('alice-target', 'Shared To Alice', 'PERSON', 'alice', '', NULL, NULL, 'bob', false, 'OPEN', '2026-01-03 00:00:00'), \
         ('public-cal', 'Public Cal', 'UNIT', '', '', NULL, NULL, 'bob', true, 'OPEN', '2026-01-04 00:00:00')",
        &[],
    ).await.expect("seed x_cal_calendar");
}

fn orm_db_url() -> String {
    let base = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".into());
    let mut u = url::Url::parse(&base).expect("DATABASE_URL 解析失败");
    u.set_path("/oa_r597a_orm");
    u.to_string()
}

#[tokio::test]
async fn sql_list_my_requires_session_and_filters_by_owner() {
    const DB: &str = "oa_r597a_sql";
    let pool = fresh_scratch(DB).await;
    seed_sql(&pool).await;
    let app = calendar_router(pool.clone());
    let (st, _) = call(app.clone(), None, SQL_MY).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "SQL 版 list/my 无会话必须 401");
    let (st, v) = call(app, Some("alice"), SQL_MY).await;
    assert_eq!(st, StatusCode::OK, "alice 带会话应成功");
    let ids = all_ids(&v);
    assert!(ids.contains(&"alice-private".to_string()), "应见本人 createor 私有行");
    assert!(ids.contains(&"alice-target".to_string()), "应见 target 指向本人行");
    assert!(!ids.contains(&"bob-private".to_string()), "alice 不得读 bob 私有（IDOR 核心）");
    assert!(!ids.contains(&"public-cal".to_string()), "SQL 版口径不含 is_public");
    drop_scratch(DB).await;
}

#[tokio::test]
async fn orm_list_my_requires_session_and_filters_by_owner() {
    const DB: &str = "oa_r597a_orm";
    let pool = fresh_scratch(DB).await;
    seed_orm(&pool).await;
    let db: DatabaseConnection = sea_orm::Database::connect(orm_db_url()).await.expect("sea-orm 连接");
    let app = calendar_core_entity_router(db);
    let (st, _) = call(app.clone(), None, ORM_MY).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "ORM 版 list/my 无会话必须 401");
    let (st, v) = call(app, Some("alice"), ORM_MY).await;
    assert_eq!(st, StatusCode::OK, "alice 带会话应成功");
    let ids = all_ids(&v);
    assert!(ids.contains(&"alice-private".to_string()), "应见本人 createor 私有行");
    assert!(ids.contains(&"public-cal".to_string()), "ORM 谓词含 is_public，公开应可见");
    assert!(!ids.contains(&"bob-private".to_string()), "alice 不得读 bob 私有（IDOR 核心）");
    assert!(!ids.contains(&"alice-target".to_string()), "ORM 谓词不含 target");
    drop_scratch(DB).await;
}

#[tokio::test]
async fn null_is_public_decodes_false_and_owner_filtered() {
    const DB: &str = "oa_r597a_null";
    let pool = fresh_scratch(DB).await;
    {
        let c = pool.get().await.expect("scratch 连接");
        c.execute(
            "INSERT INTO cal_calendar \
             (id, name, type, target, color, description, source, createor, is_public, status, create_time) \
             VALUES \
             ('null-row', 'Null Row', 'PERSON', '', '', NULL, NULL, 'alice', NULL, 'OPEN', '2026-01-01 00:00:00'), \
             ('bob-private', 'Bob Private', 'PERSON', '', '', NULL, NULL, 'bob', false, 'OPEN', '2026-01-02 00:00:00')",
            &[],
        ).await.expect("seed null 行");
    }
    let app = calendar_router(pool);
    let (st, v) = call(app, Some("alice"), SQL_MY).await;
    assert_eq!(st, StatusCode::OK, "is_public IS NULL 不应导致 panic");
    let ids = all_ids(&v);
    assert!(ids.contains(&"null-row".to_string()), "NULL 行本人可见");
    assert!(!ids.contains(&"bob-private".to_string()), "NULL 行仍须归属过滤");
    let row = v["data"].as_object()
        .and_then(|o| o.values().filter_map(|x| x.as_array()).flatten()
            .find(|x| x["id"] == "null-row"))
        .expect("定位 NULL 行");
    assert_eq!(row["isPublic"], Value::Bool(false), "is_public IS NULL 解码 false");
    drop_scratch(DB).await;
}

#[tokio::test]
async fn public_endpoint_and_predicate_unchanged() {
    const DB: &str = "oa_r597a_guard";
    let pool = fresh_scratch(DB).await;
    seed_sql(&pool).await;
    let app = calendar_router(pool.clone());
    let (st, v) = call(app, None, SQL_PUBLIC).await;
    assert_eq!(st, StatusCode::OK, "公开列表无需会话");
    let public = v["data"].as_array().expect("公开列表 data 数组");
    assert!(public.iter().any(|x| x["id"] == "public-cal"), "公开端点零改");
    assert!(!public.iter().any(|x| x["id"] == "bob-private"), "公开列表不含私有");
    let core_src = std::fs::read_to_string("crates/calendar_core_entity/src/lib.rs").expect("读源码");
    assert!(core_src.contains("CALENDAR_READ_PREDICATE"), "归属谓词常量须保留");
    drop_scratch(DB).await;
}
