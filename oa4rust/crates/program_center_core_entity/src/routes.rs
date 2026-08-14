use axum::Router;

use crate::{program_center_core_entity_router};

pub async fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_core_entity_router(pool).await
}

