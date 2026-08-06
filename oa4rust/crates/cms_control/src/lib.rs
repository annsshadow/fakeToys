use axum::Router;
use deadpool_postgres::Pool;

pub mod routes;

pub fn cms_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    cms_control_router(pool)
}
