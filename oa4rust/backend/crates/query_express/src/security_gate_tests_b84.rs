//! 安全审计回归（round4 第八十四批 · ORM 盲区系统复查 · query_express）：
//! 1 把 sea_orm 真写收会话门。
//!
//! `create_query`（POST `/api/query/create`，INSERT query_view）经 sea_orm 落笔，
//! 此前无会话门禁。这是 ORM 写，审计尺子"含写"指标不可见（ORM-blindness，
//! §五 尺子缺陷 28）——R7 降 1 但"含写"不变属预期。
//!
//! 证据臂（直调 handler，绕过 extraction）：
//! 1. 真 sea_orm 连接 + `None` 会话 → `Err(AppError::Unauthorized)`，门在 DB 访问前
//!    返回，绝不落 INSERT。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b84_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;

    assert!(matches!(
        create_query(
            Extension(db.0),
            none,
            Json(serde_json::json!({"name": "b84", "queryType": "x"}))
        )
        .await,
        Err(AppError::Unauthorized)
    ), "create_query 无会话必须 401");
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b84_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b84-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b84-unit");
}
