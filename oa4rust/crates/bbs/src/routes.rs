use axum::Router;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::bbs_router(pool)
}

