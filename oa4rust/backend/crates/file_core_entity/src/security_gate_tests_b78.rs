//! 安全审计回归（round4 第七十八批 · ORM 盲区系统复查首批）：file_core_entity
//! 3 把 sea_orm CRUD 真写收会话门。
//!
//! `folder_create`（INSERT file_folder）/ `folder_delete`（find + 软删 update）/
//! `file_create`（INSERT file_file）均经 sea_orm `ActiveModel` 落笔，此前无会话
//! 门禁。这些是 ORM 写，审计尺子"含写"指标不可见（ORM-blindness，§五 尺子缺陷
//! 28）——R7 降 3 但"含写"不变属预期；本批以离线 401 臂交叉验证。
//!
//! 注：本 crate（乃至全部 *_core_entity）的 router 在 B3 批已修复——接收
//! `sea_orm::DatabaseConnection` 并注入 `Extension<DatabaseConnection>`，HTTP 面
//! 不再在 extraction 期 500（接线前 routes 实为死路，见 §五 系统性接线缺陷项）。
//! 本批仍保留直调臂（绕过路由层，直接验证门在 DB 访问前生效）。
//!
//! 证据臂（直调 handler，绕过 extraction，与 b65/b70/b72 同型）：
//! 1. 真 sea_orm 连接 + `None` 会话 → 全部 `Err(AppError::Unauthorized)`，
//!    门在 DB 访问前返回，绝不落查询。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b78_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;

    let res = folder_create(
        Extension(db.0.clone()),
        none.clone(),
        Json(FolderCreateRequest {
            name: "b78".to_string(),
            person: "b78-person".to_string(),
            superior: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 folder_create 必须 401（门在 DB 访问前）"
    );

    let res = folder_delete(
        Extension(db.0.clone()),
        none.clone(),
        axum::extract::Path("b78-absent".to_string()),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 folder_delete 必须 401（门在 DB 访问前）"
    );

    let res = file_create(
        Extension(db.0),
        none,
        Json(FileCreateRequest {
            name: "b78".to_string(),
            person: "b78-person".to_string(),
            reference_type: "b78".to_string(),
            reference_id: None,
            extension: None,
            length: None,
            mime_type: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 file_create 必须 401（门在 DB 访问前）"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b78_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b78-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b78-unit");
}
