use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .layer(Extension(pool))
}

