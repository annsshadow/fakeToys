//! 安全审计回归（round4 第六十九批）：query_core_entity 视图创建真写收会话门。
//!
//! `view_create`（INSERT INTO QUERY_VIEW）此前无会话门禁：任何匿名调用方
//! 即可创建查询视图。本批加"会话存在档"门禁（`require_session`，无会话
//! 401，门在首个 DB 访问 `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在 `pool.get()` 之前，绝不触库）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且只挡匿名不挡已认证。

use super::*;

/// 空 JSON body（`Json<Value>` 参数占位）。
fn empty_body() -> axum::extract::Json<Value> {
    axum::extract::Json(serde_json::json!({}))
}

/// 死池：`pool.get()` 必败，用于离线证明"门在库前返回"。
fn dead_pool() -> Pool {
    let mgr = deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).build().unwrap()
}

#[tokio::test]
async fn b69_writes_reject_without_session() {
    let pool = dead_pool();

    let res = view_create(Extension(pool.clone()), None, empty_body()).await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 view_create 必须 401（门在 pool.get() 之前）"
    );
}

#[tokio::test]
async fn b69_writes_accept_valid_session_past_gate() {
    let pool = dead_pool();
    let session = Some(Extension(shared::testing::test_session("b69-valid")));

    // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
    let res = view_create(Extension(pool.clone()), session, empty_body()).await;
    assert!(
        matches!(res, Err(AppError::Internal)),
        "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b69_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b69-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b69-unit");
}
