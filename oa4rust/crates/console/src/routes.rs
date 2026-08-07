use crate::console_router;
use axum::Router;
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> axum::Router {
    console_router().layer(axum::extract::Extension(pool))
}
