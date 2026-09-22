// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, routing::post, Router};
use deadpool_postgres::Pool;

pub mod forum;
pub mod routes;
pub mod section;
pub mod subject;

pub fn bbs_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/bbs/forum/view/all", get(forum::view_all))
        .route("/api/bbs/forum/view/{id}", get(forum::view_one))
        .route(
            "/api/bbs/section/viewforum/{forumId}",
            get(section::view_forum),
        )
        .route("/api/bbs/section/view/all", get(section::view_all))
        .route("/api/bbs/subject/top/{sectionId}", get(subject::top))
        .route("/api/bbs/subject/list/{sectionId}", get(subject::list))
        .route("/api/bbs/subject/view/{id}", get(subject::view))
        .route("/api/bbs/subject/create", post(subject::create))
        .route("/api/bbs/subject/search", get(subject::search))
        .layer(Extension(pool))
}

pub fn router(pool: Pool) -> Router {
    use axum::middleware;
    bbs_router(pool)
        .layer(middleware::from_fn(
            shared::middleware::security_headers_middleware,
        ))
        .layer(middleware::from_fn(shared::middleware::trace_middleware))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
