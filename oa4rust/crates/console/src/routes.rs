use axum::Router;

use crate::console_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    console_router().layer(axum::extract::Extension(pool))
}

