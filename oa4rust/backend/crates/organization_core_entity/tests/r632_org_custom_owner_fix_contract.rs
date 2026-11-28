// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R632: x_org_custom 四端点身份盲 IDOR 真修复回归。
//! 真父(RED): custom_list 无会话裸读任意 identityId 字段值;
//!   custom_create/update/delete 只验会话存在, body/record 身份自证,
//!   任意登录者可往他人 identity 挂字段 / 凭 id 改名与软删。
//! 修复(GREEN): 四端点会话门 + 经 x_org_identity.creator 归属校验
//!   (本人 creator=actor 或 admin), 匿名 401, 非本人非 admin 403。

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Method, Request, StatusCode};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement, Value,
};
use serde_json::{json, Value as JsonValue};
use tower::ServiceExt;

const ALICE: &str = "r632-alice";
const BOB: &str = "r632-bob";
const ADMIN: &str = "r632-admin";
const SCRATCH: &str = "r632_owner_fix_contract";

fn url_with_db(base: &str, db: &str) -> String {
    match base.rfind('/') {
        Some(i) => format!("{}/{}", &base[..i], db),
        None => format!("{}/{}", base, db),
    }
}

async fn scratch_schema() {
    let base = std::env::var("DATABASE_URL").expect("DATABASE_URL must point at an isolated PG");
    let admin_cfg: deadpool_postgres::tokio_postgres::Config = base.parse().expect("DATABASE_URL");
    let mut acfg = admin_cfg.clone();
    acfg.dbname("postgres");
    let admin = deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
        acfg,
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .max_size(2)
    .build()
    .unwrap();
    let c = admin.get().await.unwrap();
    c.execute(
        &format!("DROP DATABASE IF EXISTS {SCRATCH} WITH (FORCE)"),
        &[],
    )
    .await
    .unwrap();
    c.execute(&format!("CREATE DATABASE {SCRATCH}"), &[])
        .await
        .unwrap();
    drop(c);
    let mut scfg = admin_cfg;
    scfg.dbname(SCRATCH);
    let spool = deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
        scfg,
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .max_size(4)
    .build()
    .unwrap();
    shared::migrate::run_migrations(&spool)
        .await
        .expect("scratch migrations");
}

async fn sea() -> DatabaseConnection {
    let base = std::env::var("DATABASE_URL").unwrap();
    let mut opts = ConnectOptions::new(url_with_db(&base, SCRATCH));
    opts.max_connections(5).sqlx_logging(false);
    Database::connect(opts).await.expect("sea connection")
}

async fn exec(db: &DatabaseConnection, sql: &str, params: Vec<Value>) {
    db.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        params,
    ))
    .await
    .expect("fixture SQL");
}

async fn seed(db: &DatabaseConnection) {
    for who in [ALICE, BOB, ADMIN] {
        exec(db, "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ($1,$1,$1,'r632-fixture') ON CONFLICT (id) DO NOTHING", vec![who.into()]).await;
    }
    exec(
        db,
        "INSERT INTO auth_role (id,name) VALUES ('r632-role','admin') ON CONFLICT (id) DO NOTHING",
        vec![],
    )
    .await;
    exec(db, "INSERT INTO auth_person_role (person_id,role_id,unit_id) VALUES ($1,'r632-role','r632-unit') ON CONFLICT DO NOTHING", vec![ADMIN.into()]).await;
    for (id, creator) in [("r632-alice-id", ALICE), ("r632-bob-id", BOB)] {
        exec(db, "INSERT INTO x_org_identity (id, person_id, name, type, creator) VALUES ($1,$1,$1,'r632seed',$2) ON CONFLICT (id) DO NOTHING", vec![id.into(), creator.into()]).await;
    }
    exec(db, "INSERT INTO x_org_custom (id,identity_id,field_name,field_value) VALUES ('r632-alice-rec','r632-alice-id','k','alice-value'),('r632-bob-rec','r632-bob-id','k','bob-value') ON CONFLICT (id) DO NOTHING", vec![]).await;
}

async fn call(
    db: &DatabaseConnection,
    who: Option<&str>,
    method: Method,
    uri: &str,
    body: Option<JsonValue>,
) -> (StatusCode, JsonValue) {
    let mut app = organization_core_entity::router(db.clone());
    if let Some(person) = who {
        app = app.layer(Extension(shared::testing::test_session(person)));
    }
    let req = Request::builder()
        .uri(uri)
        .method(method)
        .header("content-type", "application/json")
        .body(body.map_or_else(Body::empty, |b| Body::from(b.to_string())))
        .unwrap();
    let resp = app.oneshot(req).await.expect("oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap_or_default();
    let json = serde_json::from_slice(&bytes).unwrap_or(JsonValue::Null);
    (status, json)
}

async fn custom_value(db: &DatabaseConnection, id: &str) -> Option<String> {
    db.query_one(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT field_value FROM x_org_custom WHERE id = $1",
        vec![Value::from(id)],
    ))
    .await
    .expect("read custom value")
    .and_then(|r| r.try_get::<String>("", "field_value").ok())
}

async fn custom_deleted(db: &DatabaseConnection, id: &str) -> bool {
    db.query_one(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT deleted_at FROM x_org_custom WHERE id = $1",
        vec![Value::from(id)],
    ))
    .await
    .expect("read custom deleted_at")
    .and_then(|r| {
        r.try_get::<Option<chrono::NaiveDateTime>>("", "deleted_at")
            .ok()
            .flatten()
    })
    .is_some()
}

async fn custom_count(db: &DatabaseConnection, identity: &str) -> i64 {
    db.query_one(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT COUNT(*)::bigint AS n FROM x_org_custom WHERE identity_id = $1",
        vec![Value::from(identity)],
    ))
    .await
    .expect("count custom")
    .and_then(|r| r.try_get::<i64>("", "n").ok())
    .unwrap_or_default()
}

#[tokio::test]
async fn r632_xorg_custom_owner_gate() {
    scratch_schema().await;
    let db = sea().await;
    seed(&db).await;

    // 匿名四端点必须 401（custom_list 真父无会话 ⇒ 此臂 RED）。
    let (st, _) = call(
        &db,
        None,
        Method::GET,
        "/api/organization/custom/list/r632-bob-id",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "custom_list 匿名 401");
    let (st, _) = call(
        &db,
        None,
        Method::POST,
        "/api/organization/custom",
        Some(json!({"identity_id":"r632-bob-id","field_name":"k","field_value":"x"})),
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "custom_create 匿名 401");
    let (st, _) = call(
        &db,
        None,
        Method::PUT,
        "/api/organization/custom/r632-bob-rec",
        Some(json!({"field_name":"hacked"})),
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "custom_update 匿名 401");
    let (st, _) = call(
        &db,
        None,
        Method::DELETE,
        "/api/organization/custom/r632-bob-rec",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "custom_delete 匿名 401");

    // custom_list: 跨身份 403 + 无泄露; 本人/admin 200。
    let (st, body) = call(
        &db,
        Some(ALICE),
        Method::GET,
        "/api/organization/custom/list/r632-bob-id",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN, "alice 列 bob 字段必须 403");
    assert!(
        !body.to_string().contains("bob-value"),
        "不得泄露 bob 字段值"
    );
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::GET,
        "/api/organization/custom/list/r632-alice-id",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK, "本人列自己 200");
    let (st, _) = call(
        &db,
        Some(ADMIN),
        Method::GET,
        "/api/organization/custom/list/r632-bob-id",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 列他人 200");

    // custom_create: 跨身份 403 + DB 无新增; 本人/admin 200。
    let before = custom_count(&db, "r632-bob-id").await;
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::POST,
        "/api/organization/custom",
        Some(json!({"identity_id":"r632-bob-id","field_name":"injected","field_value":"x"})),
    )
    .await;
    assert_eq!(
        st,
        StatusCode::FORBIDDEN,
        "alice 往 bob identity 挂字段必须 403"
    );
    assert_eq!(
        custom_count(&db, "r632-bob-id").await,
        before,
        "bob 下不得新增记录"
    );
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::POST,
        "/api/organization/custom",
        Some(json!({"identity_id":"r632-alice-id","field_name":"self","field_value":"x"})),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "本人给自己挂字段 200");
    let (st, _) = call(
        &db,
        Some(ADMIN),
        Method::POST,
        "/api/organization/custom",
        Some(json!({"identity_id":"r632-bob-id","field_name":"adm","field_value":"x"})),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 给他人挂字段 200");

    // custom_update: 跨记录 403 + DB 未改; 本人/admin 200。
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::PUT,
        "/api/organization/custom/r632-bob-rec",
        Some(json!({"field_name":"hacked","field_value":"hacked"})),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN, "alice 改 bob 记录必须 403");
    assert_eq!(
        custom_value(&db, "r632-bob-rec").await.as_deref(),
        Some("bob-value"),
        "bob 记录不得被改"
    );
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::PUT,
        "/api/organization/custom/r632-alice-rec",
        Some(json!({"field_value":"self-updated"})),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "本人改自己 200");
    assert_eq!(
        custom_value(&db, "r632-alice-rec").await.as_deref(),
        Some("self-updated")
    );
    let (st, _) = call(
        &db,
        Some(ADMIN),
        Method::PUT,
        "/api/organization/custom/r632-bob-rec",
        Some(json!({"field_value":"admin-updated"})),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 改他人 200");

    // custom_delete: 跨记录 403 + 未软删; 本人/admin 200。
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::DELETE,
        "/api/organization/custom/r632-bob-rec",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN, "alice 删 bob 记录必须 403");
    assert!(
        !custom_deleted(&db, "r632-bob-rec").await,
        "bob 记录不得被软删"
    );
    let (st, _) = call(
        &db,
        Some(ALICE),
        Method::DELETE,
        "/api/organization/custom/r632-alice-rec",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK, "本人删自己 200");
    let (st, _) = call(
        &db,
        Some(ADMIN),
        Method::DELETE,
        "/api/organization/custom/r632-bob-rec",
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 删他人 200");
}

#[test]
fn r632_owner_gate_source_guard() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    assert!(
        src.contains("async fn ensure_identity_owner"),
        "必须有 identity 归属校验 helper"
    );
    for sig in [
        "pub async fn custom_list",
        "pub async fn custom_create",
        "pub async fn custom_update",
        "pub async fn custom_delete",
    ] {
        let s = src.find(sig).expect("find handler");
        let tail = &src[s + sig.len()..];
        let e = tail
            .find("pub async fn ")
            .map(|i| s + sig.len() + i)
            .unwrap_or(src.len());
        let seg = &src[s..e];
        assert!(seg.contains("require_session"), "{sig} 必须验会话");
        assert!(
            seg.contains("ensure_identity_owner"),
            "{sig} 必须调用 identity 归属校验"
        );
    }
}
