use axum::Router;

use crate::{
    file_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    file_core_entity_router(pool)
}
