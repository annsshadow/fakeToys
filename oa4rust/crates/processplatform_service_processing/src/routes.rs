use axum::Router;
use deadpool_postgres::Pool;

use crate::processplatform_service_processing_router;

pub fn router(pool: Pool) -> Router {
    processplatform_service_processing_router().layer(axum::Extension(pool))
}
