use axum::Router;
use deadpool_postgres::Pool;

use crate::processplatform_service_processing_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_service_processing_router().layer(axum::extract::Extension(pool))
}

