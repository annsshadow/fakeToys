use axum::Router;
use deadpool_postgres::Pool;

use crate::{
    message_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::message_core_entity_router(pool)
}

