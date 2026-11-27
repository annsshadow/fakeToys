// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R620a: organization_assemble_express 写端点身份绑定回归。
//! 真父: 五写端点只验会话存在, body person/personId/fromIdentity/toIdentity 自证,
//!       任意登录人可伪造他人写属性与授权日志。
//! 修复: owner 绑定会话人(admin 可写他人); 授权日志 from/to 非会话人需 admin;
//!       身份门在首个 DB 写之前; GET/路由零改。

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use deadpool_postgres::{Manager, Pool};
use organization_assemble_express::router;
use serde_json::{json, Value};
use shared::testing::test_session;
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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r620a-alice','r620a-alice','alice','x'),('r620a-bob','r620a-bob','bob','x'),('r620a-admin','r620a-admin','admin','x') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r620a-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r620a-admin','r620a-role','r620a-unit');"#).await.expect("seed admin");
    c.batch_execute(r#"INSERT INTO x_org_person (id,name) VALUES ('r620a-alice','alice'),('r620a-bob','bob'),('r620a-admin','admin') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed org persons");
    c.batch_execute(r#"INSERT INTO x_org_unit (id,name) VALUES ('r620a-ua','unit-a'),('r620a-ub','unit-b') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed units");
}

async fn scalar(
    pool: &Pool,
    sql: &str,
    params: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
) -> String {
    let c = pool.get().await.expect("scalar conn");
    let row = c.query_opt(sql, params).await.expect("scalar query");
    row.and_then(|r| r.get::<_, Option<String>>(0))
        .unwrap_or_default()
}

async fn call(app: &Router, path: &str, who: Option<&str>, body: Value) -> (StatusCode, String) {
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = match who {
        Some(p) => {
            app.clone()
                .layer(Extension(test_session(p)))
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
    (status, String::from_utf8_lossy(&bytes).to_string())
}

#[tokio::test]
async fn r620a_org_impersonation_is_gated_and_binding_enforced() {
    let dbname = "r620a_org_contract";
    let pool = fresh_scratch(dbname).await;
    seed(&pool).await;
    let app = router(pool.clone());

    // 裸请求: 无会话必须 401。
    let (st, _) = call(
        &app,
        "/api/personattribute/set/person/name",
        None,
        json!({"person":"r620a-bob","name":"k","attributeList":["v"]}),
    )
    .await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "裸请求 401");

    // 真父 RED: alice 伪造 bob 写 person 属性 -> 200 且 DB 落 bob 名下。
    // 修复后: 非本人且非 admin -> 403, DB 未改。
    let (st, _) = call(
        &app,
        "/api/personattribute/set/person/name",
        Some("r620a-alice"),
        json!({"person":"r620a-bob","personId":"r620a-bob","name":"k","attributeList":["v"]}),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN, "伪造他人写属性必须 403");
    let val = scalar(&pool, "SELECT attribute_value FROM x_org_person_attribute WHERE person_id=$1 AND attribute_key=$2 AND deleted_at IS NULL",
        &[&"r620a-bob", &"k"]).await;
    assert!(val.is_empty(), "bob 属性不得被 alice 篡改, got {val}");

    // 本人写自己 -> 200 且落自己名下。
    let (st, _) = call(
        &app,
        "/api/personattribute/set/person/name",
        Some("r620a-alice"),
        json!({"person":"r620a-bob","personId":"r620a-alice","name":"k","attributeList":["self"]}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "本人写自己 200");
    let val = scalar(&pool, "SELECT attribute_value FROM x_org_person_attribute WHERE person_id=$1 AND attribute_key=$2 AND deleted_at IS NULL",
        &[&"r620a-alice", &"k"]).await;
    assert_eq!(val, "self", "本人属性落 profile");

    // admin 反臂: 显式 body 指定他人 -> 200 且落他人名下。
    let (st, _) = call(
        &app,
        "/api/personattribute/append/person/name",
        Some("r620a-admin"),
        json!({"person":"r620a-bob","personId":"r620a-bob","name":"k","attributeList":["adm"]}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 写他人 200");
    let val = scalar(&pool, "SELECT attribute_value FROM x_org_person_attribute WHERE person_id=$1 AND attribute_key=$2 AND deleted_at IS NULL",
        &[&"r620a-bob", &"k"]).await;
    assert_eq!(val, "adm", "admin 属性落 bob");

    // unit 属性: 非 admin 伪造 unit -> 403。
    let (st, _) = call(
        &app,
        "/api/unitattribute/set/unit/name",
        Some("r620a-alice"),
        json!({"unit":"r620a-ub","unitId":"r620a-ub","name":"k","attributeList":["v"]}),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN, "伪造 unit 属性 403");

    // 授权日志: from/to 非会话人 -> 403。
    let (st, _) = call(&app, "/api/empowerlog", Some("r620a-alice"),
        json!({"application":"oa","process":"p1","fromIdentity":"r620a-bob","toIdentity":"r620a-bob"})).await;
    assert_eq!(st, StatusCode::FORBIDDEN, "伪造授权日志 403");
    let n: i64 = {
        let c = pool.get().await.unwrap();
        let r = c
            .query_one(
                "SELECT COUNT(*)::bigint AS n FROM x_org_empower_log WHERE from_identity=$1",
                &[&"r620a-bob"],
            )
            .await
            .unwrap();
        r.get("n")
    };
    assert_eq!(n, 0, "授权日志未伪造写入");

    // 授权日志: 本人作为 from -> 200 且落库。
    let (st, _) = call(&app, "/api/empowerlog", Some("r620a-alice"),
        json!({"application":"oa","process":"p1","fromIdentity":"r620a-alice","toIdentity":"r620a-bob"})).await;
    assert_eq!(st, StatusCode::OK, "本人授权日志 200");
    let n: i64 = {
        let c = pool.get().await.unwrap();
        let r = c
            .query_one(
                "SELECT COUNT(*)::bigint AS n FROM x_org_empower_log WHERE from_identity=$1",
                &[&"r620a-alice"],
            )
            .await
            .unwrap();
        r.get("n")
    };
    assert_eq!(n, 1, "本人授权日志落库");
}

#[test]
fn r620a_org_source_guard() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/endpoints_attr.rs"
    ))
    .expect("read endpoints_attr.rs");
    assert!(
        src.contains("shared::middleware::rbac::is_admin"),
        "必须复用共享 admin 判定"
    );
    assert!(src.contains("sess.person_unique"), "五写端点必须取会话身份");
    assert!(
        !src.contains("attr_write_values(&pool, &body,"),
        "owner 不得再从 body 自证"
    );
}
