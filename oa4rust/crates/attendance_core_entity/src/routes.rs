use axum::Router;

use crate::{
    attendance_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::attendance_core_entity_router(pool)
}

