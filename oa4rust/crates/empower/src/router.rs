use axum::{routing::{delete, get, post, put}, Router};
use deadpool_postgres::Pool;

use super::{
    create, delete as empower_delete, enable, get as empower_get, list_current_person,
    list_current_person_enable, list_to, list_to_enable, manager_create, manager_delete,
    manager_list_paging, manager_update, update, disable,
};
use auth::SessionManager;

/// 构建授权管理模块路由
///
/// 注册 14 个端点：
///   普通用户 CRUD + enable/disable（含 IDOR 防护）
///   管理员 CRUD + 分页列表
///   当前用户授权列表查询（4 个变体）
pub fn router(pool: Pool, session_manager: SessionManager) -> Router {
    Router::new()
        // 普通用户端点（需 owner 验证）
        .route("/jaxrs/person/empower", post(create))
        .route("/jaxrs/person/empower/{id}", get(empower_get))
        .route("/jaxrs/person/empower/{id}", put(update))
        .route("/jaxrs/person/empower/{id}", delete(empower_delete))
        .route("/jaxrs/person/empower/{id}/enable", post(enable))
        .route("/jaxrs/person/empower/{id}/disable", post(disable))
        // 管理员端点
        .route("/jaxrs/person/empower/manager", post(manager_create))
        .route("/jaxrs/person/empower/manager/{id}", put(manager_update))
        .route("/jaxrs/person/empower/manager/{id}", delete(manager_delete))
        .route(
            "/jaxrs/person/empower/manager/list/paging/{page}/size/{size}",
            post(manager_list_paging),
        )
        // 当前用户查询端点
        .route(
            "/jaxrs/person/empower/list/currentperson",
            get(list_current_person),
        )
        .route(
            "/jaxrs/person/empower/list/currentperson/enable",
            get(list_current_person_enable),
        )
        .route("/jaxrs/person/empower/list/to", get(list_to))
        .route(
            "/jaxrs/person/empower/list/to/enable",
            get(list_to_enable),
        )
        .with_state(pool)
        .with_state(session_manager)
}
