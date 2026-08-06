use axum::Router;
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> Router {
    Router::new().layer(axum::Extension(pool))
}
