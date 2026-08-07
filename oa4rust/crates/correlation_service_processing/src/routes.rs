use axum::Router;

use crate::correlation_service_processing_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    correlation_service_processing_router().layer(axum::extract::Extension(pool))
}

