//! 安全审计回归（round4 第七十三批）：message 已读标记真写收会话门。
//!
//! `mark_read`（POST `/api/message/mark_read/{id}`，`UPDATE x_msg_message SET
//! xconsumed = true WHERE xid = $1`）此前无会话门禁：任何匿名调用方即可把
//! 任意消息标为已读。本批加"会话存在档"门禁（`require_session`，无会话
//! 401，门在首个 DB 访问 `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在 `pool.get()` 之前，绝不触库）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且只挡匿名不挡已认证。
//!
//! 注：同族 `update_single`（UPDATE xconsumed 同表同列）不在已注册路由上
//! （gauge R7 未见），本批不动；若将来挂路由须同档收门。

#[cfg(test)]
mod security_gate_tests_b73 {
    use crate::mark_read;
    use axum::extract::Extension;
    use deadpool_postgres::Pool;
    use shared::error::AppError;

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
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
    async fn b73_writes_reject_without_session() {
        let pool = dead_pool();

        let res = mark_read(
            Extension(pool),
            None,
            axum::extract::Path("b73-id".to_string()),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 mark_read 必须 401（门在 pool.get() 之前）"
        );
    }

    #[tokio::test]
    async fn b73_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let session = Some(Extension(build_session("b73-valid")));

        // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
        let res = mark_read(
            Extension(pool),
            session,
            axum::extract::Path("b73-id".to_string()),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Internal)),
            "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
        );
    }

    /// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
    #[test]
    fn b73_require_session_gate_logic() {
        let none: Option<Extension<shared::session::Session>> = None;
        assert!(
            matches!(crate::require_session(none), Err(AppError::Unauthorized)),
            "require_session：无会话必须 Unauthorized"
        );

        let some = Some(Extension(build_session("b73-unit")));
        let sess = crate::require_session(some).expect("require_session：有会话必须放行");
        assert_eq!(sess.person_unique, "b73-unit");
    }
}
