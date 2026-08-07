use axum::Router;

use crate::{
    meeting_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::meeting_core_entity_router(pool)
}

