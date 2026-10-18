//! 安全审计回归（round4 第八十三批 · ORM 盲区系统复查 · correlation_core_entity）：
//! 2 把 sea_orm CRUD 真写收会话门。
//!
//! `create`（POST `/api/correlation/core/entity/create`，INSERT correlation）与
//! `delete_by_id`（DELETE `/api/correlation/core/entity/delete/{id}`，DELETE）
//! 均经 sea_orm 落笔，此前无会话门禁。这些是 ORM 写，审计尺子"含写"指标
//! 不可见（ORM-blindness，§五 尺子缺陷 28）——R7 降 2 但"含写"不变属预期。
//!
//! 注：本 crate 的 router 从不提供 `Extension<DatabaseConnection>`（§五 遗留 30
//! 系统性接线缺陷）⇒ HTTP 面 extraction 先失败（500），本批只做静态收门 +
//! 直调臂验证（与 b78-b82 同型）。
//!
//! 证据臂（直调 handler，绕过 extraction）：
//! 1. 真 sea_orm 连接 + `None` 会话 → 2 把全部 `Err(AppError::Unauthorized)`，
//!    门在 DB 访问前返回，绝不落查询。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b83_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;

    assert!(
        matches!(
            create(
                Extension(db.0.clone()),
                none.clone(),
                Json(serde_json::json!({}))
            )
            .await,
            Err(AppError::Unauthorized)
        ),
        "create 无会话必须 401"
    );
    assert!(
        matches!(
            delete_by_id(
                Extension(db.0),
                none,
                axum::extract::Path("b83-absent".to_string())
            )
            .await,
            Err(AppError::Unauthorized)
        ),
        "delete_by_id 无会话必须 401"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b83_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b83-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b83-unit");
}
