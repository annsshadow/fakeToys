use axum::Router;

use crate::query_assemble_surface_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_surface_router().layer(axum::extract::Extension(pool))
}

