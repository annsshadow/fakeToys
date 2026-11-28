// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R633: meeting crate add_participant 属主/存在校验真修复契约。
//! 真父缺陷: require_session 只验会话存在(:489)，INSERT x_meeting_invite(:499) 无属主门、不校验会议存在。
//! 修复后: 首个 DB 写前 SELECT creator FROM x_meeting WHERE id=$1，不存在→NotFound，
//!         再 shared::middleware::require_owner 跨会议非属主→Forbidden；本人/admin 200 真落库。

use axum::body::Body;
use axum::extract::{Extension, Path};
use axum::http::{Method, Request, StatusCode};
use axum::middleware::from_fn_with_state;
use deadpool_postgres::{Manager, Pool};
use meeting::add_participant;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::middleware::SecurityState;
use shared::session::SessionManager;
use shared::testing::test_session;
use tower::ServiceExt;

const SCRATCH: &str = "oa_r633_meeting_fix";
const OWNER: &str = "r633-owner";
const ALICE: &str = "r633-alice";
const ADMIN: &str = "r633-admin";
const ROLE: &str = "r633-admin-role";
const BOB_MEETING: &str = "r633-bob-meeting";
const GHOST_MEETING: &str = "r633-ghost-meeting";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let mut cfg: deadpool_postgres::tokio_postgres::Config =
        url.parse().expect("parse DATABASE_URL");
    cfg.dbname(dbname);
    let mgr = Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pool_to("postgres");
    let c = admin.get().await.expect("admin conn");
    let sql = format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)");
    c.execute(&sql, &[]).await.expect("drop scratch");
    let sql = format!("CREATE DATABASE {dbname}");
    c.execute(&sql, &[]).await.expect("create scratch");
    drop(c);
    let pool = pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch migrations");
    pool
}

async fn drop_scratch(dbname: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let sql = format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)");
        let _ = c.execute(&sql, &[]).await;
    }
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.execute(
        "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
        &[&ROLE.to_string()],
    ).await.expect("seed role");
    shared::testing::upsert_fixture_person(&c, ADMIN, ADMIN, "r633 admin").await;
    c.execute(
        "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1,$2,'r633-unit') ON CONFLICT DO NOTHING",
        &[&ADMIN.to_string(), &ROLE.to_string()],
    ).await.expect("seed admin role");
    c.execute(
        "INSERT INTO x_meeting (id,title,start_time,end_time,creator) VALUES ($1,$2,NOW(),NOW(),$3) ON CONFLICT (id) DO NOTHING",
        &[&BOB_MEETING, &"Bob meeting", &OWNER],
    ).await.expect("seed bob meeting");
    drop(c);
}

async fn invite(
    pool: &Pool,
    who: Option<&str>,
    meeting_id: &str,
    invitee: &str,
) -> Result<Value, AppError> {
    let session = who.map(|p| Extension(test_session(p)));
    let body = json!({"invitee": invitee});
    let resp = add_participant(
        Extension(pool.clone()),
        session,
        Path(meeting_id.to_string()),
        axum::extract::Json(body),
    )
    .await?;
    Ok(serde_json::to_value(&*resp).expect("serialize envelope"))
}

fn data_of(env: &Value) -> Value {
    env.get("data").cloned().expect("data in envelope")
}

async fn invites_of(pool: &Pool, meeting_id: &str, invitee: &str) -> i64 {
    let c = pool.get().await.expect("conn");
    c.query_one(
        "SELECT COUNT(*) AS n FROM x_meeting_invite WHERE meeting_id=$1 AND invitee=$2",
        &[&meeting_id, &invitee],
    )
    .await
    .expect("count invite")
    .get::<_, i64>("n")
}

#[tokio::test]
async fn add_participant_owner_and_existence_gate() {
    let pool = fresh_scratch(SCRATCH).await;
    seed(&pool).await;

    // 臂1: alice（非会议属主）对 bob 会议注入 invitee → Forbidden，DB 未改
    let r1 = invite(&pool, Some(ALICE), BOB_MEETING, "r633-victim").await;
    assert!(
        matches!(r1, Err(AppError::Forbidden)),
        "跨会议非属主必须 403: {r1:?}"
    );
    assert_eq!(
        invites_of(&pool, BOB_MEETING, "r633-victim").await,
        0,
        "跨会议非属主不得落库"
    );
    println!("R633_ARM1_CROSS_FORBIDDEN=403/no_write");

    // 臂2: 不存在的 meeting_id → NotFound，无幽灵邀请
    let r2 = invite(&pool, Some(ALICE), GHOST_MEETING, "r633-ghost").await;
    assert!(
        matches!(r2, Err(AppError::NotFound)),
        "不存在会议必须 404: {r2:?}"
    );
    assert_eq!(
        invites_of(&pool, GHOST_MEETING, "r633-ghost").await,
        0,
        "不得产生幽灵邀请"
    );
    println!("R633_ARM2_GHOST_NOTFOUND=404/no_write");

    // 臂3: 会议属主本人邀请 → success，真落库
    let raw3 = invite(&pool, Some(OWNER), BOB_MEETING, "r633-owner-invitee")
        .await
        .expect("owner invite");
    assert_eq!(raw3.get("type").and_then(|v| v.as_str()), Some("success"));
    assert_eq!(
        data_of(&raw3).get("added").and_then(|v| v.as_bool()),
        Some(true)
    );
    assert_eq!(
        invites_of(&pool, BOB_MEETING, "r633-owner-invitee").await,
        1
    );
    println!("R633_ARM3_OWNER=success/row=1");

    // 臂4: admin 跨会议邀请 → success，真落库
    let raw4 = invite(&pool, Some(ADMIN), BOB_MEETING, "r633-admin-invitee")
        .await
        .expect("admin invite");
    assert_eq!(raw4.get("type").and_then(|v| v.as_str()), Some("success"));
    assert_eq!(
        invites_of(&pool, BOB_MEETING, "r633-admin-invitee").await,
        1
    );
    println!("R633_ARM4_ADMIN=success/row=1");

    // 臂5: 无会话直调 → Unauthorized
    let r5 = invite(&pool, None, BOB_MEETING, "r633-anon").await;
    assert!(matches!(r5, Err(AppError::Unauthorized)), "无会话必须 401");
    println!("R633_ARM5_NO_SESSION=401");

    // 源码守卫: 修复含前置存在校验 + 共享归属门
    let this_src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("meeting lib");
    assert!(
        this_src.contains("SELECT creator FROM x_meeting WHERE id = $1"),
        "修复须含前置存在校验"
    );
    assert!(
        this_src.contains("require_owner(&pool.0, &session, &creator)"),
        "修复须复用共享归属门"
    );
    println!("R633_SOURCE_GUARD=existence_and_owner_gate=1");

    drop_scratch(SCRATCH).await;
}

#[tokio::test]
async fn unauthenticated_request_is_401_at_app_layer() {
    let state = SecurityState {
        session_manager: SessionManager::new(),
        rate_limiter: shared::RateLimiter::new(),
        pool: pool_to("postgres"),
    };
    let app = meeting::routes::meeting_router(pool_to("postgres")).layer(from_fn_with_state(
        state,
        shared::middleware::auth_middleware,
    ));
    let req = Request::builder()
        .method(Method::POST)
        .uri("/api/meeting/r633-bob-meeting/participant/add")
        .header("content-type", "application/json")
        .body(Body::from("{\"invitee\":\"r633-anon\"}"))
        .unwrap();
    let resp = app.oneshot(req).await.expect("oneshot");
    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "app 层裸请求必须 401"
    );
    println!("R633_UNAUTH_APP_LAYER=401");
}
