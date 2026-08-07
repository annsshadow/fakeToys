use axum::Router;

use crate::query_express_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_express_router().layer(axum::extract::Extension(pool))
}

