// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R622b 真库 oneshot：采纳端点必须在事务内校验 reply 属于 subject，
//! 否则主题 owner 可用其它主题的 reply 污染本主题 accept_reply_id 并误标他人 reply。
//! RED：真父版本缺 reply↔topic 绑定，跨主题采纳返回 success 且落库污染。
//! GREEN：修复版返回 error，且两表零写。

use axum::extract::{Extension, Path};
use bbs_assemble_control::u2::{
    u2_subject_accept_reply, u2_subject_unaccept_reply, u2_user_reply_accept,
};
use deadpool_postgres::Pool;
use serde_json::json;
use shared::response::ActionResult;
use tower::util::ServiceExt;

const SCRATCH: &str = "oa_r622b_fix";

fn pool_to(dbname: &str) -> Pool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let mut cfg: deadpool_postgres::tokio_postgres::Config = url.parse().expect("parse url");
    cfg.dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(4).build().unwrap()
}

async fn fresh_scratch(dbname: &str) -> Pool {
    let admin = pool_to("postgres");
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
    let pool = pool_to(dbname);
    shared::migrate::run_migrations(&pool)
        .await
        .expect("scratch migrations");
    pool
}

async fn drop_scratch(dbname: &str) {
    if let Ok(c) = pool_to("postgres").get().await {
        let _ = c
            .execute(
                &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                &[],
            )
            .await;
    }
}

fn session(person: &str) -> shared::session::Session {
    shared::testing::test_session(person)
}

fn is_success<T>(v: &ActionResult<T>) -> bool {
    v.r#type.as_deref() == Some("success")
}
fn is_error<T>(v: &ActionResult<T>) -> bool {
    v.r#type.as_deref() == Some("error")
}
fn raw<T: serde::Serialize>(v: &ActionResult<T>) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("scratch conn");
    c.batch_execute(concat!(
        "INSERT INTO x_bbs_topic (id,title,author_id,section_id,creator,create_time) VALUES ",
        "('tA','topic A','r622b-owner','s1','r622b-owner','2026-01-01 00:00:00'), ",
        "('tB','topic B','r622b-other','s1','r622b-other','2026-01-01 00:00:00');",
        "INSERT INTO x_bbs_reply (id,subject_id,topic_id,author_id,creator,content,create_time) VALUES ",
        "('rA','tA','tA','r622b-owner','r622b-owner','reply A','2026-01-01 00:00:00'), ",
        "('rA2','tA','tA','r622b-owner','r622b-owner','reply A2','2026-01-01 00:00:00'), ",
        "('rB','tB','tB','r622b-other','r622b-other','reply B','2026-01-01 00:00:00');",
    )).await.expect("seed topics/replies");
    drop(c);
}

async fn topic_accept(pool: &Pool, topic: &str) -> Option<String> {
    let c = pool.get().await.expect("conn");
    let r = c
        .query_opt(
            "SELECT accept_reply_id FROM x_bbs_topic WHERE id=$1",
            &[&topic.to_string()],
        )
        .await
        .expect("select topic");
    r.and_then(|row| row.get::<_, Option<String>>("accept_reply_id"))
}

async fn reply_accepted(pool: &Pool, reply_id: &str) -> bool {
    let c = pool.get().await.expect("conn");
    let r = c
        .query_one(
            "SELECT accepted FROM x_bbs_reply WHERE id=$1",
            &[&reply_id.to_string()],
        )
        .await
        .expect("select reply");
    r.get::<_, bool>("accepted")
}

#[tokio::test]
async fn r622b_cross_topic_rejected_without_writes() {
    let pool = fresh_scratch(SCRATCH).await;
    seed(&pool).await;

    // 臂① GET 端点：主题 A owner 传主题 B 的 reply id → 必须 error 且两表零写
    let r = u2_subject_accept_reply(
        Extension(pool.clone()),
        Extension(session("r622b-owner")),
        Path(("tA".to_string(), "rB".to_string())),
    )
    .await
    .expect("handler GET cross-topic");
    println!("R622B_ARM1_GET_CROSS raw={}", raw(&r));
    assert!(
        is_error(&r),
        "跨主题回帖必须被拒（error），真父缺陷返回 success: {}",
        raw(&r)
    );
    assert_eq!(
        topic_accept(&pool, "tA").await.as_deref(),
        None,
        "跨主题不得写 tA.accept_reply_id"
    );
    assert!(
        !reply_accepted(&pool, "rB").await,
        "跨主题不得置 rB.accepted"
    );

    // 臂①b PUT 端点同样拒绝且零写
    let r = u2_user_reply_accept(
        Extension(pool.clone()),
        Extension(session("r622b-owner")),
        axum::Json(json!({"subjectId":"tA","replyId":"rB"})),
    )
    .await
    .expect("handler PUT cross-topic");
    println!("R622B_ARM1B_PUT_CROSS raw={}", raw(&r));
    assert!(is_error(&r), "PUT 跨主题回帖必须被拒: {}", raw(&r));
    assert_eq!(topic_accept(&pool, "tA").await.as_deref(), None);
    assert!(!reply_accepted(&pool, "rB").await);

    // 反臂④ 同主题本人 GET → success 且两写落库
    let r = u2_subject_accept_reply(
        Extension(pool.clone()),
        Extension(session("r622b-owner")),
        Path(("tA".to_string(), "rA".to_string())),
    )
    .await
    .expect("handler GET own");
    assert!(is_success(&r), "同主题本人采纳应 success: {}", raw(&r));
    assert_eq!(topic_accept(&pool, "tA").await.as_deref(), Some("rA"));
    assert!(reply_accepted(&pool, "rA").await);
    println!("R622B_ARM4_OWN_GET=PASS {}", raw(&r));

    // 反臂④b 同主题本人 PUT 另一回帖 rA2 → success
    let r = u2_user_reply_accept(
        Extension(pool.clone()),
        Extension(session("r622b-owner")),
        axum::Json(json!({"subjectId":"tA","replyId":"rA2"})),
    )
    .await
    .expect("handler PUT own");
    assert!(is_success(&r), "PUT 同主题本人应 success: {}", raw(&r));
    assert_eq!(topic_accept(&pool, "tA").await.as_deref(), Some("rA2"));
    assert!(reply_accepted(&pool, "rA2").await);
    println!("R622B_ARM4B_OWN_PUT=PASS {}", raw(&r));

    // 反臂③ 非属主 PUT → 403
    let r = u2_user_reply_accept(
        Extension(pool.clone()),
        Extension(session("r622b-other")),
        axum::Json(json!({"subjectId":"tA","replyId":"rA"})),
    )
    .await;
    assert!(
        matches!(r, Err(shared::error::AppError::Forbidden)),
        "非属主必须 403"
    );
    println!("R622B_ARM3_NONOWNER_403=PASS");

    // 反臂 取消采纳清标记（兄弟惯例不受影响）
    let r = u2_subject_unaccept_reply(
        Extension(pool.clone()),
        Extension(session("r622b-owner")),
        Path("tA".to_string()),
    )
    .await
    .expect("handler unaccept");
    assert!(is_success(&r));
    assert_eq!(topic_accept(&pool, "tA").await.as_deref(), None);
    assert!(!reply_accepted(&pool, "rA2").await, "取消采纳应清回帖标记");
    println!("R622B_UNACCEPT_CLEAR=PASS");

    drop_scratch(SCRATCH).await;
}

#[tokio::test]
async fn r622b_bare_session_middleware_401() {
    let state = shared::middleware::security::SecurityState {
        session_manager: shared::session::SessionManager::new(),
        rate_limiter: shared::rate_limit::RateLimiter::new(),
        pool: shared::testing::mock_pool(),
    };
    let app = axum::Router::new()
        .route(
            "/api/bbs_assemble_control/user/reply/accept",
            axum::routing::put(|| async { "ok" }),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            shared::middleware::auth_middleware,
        ));
    let res = app
        .oneshot(
            axum::http::Request::builder()
                .method("PUT")
                .uri("/api/bbs_assemble_control/user/reply/accept")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");
    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    println!("R622B_ARM2_BARE_SESSION_401=PASS");
}
