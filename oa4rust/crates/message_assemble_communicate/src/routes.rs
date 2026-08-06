use axum::Router;
use deadpool_postgres::Pool;

use crate::message_assemble_communicate_router;

pub fn router(pool: Pool) -> Router {
    message_assemble_communicate_router(Some(pool))
}
