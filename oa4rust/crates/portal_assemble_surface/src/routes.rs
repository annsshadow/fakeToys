use axum::Router;

use crate::portal_assemble_surface_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_assemble_surface_router().layer(axum::extract::Extension(pool))
}

