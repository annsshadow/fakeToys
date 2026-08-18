use axum::Router;

use crate::{
    ai_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::ai_core_entity_router(pool)
}

