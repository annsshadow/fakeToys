use axum::Router;
use deadpool_postgres::Pool;

pub mod routes;

#[cfg(test)]
mod tests;

pub fn correlation_router(pool: Pool) -> Router {
    routes::correlation_router(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/correlation/health", axum::routing::get(|| async { "TODO: correlation - real implementation needed" }))
}