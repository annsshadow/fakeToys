use axum::{
    extract::Extension,
    Json,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod forum;
pub mod section;
pub mod subject;

pub fn bbs_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/forum/view/all", get(forum::view_all))
        .route("/jaxrs/bbs/forum/view/{id}", get(forum::view_one))
        .route("/jaxrs/bbs/section/viewforum/{forumId}", get(section::view_forum))
        .route("/jaxrs/bbs/section/view/all", get(section::view_all))
        .route("/jaxrs/bbs/subject/top/{sectionId}", get(subject::top))
        .route("/jaxrs/bbs/subject/list/{sectionId}", get(subject::list))
        .route("/jaxrs/bbs/subject/view/{id}", get(subject::view))
        .route("/jaxrs/bbs/subject/create", axum::routing::post(subject::create))
        .route("/jaxrs/bbs/subject/search", get(subject::search))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::bbs_router(pool)
}

#[cfg(test)]
mod tests;
