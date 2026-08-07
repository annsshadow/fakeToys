use axum::Router;

use crate::{
    jpush_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_core_entity_router(pool)
}

