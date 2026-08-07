use axum::Router;

use crate::{
    consume_list, custom_create, message_router, update_single
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    message_router().layer(axum::extract::Extension(pool))
}

