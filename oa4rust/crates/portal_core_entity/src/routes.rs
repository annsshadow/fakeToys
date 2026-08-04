use axum::Router;

use crate::{portal_core_entity_router};

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    portal_core_entity_router(pool)
}
