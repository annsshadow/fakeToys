//! 安全审计回归（round4 第七十九批 · ORM 盲区系统复查)：bbs_core_entity 9 把
//! sea_orm CRUD 真写收会话门。
//!
//! forum/section/subject 三族的 create/update/delete 共 9 把（`create_reply`
//! 为**功能桩**——不落任何库，false-positive 不收门，记 §五）均经 sea_orm
//! `ActiveModel` 落笔，此前无会话门禁。这些是 ORM 写，审计尺子"含写"指标
//! 不可见（ORM-blindness，§五 尺子缺陷 28）——R7 降 9 但"含写"不变属预期；
//! 本批以离线 401 臂交叉验证。
//!
//! 注：本 crate 的 router 从不提供 `Extension<DatabaseConnection>`（§五 遗留 30
//! 系统性接线缺陷）⇒ HTTP 面 extraction 先失败（500），本批只做静态收门 +
//! 直调臂验证（与 b65/b70/b72/b78 同型）。
//!
//! 证据臂（直调 handler，绕过 extraction）：
//! 1. 真 sea_orm 连接 + `None` 会话 → 9 把全部 `Err(AppError::Unauthorized)`，
//!    门在 DB 访问前返回，绝不落查询。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b79_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;
    let path = || axum::extract::Path("b79-absent".to_string());
    let body = || Json(serde_json::json!({}));

    // forum
    assert!(
        matches!(
            create_forum(Extension(db.0.clone()), none.clone(), body()).await,
            Err(AppError::Unauthorized)
        ),
        "create_forum 无会话必须 401"
    );
    assert!(
        matches!(
            update_forum(Extension(db.0.clone()), none.clone(), path(), body()).await,
            Err(AppError::Unauthorized)
        ),
        "update_forum 无会话必须 401"
    );
    assert!(
        matches!(
            delete_forum(Extension(db.0.clone()), none.clone(), path()).await,
            Err(AppError::Unauthorized)
        ),
        "delete_forum 无会话必须 401"
    );

    // section
    assert!(
        matches!(
            create_section(Extension(db.0.clone()), none.clone(), body()).await,
            Err(AppError::Unauthorized)
        ),
        "create_section 无会话必须 401"
    );
    assert!(
        matches!(
            update_section(Extension(db.0.clone()), none.clone(), path(), body()).await,
            Err(AppError::Unauthorized)
        ),
        "update_section 无会话必须 401"
    );
    assert!(
        matches!(
            delete_section(Extension(db.0.clone()), none.clone(), path()).await,
            Err(AppError::Unauthorized)
        ),
        "delete_section 无会话必须 401"
    );

    // subject
    assert!(
        matches!(
            create_subject(Extension(db.0.clone()), none.clone(), body()).await,
            Err(AppError::Unauthorized)
        ),
        "create_subject 无会话必须 401"
    );
    assert!(
        matches!(
            update_subject(Extension(db.0.clone()), none.clone(), path(), body()).await,
            Err(AppError::Unauthorized)
        ),
        "update_subject 无会话必须 401"
    );
    assert!(
        matches!(
            delete_subject(Extension(db.0), none, path()).await,
            Err(AppError::Unauthorized)
        ),
        "delete_subject 无会话必须 401"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b79_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b79-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b79-unit");
}
