//! 安全审计回归（round4 第七十批·calendar_assemble_control）：setting_create 真写收会话门。
//!
//! `setting_create`（POST `/api/calendar_assemble_control/setting`，INSERT
//! INTO cal_setting）此前无会话门禁：任何匿名调用方即可写设置行。本批加
//! "会话存在档"门禁（`require_session`，无会话 401，门在首个 DB 访问
//! `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在 `pool.get()` 之前，绝不触库）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且只挡匿名不挡已认证。

use super::u2::setting_create;
use axum::extract::{Extension, Json};
use deadpool_postgres::Pool;
use shared::error::AppError;

/// 死池：`pool.get()` 必败，用于离线证明"门在库前返回"。
fn dead_pool() -> Pool {
    let mgr = deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).build().unwrap()
}

#[tokio::test]
async fn b70_writes_reject_without_session() {
    let pool = dead_pool();

    let res = setting_create(Extension(pool), None, Json(serde_json::json!({}))).await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 setting_create 必须 401（门在 pool.get() 之前）"
    );
}

#[tokio::test]
async fn b70_writes_accept_valid_session_past_gate() {
    let pool = dead_pool();
    let session = Some(Extension(shared::testing::test_session("b70-valid")));

    // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
    let res = setting_create(Extension(pool), session, Json(serde_json::json!({}))).await;
    assert!(
        matches!(res, Err(AppError::Internal)),
        "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b70_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(super::u2::require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b70-unit")));
    let sess = super::u2::require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b70-unit");
}
