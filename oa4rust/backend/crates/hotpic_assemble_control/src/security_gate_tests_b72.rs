//! 安全审计回归（round4 第七十二批·hotpic_assemble_control）：
//! user_hotpic_delete_by_ids 真写收会话门。
//!
//! `user_hotpic_delete_by_ids`（DELETE `/api/hotpic/assemble/control/user/hotpic/{id}/{id2}`，
//! `UPDATE x_hotpic SET deleted_at = NOW() WHERE id = ANY($1)`）此前无会话门禁：
//! 任何匿名调用方即可软删任意两条热图。本批加"会话存在档"门禁（沿用本 crate
//! `user_hotpic_changeTitle` 的 inline let-else 惯例，无会话 401，门在首个
//! DB 访问 `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在 `pool.get()` 之前，绝不触库）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且只挡匿名不挡已认证。

#[cfg(test)]
mod security_gate_tests_b72 {
    use crate::user_hotpic_delete_by_ids;
    use axum::extract::Extension;
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
    async fn b72_writes_reject_without_session() {
        let pool = dead_pool();

        let res = user_hotpic_delete_by_ids(
            Extension(pool),
            None,
            axum::extract::Path(("b72-a".to_string(), "b72-b".to_string())),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 user_hotpic_delete_by_ids 必须 401（门在 pool.get() 之前）"
        );
    }

    #[tokio::test]
    async fn b72_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let session = Some(Extension(shared::testing::test_session("b72-valid")));

        // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
        let res = user_hotpic_delete_by_ids(
            Extension(pool),
            session,
            axum::extract::Path(("b72-a".to_string(), "b72-b".to_string())),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Internal)),
            "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
        );
    }
}
