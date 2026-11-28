// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R630b: organization_assemble_express 组织属性/授权四个裸读端点归属门回归。
//! 真父: personattr/unitattr/empower 四读端点无 Session, body 身份不自证, 任意登录人可读他人属性/授权。
//! 修复: 四端点加 Session 门置首个 DB 访问前; personattr 复用 resolve_write_owner 语义
//!       (非本人且非 admin -> 403); unitattr/empower admin-only (x_org_unit_attribute/x_empower
//!       无 person 归属列, 参照 R616b 先例); 本人/admin 正常读保留。

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
    c.batch_execute(r#"INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('r630b-alice','r630b-alice','alice','x'),('r630b-bob','r630b-bob','bob','x'),('r630b-admin','r630b-admin','admin','x') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed persons");
    c.batch_execute(r#"INSERT INTO auth_role (id, name, deleted_at, disable) VALUES ('r630b-role','admin',NULL,false) ON CONFLICT (id) DO NOTHING;"#).await.expect("seed role");
    c.batch_execute(r#"INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r630b-admin','r630b-role','r630b-unit');"#).await.expect("seed admin");
    c.batch_execute(r#"INSERT INTO x_org_person (id,name) VALUES ('r630b-alice','alice'),('r630b-bob','bob') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed org persons");
    c.batch_execute(r#"INSERT INTO x_org_person_attribute (id,person_id,attribute_key,attribute_value) VALUES ('r630b-pa1','r630b-bob','secret','BOB_SECRET_VALUE'),('r630b-pa2','r630b-alice','secret','ALICE_VALUE') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed person attrs");
    c.batch_execute(r#"INSERT INTO x_org_unit_attribute (id,unit_id,attribute_key,attribute_value) VALUES ('r630b-ua1','unit-a','secret','UNIT_SECRET_VALUE') ON CONFLICT (id) DO NOTHING;"#).await.expect("seed unit attrs");
    c.batch_execute(r#"INSERT INTO x_empower (id,from_person,to_person,from_identity,to_identity,enabled) VALUES ('r630b-e1','r630b-bob','r630b-bob','r630b-bob','r630b-alice',true) ON CONFLICT (id) DO NOTHING;"#).await.expect("seed empower");
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
async fn r630b_personattr_cross_person_read_requires_owner_or_admin() {
    let pool = fresh_scratch("r630b_contract").await;
    seed(&pool).await;
    let app = router(pool.clone());
    let path = "/api/personattribute/list/attribute/person/name";
    // 真父 RED: alice 读 bob 属性 -> 200 且泄露 BOB_SECRET_VALUE; 修复后 -> 403。
    let (st, body) = call(
        &app,
        path,
        Some("r630b-alice"),
        json!({"person":"r630b-bob","name":"secret"}),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN, "跨人读属性必须 403, body={body}");
    assert!(!body.contains("BOB_SECRET_VALUE"), "不得泄露他人属性值");
    // 本人读自己 -> 200 且读到自己的值。
    let (st, body) = call(
        &app,
        path,
        Some("r630b-alice"),
        json!({"person":"r630b-alice","name":"secret"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "本人读自己 200");
    assert!(
        body.contains("ALICE_VALUE"),
        "本人读自己须返回值, body={body}"
    );
    // admin 跨人读 -> 200 且可见。
    let (st, body) = call(
        &app,
        path,
        Some("r630b-admin"),
        json!({"person":"r630b-bob","name":"secret"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 跨人读 200");
    assert!(body.contains("BOB_SECRET_VALUE"), "admin 可见他人属性");
}

#[tokio::test]
async fn r630b_read_endpoints_require_session_before_db() {
    let pool = fresh_scratch("r630b_contract_sess").await;
    seed(&pool).await;
    let app = router(pool.clone());
    // 四个裸读端点无会话一律 401, 门在 DB 访问前。
    let cases = [
        (
            "/api/personattribute/list/attribute/person/name",
            json!({"person":"r630b-bob","name":"secret"}),
        ),
        (
            "/api/personattribute/list/person/object",
            json!({"personList":["r630b-bob"]}),
        ),
        (
            "/api/unitattribute/list/unit/object",
            json!({"unitList":["unit-a"]}),
        ),
        (
            "/api/empower/list/identity/object",
            json!({"identityList":["r630b-bob"]}),
        ),
    ];
    for (path, body) in cases {
        let (st, _) = call(&app, path, None, body).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED, "裸调 {path} 必须 401");
    }
}

#[tokio::test]
async fn r630b_unitattr_and_empower_are_admin_only() {
    let pool = fresh_scratch("r630b_contract_admin").await;
    seed(&pool).await;
    let app = router(pool.clone());
    // 真父 RED: alice 读 unit/empower -> 200 泄露; 修复后 admin-only -> 403。
    let (st, body) = call(
        &app,
        "/api/unitattribute/list/unit/object",
        Some("r630b-alice"),
        json!({"unitList":["unit-a"]}),
    )
    .await;
    assert_eq!(
        st,
        StatusCode::FORBIDDEN,
        "非 admin 读 unit 属性 403, body={body}"
    );
    assert!(!body.contains("UNIT_SECRET_VALUE"), "不得泄露 unit 属性");
    let (st, body) = call(
        &app,
        "/api/empower/list/identity/object",
        Some("r630b-alice"),
        json!({"identityList":["r630b-bob"]}),
    )
    .await;
    assert_eq!(
        st,
        StatusCode::FORBIDDEN,
        "非 admin 读授权 403, body={body}"
    );
    assert!(!body.contains("r630b-bob"), "不得泄露授权记录");
    // admin 两端点 -> 200 且可见。
    let (st, body) = call(
        &app,
        "/api/unitattribute/list/unit/object",
        Some("r630b-admin"),
        json!({"unitList":["unit-a"]}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 读 unit 200");
    assert!(body.contains("UNIT_SECRET_VALUE"), "admin 可见 unit 属性");
    let (st, body) = call(
        &app,
        "/api/empower/list/identity/object",
        Some("r630b-admin"),
        json!({"identityList":["r630b-bob"]}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "admin 读授权 200");
    assert!(body.contains("r630b-bob"), "admin 可见授权记录");
}

#[test]
fn r630b_read_source_guard() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/endpoints_attr.rs"
    ))
    .expect("read endpoints_attr.rs");
    // 四个裸读端点都必须有 session 参数门。
    for name in [
        "personattr_list_attribute_person_name",
        "personattr_list_person_object",
        "unitattr_list_unit_object",
        "empower_list_identity_object",
    ] {
        let sig = format!("pub async fn {name}");
        let start = src.find(&sig).expect("fn exists");
        let search_from = start + sig.len();
        let end = src[search_from..]
            .find("pub async fn")
            .map(|i| search_from + i)
            .unwrap_or(src.len());
        let body = &src[start..end];
        assert!(
            body.contains("session: Option<Extension<shared::session::Session>>"),
            "{name} 必须接收 session, got={body:.200}"
        );
        assert!(
            body.contains("require_session(session)?"),
            "{name} 必须 require_session"
        );
    }
    assert!(
        src.contains("resolve_write_owner(&pool, &sess, &body, \"personId\", \"person\").await?"),
        "personattr 读须复用 resolve_write_owner 语义"
    );
    assert!(
        src.matches("shared::middleware::rbac::is_admin(&pool, &sess.person_unique).await")
            .count()
            >= 3,
        "unitattr/empower 须 admin-only 门"
    );
}
