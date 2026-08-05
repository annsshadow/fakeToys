use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{group, person, role, unit};

/// 构建 control 模块路由
///
/// 路径对齐 Java Action 契约（PersonAction/GroupAction/RoleAction/UnitAction）：
/// - 创建: POST /jaxrs/{entity}
/// - 单条: GET/PUT/DELETE /jaxrs/{entity}/{flag}
/// - 游标分页: GET /jaxrs/{entity}/list/{flag}/next|prev/{count}
/// 增删改接口（POST/PUT/DELETE）需配合权限中间件使用。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn control_router(pool: Pool) -> Router {
    Router::new()
        // 人员管理
        .route("/jaxrs/person", post(person::create))
        .route("/jaxrs/person/{flag}", get(person::get))
        .route("/jaxrs/person/{flag}", put(person::update))
        .route("/jaxrs/person/{flag}", delete(person::delete))
        .route("/jaxrs/person/list/{flag}/next/{count}", get(person::list_next))
        .route("/jaxrs/person/list/{flag}/prev/{count}", get(person::list_prev))
        // 用户组管理
        .route("/jaxrs/group", post(group::create))
        .route("/jaxrs/group/{flag}", get(group::get))
        .route("/jaxrs/group/{flag}", put(group::update))
        .route("/jaxrs/group/{flag}", delete(group::delete))
        .route("/jaxrs/group/list/{flag}/next/{count}", get(group::list_next))
        .route("/jaxrs/group/list/{flag}/prev/{count}", get(group::list_prev))
        // 角色管理
        .route("/jaxrs/role", post(role::create))
        .route("/jaxrs/role/{flag}", get(role::get))
        .route("/jaxrs/role/{flag}", put(role::update))
        .route("/jaxrs/role/{flag}", delete(role::delete))
        .route("/jaxrs/role/list/{flag}/next/{count}", get(role::list_next))
        .route("/jaxrs/role/list/{flag}/prev/{count}", get(role::list_prev))
        // 单位管理
        .route("/jaxrs/unit", post(unit::create))
        .route("/jaxrs/unit/list", get(unit::list))
        .route("/jaxrs/unit/{flag}", get(unit::get))
        .route("/jaxrs/unit/{flag}", put(unit::update))
        .route("/jaxrs/unit/{flag}", delete(unit::delete))
        .route("/jaxrs/unit/list/{flag}/next/{count}", get(unit::list_next))
        .route("/jaxrs/unit/list/{flag}/prev/{count}", get(unit::list_prev))
        .layer(Extension(pool))
}