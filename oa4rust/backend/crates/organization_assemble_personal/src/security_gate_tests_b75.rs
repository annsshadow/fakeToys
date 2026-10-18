//! 安全审计回归（round4 第七十五批）：org_personal 自定义字段更新真写收会话门。
//!
//! `custom_mockputtopost`（POST `/api/organization/assemble/personal/custom/{id}/mockputtopost`，
//! `UPDATE x_org_custom SET field_value = $2 WHERE id = $1 AND deleted_at IS NULL`）
//! 此前无会话门禁：任何匿名调用方即可改写自定义字段值。本批加"会话存在档"
//! 门禁（`require_session`，无会话 401，门在首个 DB 访问 `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在 `pool.get()` 之前，绝不触库）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且只挡匿名不挡已认证。
//!
//! 注：UPDATE 仅按 id 过滤、无属主条件（IDOR 收紧残留，列 §五）——任意已认证
//! 用户可改任意 custom 行；本批先抹匿名写面。

#[cfg(test)]
mod security_gate_tests_b75 {
    use crate::custom_mockputtopost;
    use axum::extract::Extension;
    use deadpool_postgres::Pool;
    use serde_json::json;
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
    async fn b75_writes_reject_without_session() {
        let pool = dead_pool();

        let res = custom_mockputtopost(
            Extension(pool),
            None,
            axum::extract::Path("b75-id".to_string()),
            axum::extract::Json(json!({"fieldValue": "v"})),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 custom_mockputtopost 必须 401（门在 pool.get() 之前）"
        );
    }

    #[tokio::test]
    async fn b75_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let session = Some(Extension(build_session("b75-valid")));

        // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
        let res = custom_mockputtopost(
            Extension(pool),
            session,
            axum::extract::Path("b75-id".to_string()),
            axum::extract::Json(json!({"fieldValue": "v"})),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Internal)),
            "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
        );
    }

    /// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
    #[test]
    fn b75_require_session_gate_logic() {
        let none: Option<Extension<shared::session::Session>> = None;
        assert!(
            matches!(crate::require_session(none), Err(AppError::Unauthorized)),
            "require_session：无会话必须 Unauthorized"
        );

        let some = Some(Extension(build_session("b75-unit")));
        let sess = crate::require_session(some).expect("require_session：有会话必须放行");
        assert_eq!(sess.person_unique, "b75-unit");
    }
}
