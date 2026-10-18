//! 安全审计回归（round4 第六十七批）：program_center invoke 执行真写收会话门。
//!
//! `u3_invoke_execute` / `u3_invoke_execute_with_token` 均会写
//! `x_program_invoke.last_start_time/last_end_time`（UPDATE），此前无会话门禁：
//! 任何匿名调用方 POST 一记即可触发写入。本批加"会话存在档"门禁
//! （`require_session`，无会话 401，门在首个 DB 访问 `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`，且因门在 `pool.get()` 之前，
//!    绝不触达数据库（死池也能返回 401 即是证明）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且门只挡匿名不挡已认证。

use super::*;

/// 死池：`pool.get()` 必败，用于离线证明"门在库前返回"。
fn dead_pool() -> Pool {
    let mgr = deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).build().unwrap()
}

#[tokio::test]
async fn b67_writes_reject_without_session() {
    let pool = dead_pool();

    let res = u3_invoke_execute(
        Extension(pool.clone()),
        None,
        axum::extract::Path("b67-flag".to_string()),
        axum::extract::Json(json!({})),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 u3_invoke_execute 必须 401（门在 pool.get() 之前）"
    );

    let res = u3_invoke_execute_with_token(
        Extension(pool.clone()),
        None,
        axum::extract::Path((
            "b67-flag".to_string(),
            "b67-client".to_string(),
            "b67-token".to_string(),
        )),
        axum::extract::Json(json!({})),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 u3_invoke_execute_with_token 必须 401（门在 pool.get() 之前）"
    );
}

#[tokio::test]
async fn b67_writes_accept_valid_session_past_gate() {
    let pool = dead_pool();
    let session = Some(Extension(shared::testing::test_session("b67-valid")));

    // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
    let res = u3_invoke_execute(
        Extension(pool.clone()),
        session.clone(),
        axum::extract::Path("b67-flag".to_string()),
        axum::extract::Json(json!({})),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Internal)),
        "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
    );

    let res = u3_invoke_execute_with_token(
        Extension(pool.clone()),
        session,
        axum::extract::Path((
            "b67-flag".to_string(),
            "b67-client".to_string(),
            "b67-token".to_string(),
        )),
        axum::extract::Json(json!({})),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Internal)),
        "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b67_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b67-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b67-unit");
}
