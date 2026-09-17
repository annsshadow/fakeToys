use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{consume_list, custom_create, mark_read, unread_count, update_single};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/api/message/consume/list/{consume}/count/{count}",
            get(consume_list),
        )
        .route("/api/message/consume/{id}/type/{type}", get(update_single))
        .route("/api/message/custom/create", post(custom_create))
        .route("/api/message/mark_read/{id}", post(mark_read))
        .route("/api/message/unread/count/{consume}", get(unread_count))
        .layer(Extension(pool))
}
