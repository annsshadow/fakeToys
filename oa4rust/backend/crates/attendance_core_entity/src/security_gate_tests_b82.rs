//! 安全审计回归（round4 第八十二批 · ORM 盲区系统复查)：attendance_core_entity
//! 4 把 sea_orm CRUD 真写收会话门。
//!
//! `record_create` / `record_update` / `rule_create` / `rule_update` 均经
//! sea_orm `ActiveModel` 落笔，此前无会话门禁。这些是 ORM 写，审计尺子"含写"
//! 指标不可见（ORM-blindness，§五 尺子缺陷 28）——R7 降 4 但"含写"不变属预期；
//! 本批以离线 401 臂交叉验证。
//!
//! 注：本 crate 的 router 从不提供 `Extension<DatabaseConnection>`（§五 遗留 30
//! 系统性接线缺陷）⇒ HTTP 面 extraction 先失败（500），本批只做静态收门 +
//! 直调臂验证（与 b78-b81 同型）。
//!
//! 证据臂（直调 handler，绕过 extraction）：
//! 1. 真 sea_orm 连接 + `None` 会话 → 4 把全部 `Err(AppError::Unauthorized)`，
//!    门在 DB 访问前返回，绝不落查询。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b82_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;
    let path = || axum::extract::Path("b82-absent".to_string());
    let body = || axum::extract::Json(serde_json::json!({}));

    assert!(matches!(
        record_create(Extension(db.0.clone()), none.clone(), body()).await,
        Err(AppError::Unauthorized)
    ), "record_create 无会话必须 401");
    assert!(matches!(
        record_update(Extension(db.0.clone()), none.clone(), path(), body()).await,
        Err(AppError::Unauthorized)
    ), "record_update 无会话必须 401");
    assert!(matches!(
        rule_create(Extension(db.0.clone()), none.clone(), body()).await,
        Err(AppError::Unauthorized)
    ), "rule_create 无会话必须 401");
    assert!(matches!(
        rule_update(Extension(db.0), none, path(), body()).await,
        Err(AppError::Unauthorized)
    ), "rule_update 无会话必须 401");
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b82_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b82-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b82-unit");
}
