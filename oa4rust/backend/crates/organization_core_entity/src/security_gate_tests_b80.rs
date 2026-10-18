//! 安全审计回归（round4 第八十批 · ORM 盲区系统复查)：organization_core_entity
//! 12 把 sea_orm CRUD 真写收会话门。
//!
//! definition / identity / custom / bind 四族的 create/update/delete 共 12 把均经
//! sea_orm `ActiveModel` 落笔，此前无会话门禁。这些是 ORM 写，审计尺子"含写"
//! 指标不可见（ORM-blindness，§五 尺子缺陷 28）——R7 降 12 但"含写"不变属预期；
//! 本批以离线 401 臂交叉验证。同 crate 的 group/person 6 写路由挂中间件
//! ADMIN_WRITE_PREFIXES（R1-中间件 admin），不在本批范围。
//!
//! 注：本 crate 的 router 从不提供 `Extension<DatabaseConnection>`（§五 遗留 30
//! 系统性接线缺陷）⇒ HTTP 面 extraction 先失败（500），本批只做静态收门 +
//! 直调臂验证（与 b78/b79 同型）。
//!
//! 证据臂（直调 handler，绕过 extraction）：
//! 1. 真 sea_orm 连接 + `None` 会话 → 12 把全部 `Err(AppError::Unauthorized)`，
//!    门在 DB 访问前返回，绝不落查询。
//! 2. 纯单元：`require_session` 门逻辑（无 DB 副作用）。

use super::*;

#[tokio::test]
async fn b80_writes_reject_without_session() {
    let db = shared::testing::test_sea_orm_pool()
        .await
        .expect("test sea_orm db");
    let db = Extension(db);
    let none: Option<Extension<shared::session::Session>> = None;
    let path = || axum::extract::Path("b80-absent".to_string());

    // definition
    assert!(matches!(
        definition_create(
            Extension(db.0.clone()),
            none.clone(),
            Json(DefinitionCreateRequest {
                name: "b80".to_string(),
                category: "b80".to_string(),
                type_: "b80".to_string(),
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "definition_create 无会话必须 401");
    assert!(matches!(
        definition_update(
            Extension(db.0.clone()),
            none.clone(),
            path(),
            Json(DefinitionUpdateRequest {
                name: None,
                category: None,
                type_: None,
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "definition_update 无会话必须 401");
    assert!(matches!(
        definition_delete(Extension(db.0.clone()), none.clone(), path()).await,
        Err(AppError::Unauthorized)
    ), "definition_delete 无会话必须 401");

    // identity
    assert!(matches!(
        identity_create(
            Extension(db.0.clone()),
            none.clone(),
            Json(IdentityCreateRequest {
                person_id: "b80".to_string(),
                name: "b80".to_string(),
                type_: "b80".to_string(),
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "identity_create 无会话必须 401");
    assert!(matches!(
        identity_update(
            Extension(db.0.clone()),
            none.clone(),
            path(),
            Json(IdentityUpdateRequest {
                person_id: None,
                name: None,
                type_: None,
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "identity_update 无会话必须 401");
    assert!(matches!(
        identity_delete(Extension(db.0.clone()), none.clone(), path()).await,
        Err(AppError::Unauthorized)
    ), "identity_delete 无会话必须 401");

    // custom
    assert!(matches!(
        custom_create(
            Extension(db.0.clone()),
            none.clone(),
            Json(CustomCreateRequest {
                identity_id: "b80".to_string(),
                field_name: "b80".to_string(),
                field_value: "b80".to_string(),
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "custom_create 无会话必须 401");
    assert!(matches!(
        custom_update(
            Extension(db.0.clone()),
            none.clone(),
            path(),
            Json(CustomUpdateRequest {
                field_name: None,
                field_value: None,
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "custom_update 无会话必须 401");
    assert!(matches!(
        custom_delete(Extension(db.0.clone()), none.clone(), path()).await,
        Err(AppError::Unauthorized)
    ), "custom_delete 无会话必须 401");

    // bind
    assert!(matches!(
        bind_create(
            Extension(db.0.clone()),
            none.clone(),
            Json(BindCreateRequest {
                identity_id: "b80".to_string(),
                group_id: "b80".to_string(),
                role: None,
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "bind_create 无会话必须 401");
    assert!(matches!(
        bind_update(
            Extension(db.0.clone()),
            none.clone(),
            path(),
            Json(BindUpdateRequest {
                identity_id: None,
                group_id: None,
                role: None,
            }),
        )
        .await,
        Err(AppError::Unauthorized)
    ), "bind_update 无会话必须 401");
    assert!(matches!(
        bind_delete(Extension(db.0), none, path()).await,
        Err(AppError::Unauthorized)
    ), "bind_delete 无会话必须 401");
}

/// 纯单元：`require_session` 门逻辑本身（无 DB 副作用）——无会话拒绝、有会话放行。
#[test]
fn b80_require_session_gate_logic() {
    let none: Option<Extension<shared::session::Session>> = None;
    assert!(
        matches!(require_session(none), Err(AppError::Unauthorized)),
        "require_session：无会话必须 Unauthorized"
    );

    let some = Some(Extension(shared::testing::test_session("b80-unit")));
    let sess = require_session(some).expect("require_session：有会话必须放行");
    assert_eq!(sess.person_unique, "b80-unit");
}
