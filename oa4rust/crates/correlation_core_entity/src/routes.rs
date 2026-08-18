use axum::Router;

use crate::{
    correlation_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::correlation_core_entity_router(pool)
}

