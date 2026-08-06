use axum::Router;
use deadpool_postgres::Pool;

use crate::{
    message_core_entity_router,
};

pub fn router(pool: Pool) -> Router {
    message_core_entity_router(pool)
}
