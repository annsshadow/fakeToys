use axum::Router;
use deadpool_postgres::Pool;

use crate::processplatform_assemble_designer_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_assemble_designer_router().layer(axum::extract::Extension(pool))
}

