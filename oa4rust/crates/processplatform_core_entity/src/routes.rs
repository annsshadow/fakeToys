use axum::Router;

use crate::{processplatform_core_entity_router};

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    processplatform_core_entity_router(pool)
}
