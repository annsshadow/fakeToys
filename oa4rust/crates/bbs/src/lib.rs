use axum::{extract::Extension, routing::get, routing::post, Router};
use deadpool_postgres::Pool;
use shared::middleware::SecurityState;

pub mod forum;
pub mod section;
pub mod subject;
pub mod routes;

pub fn bbs_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/forum/view/all", get(forum::view_all))
        .route("/jaxrs/bbs/forum/view/{id}", get(forum::view_one))
        .route("/jaxrs/bbs/section/viewforum/{forumId}", get(section::view_forum))
        .route("/jaxrs/bbs/section/view/all", get(section::view_all))
        .route("/jaxrs/bbs/subject/top/{sectionId}", get(subject::top))
        .route("/jaxrs/bbs/subject/list/{sectionId}", get(subject::list))
        .route("/jaxrs/bbs/subject/view/{id}", get(subject::view))
        .route("/jaxrs/bbs/subject/create", post(subject::create))
        .route("/jaxrs/bbs/subject/search", get(subject::search))
        .layer(Extension(pool))
}

pub fn router(pool: Pool) -> Router {
    use axum::middleware;
    bbs_router(pool)
        .layer(middleware::from_fn(shared::middleware::security_headers_middleware))
        .layer(middleware::from_fn(shared::middleware::trace_middleware))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

