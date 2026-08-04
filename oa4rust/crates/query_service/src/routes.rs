use axum::Router;
use deadpool_postgres::Pool;

pub fn build_router(pool: Pool) -> Router {
    Router::new().layer(axum::Extension(pool))
}
