use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{consume_list, custom_create, mark_read, update_single, unread_count};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/message/consume/list/{consume}/count/{count}", get(consume_list))
        .route("/jaxrs/message/consume/{id}/type/{type}", get(update_single))
        .route("/jaxrs/message/custom/create", post(custom_create))
        .route("/jaxrs/message/mark_read/{id}", post(mark_read))
        .route("/jaxrs/message/unread/count/{consume}", get(unread_count))
        .layer(Extension(pool))
}
