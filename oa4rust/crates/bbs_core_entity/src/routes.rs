use axum::Router;

use crate::{
    bbs_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::bbs_core_entity_router(pool)
}

