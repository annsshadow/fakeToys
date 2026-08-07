use axum::Router;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::file_router(pool)
}

