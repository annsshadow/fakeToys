use axum::Router;

use crate::{
    calendar_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    calendar_core_entity_router(pool)
}
