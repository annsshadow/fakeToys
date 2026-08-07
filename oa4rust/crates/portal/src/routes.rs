use axum::Router;

use crate::portal_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_router().layer(axum::extract::Extension(pool))
}

