// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::Router;
use deadpool_postgres::Pool;

pub mod entities;
pub mod handlers;
pub mod routes;

pub use routes::router;

const MAX_NAME_LEN: usize = 200;
const MAX_TEXT_LEN: usize = 500;
const MAX_LONG_TEXT_LEN: usize = 2000;

// 本 crate 的 5 个子 router 全部返回 `Router::new()`——`/api/program_center/*`
// 路由由 program_center crate 持有（裸 SQL 实现），在此重复注册会
// panic axum（"Overlapping method route"，见 handlers/application.rs 注释）。
//
// 2026-10-24：删掉这里曾经的空转 SeaORM 池创建（`create_sea_orm_pool().await.ok()`）——
// 每次启动多建最多 20 条连接的池却零路由消费，白占 PG 连接额度，
// `.ok()` 还把初始化失败吞掉。子 router 的 `_db` 参数保持签名（下划线
// 标记弃用），传 None 即无需改动 5 个 handler 文件。
pub fn program_center_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .merge(handlers::application::_router(pool.clone(), None))
        .merge(handlers::script::_router(pool.clone(), None))
        .merge(handlers::invoke::_router(pool.clone(), None))
        .merge(handlers::agent::_router(pool.clone(), None))
        .merge(handlers::structure::_router(pool, None))
}

#[cfg(test)]
pub fn program_center_mock_router(_db: sea_orm::DatabaseConnection) -> Router {
    use axum::routing::get;
    Router::new()
        .route(
            "/api/program_center/application/list",
            get(|| async { "ok" }),
        )
        .route("/api/program_center/script/list", get(|| async { "ok" }))
        .route("/api/program_center/invoke/list", get(|| async { "ok" }))
        .route("/api/program_center/agent/list", get(|| async { "ok" }))
        .route("/api/program_center/structure/list", get(|| async { "ok" }))
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[cfg(test)]
pub fn test_router(_pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_mock_router(sea_orm::DatabaseConnection::default())
}
