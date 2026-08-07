use axum::Router;

use crate::express_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    express_router().layer(axum::extract::Extension(pool))
}

