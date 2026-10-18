//! 安全审计回归（round4 第七十批·calendar_core_entity）：calendar/event 6 把
//! sea_orm CRUD 真写收会话门。
//!
//! `calendar_create` / `calendar_update` / `calendar_remove` / `event_create` /
//! `event_update` / `event_remove` 均经 sea_orm `ActiveModel::insert/update`
//! （或 `Entity::find_by_id().one()` 后 update）写 `cal_calendar` / `cal_event`，
//! 此前无会话门禁：任何匿名调用方即可创建/改写/关闭日历与事件。本批加
//! "会话存在档"门禁（`require_session`，无会话 401，门在首个 DB 访问之前）。
//!
//! 注意：这些是 sea_orm ORM 写，审计尺子"含写"指标不可见（ORM-blindness，
//! §五 尺子缺陷 28）——R7 降 6 但"含写"不变属预期；本批以离线 401 臂交叉验证。
//!
//! 证据臂：
//! 1. 401 臂：真实 sea_orm 连接（`shared::testing::test_sea_orm_pool()`，门在
//!    DB 访问前返回，绝不落查询）+ `None` 会话 → 全部 `Err(AppError::Unauthorized)`。
//! 2. 过门臂（仅 update/remove 四把，不落任何写）：有效会话 + 不存在的 id →
//!    过门后 `find_by_id` 查不到 → `Err(AppError::NotFound)`，证门在库前且只挡匿名。
//!    （create 两把不作过门臂——有效会话会真落库 INSERT。）
//! 3. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

/// 不存在的 id：过门后 `find_by_id` 必 None → NotFound（不产生任何写）。
const ABSENT_ID: &str = "b70-absent-id-does-not-exist";

#[tokio::test]
async fn b70_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;

    // calendar_create：必填字段齐全，断言语义落在"门先拦"（无会话 401）。
    let res = calendar_create(
        Extension(db.0.clone()),
        none.clone(),
        AxumJson(CreateCalendarRequest {
            name: Some("b70".to_string()),
            calendar_type: Some("PERSONAL".to_string()),
            target: None,
            color: None,
            description: None,
            source: None,
            createor: None,
            is_public: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 calendar_create 必须 401（门在 DB 访问前）"
    );

    let res = calendar_update(
        Extension(db.0.clone()),
        none.clone(),
        AxumJson(UpdateCalendarRequest {
            id: Some(ABSENT_ID.to_string()),
            name: None,
            calendar_type: None,
            target: None,
            color: None,
            description: None,
            is_public: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 calendar_update 必须 401（门在 DB 访问前）"
    );

    let res = calendar_remove(
        Extension(db.0.clone()),
        none.clone(),
        AxumJson(DeleteCalendarRequest {
            id: Some(ABSENT_ID.to_string()),
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 calendar_remove 必须 401（门在 DB 访问前）"
    );

    let res = event_create(
        Extension(db.0.clone()),
        none.clone(),
        AxumJson(CreateEventRequest {
            calendar_id: Some("b70-cal".to_string()),
            title: Some("b70".to_string()),
            content: None,
            location: None,
            start_time: Some("2026-10-18T00:00:00".to_string()),
            end_time: Some("2026-10-18T01:00:00".to_string()),
            all_day: None,
            visibility: None,
            createor: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 event_create 必须 401（门在 DB 访问前）"
    );

    let res = event_update(
        Extension(db.0.clone()),
        none.clone(),
        AxumJson(UpdateEventRequest {
            id: Some(ABSENT_ID.to_string()),
            title: None,
            content: None,
            location: None,
            start_time: None,
            end_time: None,
            all_day: None,
            visibility: None,
            status: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 event_update 必须 401（门在 DB 访问前）"
    );

    let res = event_remove(
        Extension(db.0),
        none,
        AxumJson(DeleteEventRequest {
            id: Some(ABSENT_ID.to_string()),
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::Unauthorized)),
        "无会话调用 event_remove 必须 401（门在 DB 访问前）"
    );
}

#[tokio::test]
async fn b70_update_remove_pass_gate_reach_db_without_write() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let session = Some(Extension(shared::testing::test_session("b70-valid")));

    // 有效会话 + 不存在的 id：过门后 find_by_id 查无 → NotFound。
    // 证明门只挡匿名、挡不住已认证；且该路径不产生任何写（无行可更新）。
    let res = calendar_update(
        Extension(db.0.clone()),
        session.clone(),
        AxumJson(UpdateCalendarRequest {
            id: Some(ABSENT_ID.to_string()),
            name: None,
            calendar_type: None,
            target: None,
            color: None,
            description: None,
            is_public: None,
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::NotFound)),
        "有效会话应过门并触库（不存在的 id → NotFound），证门在库前"
    );

    let res = event_remove(
        Extension(db.0),
        session,
        AxumJson(DeleteEventRequest {
            id: Some(ABSENT_ID.to_string()),
        }),
    )
    .await;
    assert!(
        matches!(res, Err(AppError::NotFound)),
        "有效会话应过门并触库（不存在的 id → NotFound），证门在库前"
    );
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b70_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b70-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b70-unit");
}
