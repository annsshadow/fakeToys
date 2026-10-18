//! 安全审计回归（round4 第七十四批）：OAuth 授权码签发真写收会话门。
//!
//! `oauth_generate_code`（POST `/api/organization/assemble/authentication/oauth/generate/code`，
//! 经 `oauth_code_store` INSERT `x_org_oauth_code`）此前无会话门禁：任何匿名
//! 调用方即可签发 OAuth 授权码。本批加"会话存在档"门禁（`require_session`，
//! 无会话 401，门在首个 DB 访问 `pool.get()` 之前）。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在 `pool.get()` 之前，绝不触库）。
//! 2. 死池 + 有效会话 → 过门后直落 `pool.get()` 必败 → `Internal`，
//!    证明门确实在库前、且只挡匿名不挡已认证。
//!
//! 注：clientId 未与已注册应用比对、personId 取自 body（可伪造）——属 §五
//! "第三方回调零验签族"，本批按"加门与验签分开记"只收会话档，验签列遗留。

#[cfg(test)]
mod security_gate_tests_b74 {
    use crate::oauth_generate_code;
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
    async fn b74_writes_reject_without_session() {
        let pool = dead_pool();

        let res = oauth_generate_code(Extension(pool), None, axum::extract::Json(json!({}))).await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 oauth_generate_code 必须 401（门在 pool.get() 之前）"
        );
    }

    #[tokio::test]
    async fn b74_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let session = Some(Extension(build_session("b74-valid")));

        // 有效会话过门后直落死池 pool.get() 必败 → Internal（证明门在库前、只挡匿名）。
        let res = oauth_generate_code(Extension(pool), session, axum::extract::Json(json!({}))).await;
        assert!(
            matches!(res, Err(AppError::Internal)),
            "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
        );
    }

    /// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
    #[test]
    fn b74_require_session_gate_logic() {
        let none: Option<Extension<shared::session::Session>> = None;
        assert!(
            matches!(crate::require_session(none), Err(AppError::Unauthorized)),
            "require_session：无会话必须 Unauthorized"
        );

        let some = Some(Extension(build_session("b74-unit")));
        let sess = crate::require_session(some).expect("require_session：有会话必须放行");
        assert_eq!(sess.person_unique, "b74-unit");
    }
}
