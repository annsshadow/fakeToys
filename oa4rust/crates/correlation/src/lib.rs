use axum::Router;
use deadpool_postgres::Pool;

pub mod routes;

#[cfg(test)]
mod tests;

pub fn correlation_router(pool: Pool) -> Router {
    routes::correlation_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    correlation_router(pool)
}