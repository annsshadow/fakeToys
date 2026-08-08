use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    forum, section, subject,
};

pub fn router(pool: Pool) -> Router {
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
