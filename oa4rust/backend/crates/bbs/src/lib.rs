// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, routing::post, Router};
use deadpool_postgres::Pool;
use shared::error::AppError;

pub mod forum;
pub mod routes;
pub mod section;
pub mod subject;

/// 会话存在档门：无会话（Option<Extension<Session>> = None）→ 401。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

/// 写侧归属防护：非管理员不得以他人身份发帖/回复（冒名）。管理员代写仍允许。
async fn reject_impersonation(
    pool: &Pool,
    sess: &shared::session::Session,
    payload: &serde_json::Value,
    fields: &[&str],
) -> Result<(), AppError> {
    let requested = fields.iter().find_map(|f| {
        payload
            .get(*f)
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
    });
    if let Some(owner) = requested {
        if owner != sess.person_unique
            && !shared::middleware::is_admin(pool, &sess.person_unique).await
        {
            return Err(AppError::Forbidden);
        }
    }
    Ok(())
}

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
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;
