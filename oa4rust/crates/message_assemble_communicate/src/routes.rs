use axum::Router;
use deadpool_postgres::Pool;

use crate::message_assemble_communicate_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    message_assemble_communicate_router(Some(pool))
}

