//! 安全审计回归（round4 第七十六批）：personal 密码重置 mock 别名收会话门。
//!
//! `POST /api/reset/mockputtopost` 是 PUT `/api/reset`（自服务密码重置流程，
//! `AUTH_EXEMPT_PATHS` 在册、匿名可用）的 mock 别名。别名本身不在豁免表内 ⇒
//! 中间件要求会话可达，但 handler（`reset_password` → `apply_reset` →
//! UPDATE auth_person password_hash）体内无门禁。本批给别名加 wrapper 收
//! "会话存在档"（无会话 401，门在真实处理之前），**共享 handler 不动**——PUT
//! 规范路径的匿名重置流不受影响。
//!
//! 两臂离线证据：
//! 1. 死池 + 无会话 → 必须 `Unauthorized`（门在验证码校验 / DB 访问之前）。
//! 2. 死池 + 有效会话 + 已签发验证码 → 过门后密码规则过、验证码过、
//!    直落 `pool.get()` 必败 → `Internal`，证明门在库前、只挡匿名。
//!
//! §五 决策项：mock 别名与规范路径的豁免表不对齐（别名认证-only 形同死路），
//! 要么补豁免让别名可用（加宽前置面）、要么保持现状，应显式决策。

#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests_b76 {
    use crate::reset::{self, ResetCodeStore};
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

    fn reset_req(
        credential: &str,
        code: &str,
    ) -> axum::extract::Json<crate::reset::ResetPasswordRequest> {
        axum::extract::Json(crate::reset::ResetPasswordRequest {
            credential: credential.to_string(),
            code: code.to_string(),
            password: "abc12345".to_string(),
        })
    }

    #[tokio::test]
    async fn b76_writes_reject_without_session() {
        let pool = dead_pool();
        let store = Extension(ResetCodeStore::default());

        let res = reset::reset_password_mockputtopost(
            Extension(pool),
            store,
            None,
            reset_req("b76-cred", "any-code"),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用 /api/reset/mockputtopost 必须 401（门在验证码校验与 DB 之前）"
        );
    }

    #[tokio::test]
    async fn b76_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let store = Extension(ResetCodeStore::default());
        // 先签发一个真验证码（内存 store），确保过门后能走到 pool.get()。
        let code = store.0.issue("b76-cred").await;
        let session = Some(Extension(build_session("b76-valid")));

        // 过门 → 密码规则过 → 验证码校验并消费 → 死池 pool.get() 必败 → Internal。
        let res = reset::reset_password_mockputtopost(
            Extension(pool),
            store,
            session,
            reset_req("b76-cred", &code),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Internal)),
            "有效会话应过门、之后才落库（死池 → Internal），证门在库前"
        );
    }

    /// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
    #[test]
    fn b76_require_session_gate_logic() {
        let none: Option<Extension<shared::session::Session>> = None;
        assert!(
            matches!(reset::require_session(none), Err(AppError::Unauthorized)),
            "require_session：无会话必须 Unauthorized"
        );

        let some = Some(Extension(build_session("b76-unit")));
        let sess = reset::require_session(some).expect("require_session：有会话必须放行");
        assert_eq!(sess.person_unique, "b76-unit");
    }

    /// PUT /api/reset（匿名流）不经过 wrapper：仍然无会话可用（回归护栏）——
    /// 死池 + 无会话 + 已签发验证码 → 过密码与验证码后落死池 → Internal（而非 401）。
    #[tokio::test]
    async fn b76_put_canonical_route_stays_anonymous_capable() {
        let pool = dead_pool();
        let store = Extension(ResetCodeStore::default());
        let code = store.0.issue("b76-put").await;

        let res = reset::reset_password(Extension(pool), store, reset_req("b76-put", &code)).await;
        assert!(
            matches!(res, Err(AppError::Internal)),
            "PUT /api/reset 规范路径必须保持匿名可用（死池 → Internal，而非 401）"
        );
    }
}
