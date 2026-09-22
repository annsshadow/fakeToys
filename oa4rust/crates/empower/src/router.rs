// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use super::{
    create, delete as empower_delete, disable, enable, get as empower_get, list_current_person,
    list_current_person_enable, list_to, list_to_enable, manager_create, manager_delete,
    manager_list_paging, manager_update, update,
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
        .route("/api/person/empower", post(create))
        .route("/api/person/empower/{id}", get(empower_get))
        .route("/api/person/empower/{id}", put(update))
        .route("/api/person/empower/{id}", delete(empower_delete))
        // enable/disable 主注册为 POST（防 CSRF），追加 GET 变体对齐 o2server 契约
        .route("/api/person/empower/{id}/enable", post(enable).get(enable))
        .route(
            "/api/person/empower/{id}/disable",
            post(disable).get(disable),
        )
        // 管理员端点
        .route("/api/person/empower/manager", post(manager_create))
        .route("/api/person/empower/manager/{id}", put(manager_update))
        .route("/api/person/empower/manager/{id}", delete(manager_delete))
        .route(
            "/api/person/empower/manager/list/paging/{page}/size/{size}",
            post(manager_list_paging),
        )
        // 当前用户查询端点
        .route(
            "/api/person/empower/list/currentperson",
            get(list_current_person),
        )
        .route(
            "/api/person/empower/list/currentperson/enable",
            get(list_current_person_enable),
        )
        .route("/api/person/empower/list/to", get(list_to))
        .route("/api/person/empower/list/to/enable", get(list_to_enable))
        .with_state(pool)
        .with_state(session_manager)
}
