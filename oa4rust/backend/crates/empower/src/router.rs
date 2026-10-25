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
        // ── o2server 契约路径别名（2026-10-24 R1，OA-P0-1 收口）────────────
        // 契约（scripts/all_java_handlers.json 的 x_organization_assemble_personal，
        // 76 条）以 /api/organization/assemble/personal/ 为基路径；Rust 前端一直调
        // /api/person/* 与 /api/reset/*。此处把契约路径**原样委托同一 handler**，
        // 旧路径行为零变化（前提是 shared 中间件四张表——豁免/限流/权限/admin——
        // 已按别名形态补齐且 parity 为 0，见 tests 的契约别名守卫）。
        // 每条注释 <- 后是该别名委托的旧路由；handler 相同则行为逐位相同。
        .route("/api/organization/assemble/personal/empower", post(create)) // <- POST /api/person/empower [create]
        .route(
            "/api/organization/assemble/personal/empower/list/currentperson",
            get(list_current_person),
        ) // <- GET /api/person/empower/list/currentperson [list_current_person]
        .route(
            "/api/organization/assemble/personal/empower/list/currentperson/enable",
            get(list_current_person_enable),
        ) // <- GET /api/person/empower/list/currentperson/enable [list_current_person_enable]
        .route(
            "/api/organization/assemble/personal/empower/list/to",
            get(list_to),
        ) // <- GET /api/person/empower/list/to [list_to]
        .route(
            "/api/organization/assemble/personal/empower/list/to/enable",
            get(list_to_enable),
        ) // <- GET /api/person/empower/list/to/enable [list_to_enable]
        .route(
            "/api/organization/assemble/personal/empower/manager",
            post(manager_create),
        ) // <- POST /api/person/empower/manager [manager_create]
        .route(
            "/api/organization/assemble/personal/empower/manager/list/paging/{page}/size/{size}",
            post(manager_list_paging),
        ) // <- POST /api/person/empower/manager/list/paging/{page}/size/{size} [manager_list_paging]
        .route(
            "/api/organization/assemble/personal/empower/manager/{id}",
            delete(manager_delete),
        ) // <- DELETE /api/person/empower/manager/{id} [manager_delete]
        .route(
            "/api/organization/assemble/personal/empower/manager/{id}",
            put(manager_update),
        ) // <- PUT /api/person/empower/manager/{id} [manager_update]
        .route(
            "/api/organization/assemble/personal/empower/{id}",
            delete(empower_delete),
        ) // <- DELETE /api/person/empower/{id} [empower_delete]
        .route(
            "/api/organization/assemble/personal/empower/{id}",
            get(empower_get),
        ) // <- GET /api/person/empower/{id} [empower_get]
        .route(
            "/api/organization/assemble/personal/empower/{id}",
            put(update),
        ) // <- PUT /api/person/empower/{id} [update]
        .route(
            "/api/organization/assemble/personal/empower/{id}/disable",
            get(disable),
        ) // <- GET /api/person/empower/{id}/disable [disable]
        .route(
            "/api/organization/assemble/personal/empower/{id}/enable",
            get(enable),
        ) // <- GET /api/person/empower/{id}/enable [enable]
        .with_state(pool)
        .with_state(session_manager)
}
