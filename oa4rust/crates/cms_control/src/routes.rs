use axum::Router;
use deadpool_postgres::Pool;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    let _ = pool;
    Router::new()
}
