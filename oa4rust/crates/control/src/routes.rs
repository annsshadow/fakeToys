use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;

use crate::{group, person, role, unit};

/// 构建 control 模块路由
///
/// 注册人员管理、单位管理、角色管理、用户组管理所有接口。
/// 增删改接口（POST/PUT/DELETE）需配合权限中间件使用，
/// 当前返回 403 表示未授权（实际权限校验由上层中间件完成）。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn control_router(pool: Pool) -> Router {
    Router::new()
        // 健康检查
        // 人员管理
        .route("/jaxrs/person/list", get(person::list))
        .route("/jaxrs/person/create", post(person::create))
        .route("/jaxrs/person/{id}", get(person::get))
        .route("/jaxrs/person/{id}/update", put(person::update))
        .route("/jaxrs/person/{id}/delete", delete(person::delete))
        // 单位管理
        .route("/jaxrs/unit/list", get(unit::list))
        .route("/jaxrs/unit/create", post(unit::create))
        .route("/jaxrs/unit/{id}", get(unit::get))
        .route("/jaxrs/unit/{id}/update", put(unit::update))
        .route("/jaxrs/unit/{id}/delete", delete(unit::delete))
        // 角色管理
        .route("/jaxrs/role/list", get(role::list))
        .route("/jaxrs/role/create", post(role::create))
        .route("/jaxrs/role/{id}", get(role::get))
        .route("/jaxrs/role/{id}/update", put(role::update))
        .route("/jaxrs/role/{id}/delete", delete(role::delete))
        // 用户组管理
        .route("/jaxrs/group/list", get(group::list))
        .route("/jaxrs/group/create", post(group::create))
        .route("/jaxrs/group/{id}", get(group::get))
        .route("/jaxrs/group/{id}/update", put(group::update))
        .route("/jaxrs/group/{id}/delete", delete(group::delete))
        .layer(Extension(pool))
}

/// 健康检查处理器（返回 ActionResult 格式）
///
/// 与 `/health` 路由对应，返回统一的成功响应格式。
pub(super) async fn health_check() -> axum::Json<shared::response::ActionResult<Value>> {
    axum::Json(shared::response::ActionResult::success(serde_json::json!({"status": "ok", "module": "control"})))
}
