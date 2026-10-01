// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{group, person, role, unit};

/// 构建 control 模块路由
///
/// 路径对齐 o2server Action 契约（PersonAction/GroupAction/RoleAction/UnitAction）：
/// - 创建: POST /api/{entity}
/// - 单条: GET/PUT/DELETE /api/{entity}/{flag}
/// - 游标分页: GET /api/{entity}/list/{flag}/next|prev/{count}
///
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
        .route("/api/person", post(person::create))
        .route("/api/person/{flag}", get(person::get))
        .route("/api/person/{flag}", put(person::update))
        .route("/api/person/{flag}", delete(person::delete))
        .route(
            "/api/person/list/{flag}/next/{count}",
            get(person::list_next),
        )
        .route(
            "/api/person/list/{flag}/prev/{count}",
            get(person::list_prev),
        )
        // 用户组管理
        .route("/api/group", post(group::create))
        .route("/api/group/{flag}", get(group::get))
        .route("/api/group/{flag}", put(group::update))
        .route("/api/group/{flag}", delete(group::delete))
        .route("/api/group/list/{flag}/next/{count}", get(group::list_next))
        .route("/api/group/list/{flag}/prev/{count}", get(group::list_prev))
        // 角色管理
        .route("/api/role", post(role::create))
        .route("/api/role/{flag}", get(role::get))
        .route("/api/role/{flag}", put(role::update))
        .route("/api/role/{flag}", delete(role::delete))
        .route("/api/role/list/{flag}/next/{count}", get(role::list_next))
        .route("/api/role/list/{flag}/prev/{count}", get(role::list_prev))
        // 单位管理
        .route("/api/unit", post(unit::create))
        .route("/api/unit/list", get(unit::list))
        .route("/api/unit/{flag}", get(unit::get))
        .route("/api/unit/{flag}", put(unit::update))
        .route("/api/unit/{flag}", delete(unit::delete))
        .route("/api/unit/list/{flag}/next/{count}", get(unit::list_next))
        .route("/api/unit/list/{flag}/prev/{count}", get(unit::list_prev))
        .layer(Extension(pool))
}
