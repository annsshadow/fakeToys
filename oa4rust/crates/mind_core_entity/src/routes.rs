use axum::Router;

use crate::{
    mind_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::mind_core_entity_router(pool)
}

