use axum::Router;

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    crate::hotpic_router(pool)
}
