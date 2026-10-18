//! 安全审计回归（round4 第八十一批 · ORM 盲区系统复查)：mind_core_entity 7 把
//! sea_orm CRUD 真写收会话门。
//!
//! mind/folder 两族 × create/update/delete + version create 共 7 把均经 sea_orm
//! `ActiveModel` 落笔，此前无会话门禁。这些是 ORM 写，审计尺子"含写"指标
//! 不可见（ORM-blindness，§五 尺子缺陷 28）——R7 降 7 但"含写"不变属预期；
//! 本批以离线 401 臂交叉验证。
//!
//! 注：本 crate 的 router 从不提供 `Extension<DatabaseConnection>`（§五 遗留 30
//! 系统性接线缺陷）⇒ HTTP 面 extraction 先失败（500），本批只做静态收门 +
//! 直调臂验证（与 b78/b79/b80 同型）。
//!
//! 证据臂（直调 handler，绕过 extraction）：
//! 1. 真 sea_orm 连接 + `None` 会话 → 7 把全部 `Err(AppError::Unauthorized)`，
//!    门在 DB 访问前返回，绝不落查询。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b81_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;
    let path = || axum::extract::Path("b81-absent".to_string());
    let body = || Json(serde_json::json!({}));

    assert!(matches!(
        create_mind(Extension(db.0.clone()), none.clone(), body()).await,
        Err(AppError::Unauthorized)
    ), "create_mind 无会话必须 401");
    assert!(matches!(
        update_mind(Extension(db.0.clone()), none.clone(), path(), body()).await,
        Err(AppError::Unauthorized)
    ), "update_mind 无会话必须 401");
    assert!(matches!(
        delete_mind(Extension(db.0.clone()), none.clone(), path()).await,
        Err(AppError::Unauthorized)
    ), "delete_mind 无会话必须 401");

    assert!(matches!(
        create_folder(Extension(db.0.clone()), none.clone(), body()).await,
        Err(AppError::Unauthorized)
    ), "create_folder 无会话必须 401");
    assert!(matches!(
        update_folder(Extension(db.0.clone()), none.clone(), path(), body()).await,
        Err(AppError::Unauthorized)
    ), "update_folder 无会话必须 401");
    assert!(matches!(
        delete_folder(Extension(db.0.clone()), none.clone(), path()).await,
        Err(AppError::Unauthorized)
    ), "delete_folder 无会话必须 401");

    assert!(matches!(
        create_version(Extension(db.0), none, body()).await,
        Err(AppError::Unauthorized)
    ), "create_version 无会话必须 401");
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b81_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b81-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b81-unit");
}
