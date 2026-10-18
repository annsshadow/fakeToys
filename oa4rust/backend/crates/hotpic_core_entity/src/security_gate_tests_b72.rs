//! 安全审计回归（round4 第七十二批·hotpic_core_entity）：hotpic 2 把
//! sea_orm CRUD 真写收会话门。
//!
//! `create`（POST `/api/hotpic/core/entity/create`，sea_orm insert `hotpic`）
//! 与 `delete_by_id`（DELETE `/api/hotpic/core/entity/delete/{id}`，find 后
//! update `deleted_at` 软删）此前无会话门禁：任何匿名调用方即可创建/删除
//! 热图。本批加"会话存在档"门禁（`require_session`，无会话 401，门在首个
//! DB 访问之前）。
//!
//! 注意：这些是 sea_orm ORM 写，审计尺子"含写"指标不可见（ORM-blindness，
//! §五 尺子缺陷 28）——R7 降 2 但"含写"不变属预期；本批以离线 401 臂交叉验证。
//!
//! 证据臂：
//! 1. 401 臂：真实 sea_orm 连接（`shared::testing::test_sea_orm_pool()`，门在
//!    DB 访问前返回，绝不落查询）+ `None` 会话 → 全部 `Err(AppError::Unauthorized)`。
//! 2. 过门臂（仅 delete_by_id，不落任何写）：有效会话 + 不存在的 id →
//!    过门后 `find_by_id` 查无 → `ActionResult::error("hotpic not found")`（Ok 信封），
//!    证门只挡匿名、挡不住已认证，且该路径不产生写。
//!    （`create` 不做过门臂——有效会话会真落库 INSERT。）
//! 3. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

/// 不存在的 id：过门后 `find_by_id` 必 None → error 信封（不产生任何写）。
const ABSENT_ID: &str = "b72-absent-id-does-not-exist";

#[tokio::test]
async fn b72_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let none: Option<Extension<shared::session::Session>> = None;

    let res = create(Extension(db.clone()), none.clone(), Json(serde_json::json!({}))).await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 create 必须 401（门在 DB 访问前）"
    );

    let res = delete_by_id(
        Extension(db),
        none,
        axum::extract::Path(ABSENT_ID.to_string()),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 delete_by_id 必须 401（门在 DB 访问前）"
    );
}

#[tokio::test]
async fn b72_delete_passes_gate_reaches_db_without_write() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let session = Some(Extension(shared::testing::test_session("b72-valid")));

    // 有效会话 + 不存在的 id：过门后 find_by_id 查无 → error 信封（Ok），不落任何写。
    let res = delete_by_id(
        Extension(db),
        session,
        axum::extract::Path(ABSENT_ID.to_string()),
    )
    .await;
    assert!(
        res.is_ok(),
        "有效会话应过门并触库（不存在的 id → error 信封），证门在库前"
    );
}
/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b72_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b72-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b72-unit");
}
